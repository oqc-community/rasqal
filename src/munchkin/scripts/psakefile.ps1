# Copyright (c) Microsoft Corporation.
# Licensed under the MIT License.

Include utils.ps1

Properties {
    $Root = Resolve-Path (Split-Path -Parent $PSScriptRoot)
    $BuildLlvm = Join-Path $Root build-llvm
    $Pykin = Join-Path $Root pykin
    $Examples = Join-Path $Root examples
    $Target = Join-Path $Root target
    $Wheels = Join-Path $Target wheels
    $CargoConfigToml = Join-Path $Root .cargo config.toml
    $VscodeSettingsJson = Join-Path $Root .vscode settings.json
    $DocsRoot = Join-Path $Root docs
    $DocsBuild = Join-Path $DocsRoot _build
    $RustVersion = "1.64.0"
    $ManylinuxTag = "manylinux2014_x86_64_maturin"
    $ManylinuxRoot = "/io"
    $Python = Resolve-Python
}

task default -depends build
task build -depends build-llvm, munchqin
task checks -depends cargo-fmt, cargo-clippy
task manylinux -depends build-manylinux-container-image, run-manylinux-container-image

task run-manylinux-container-image -preaction { Write-CacheStats } -postaction { Write-CacheStats } {
    # For any of the volumes mapped, if the dir doesn't exist,
    # docker will create it and it will be owned by root and
    # the caching/install breaks with permission errors.
    # New-Item is idempotent so we don't need to check for existence
    $cacheMount, $cacheEnv = Get-CCacheParams
    Write-BuildLog "Running container image: $ManylinuxTag"
    $ioVolume = "${Root}:$ManylinuxRoot"
    $userName = Get-LinuxContainerUserName

    Invoke-LoggedCommand {
        docker run --rm `
            --user $userName `
            --volume $ioVolume @cacheMount @cacheEnv `
            --env MK_CACHE_DIR=/tmp/llvm `
            --workdir $ManylinuxRoot `
            $ManylinuxTag `
            conda run --no-capture-output pwsh build.ps1 -t default
    }
}

task cargo-fmt {
    Invoke-LoggedCommand -workingDirectory $Root -errorMessage "Please run 'cargo fmt --all' before pushing" {
        cargo fmt --all -- --check
    }
}

task cargo-clippy -depends init {
    Invoke-LoggedCommand -workingDirectory $Root -errorMessage "Please fix the above clippy errors" {
        cargo clippy --workspace --all-targets @(Get-CargoArgs) -- -D warnings
    }
}

task build-llvm -depends init {
    Invoke-LoggedCommand -workingDirectory $BuildLlvm { cargo test --release @(Get-CargoArgs) }
    Invoke-LoggedCommand -workingDirectory $BuildLlvm { cargo build --release @(Get-CargoArgs) }
}

task munchqin -depends init {
    $env:MATURIN_PEP517_ARGS = (Get-CargoArgs) -Join " "
    Get-Wheels munchqin | Remove-Item -Verbose
    Invoke-LoggedCommand { pip --verbose wheel --no-deps --wheel-dir $Wheels $Pykin }

    if (Test-CommandExists auditwheel) {
        $unauditedWheels = Get-Wheels munchqin
        Invoke-LoggedCommand { auditwheel repair --wheel-dir $Wheels $unauditedWheels }
        $unauditedWheels | Remove-Item
    }

    # Force reinstall the package if it exists, but not its depenencies.
    $packages = Get-Wheels munchqin
    Invoke-LoggedCommand -workingDirectory $Root {
        pip install $packages
        pip install --force-reinstall --no-deps $packages
    }

    Invoke-LoggedCommand -workingDirectory $Root { pytest . }
}

task wheelhouse -precondition { -not (Test-Path (Join-Path $Wheels *.whl)) } {
    Invoke-Task build
}

task docs -depends check-environment, wheelhouse {
    Invoke-LoggedCommand {
        pip install --requirement (Join-Path $DocsRoot requirements.txt) (Join-Path $Wheels *.whl)
    }
    Invoke-LoggedCommand { sphinx-build -M html $DocsRoot $DocsBuild -W --keep-going }
}

task check-environment {
    $pyenv = Join-Path $Root ".env"
    if ((Test-Path -Path $pyenv) -eq $false) {
        Write-BuildLog "No virtual environment found."
        Write-BuildLog "Setting up virtual environment in $pyenv"
        & $Python -m venv $pyenv
    }
    else {
        Write-BuildLog "Virtual environment found."
    }

    if ($IsWindows) {
        Write-BuildLog "In Windows"
        . (Join-Path $pyenv Scripts Activate.ps1)
    }
    else {
        Write-BuildLog "Not in Windows"
        . (Join-Path $pyenv bin Activate.ps1)
    }

    $env_message = @(
        "Building LLVM requires a virtualenv or conda environment to build.",
        "Neither the VIRTUAL_ENV nor CONDA_PREFIX environment variables are set.",
        "See https://virtualenv.pypa.io/en/latest/index.html on how to use virtualenv"
    )
    Assert ((Test-InVirtualEnvironment) -eq $true) ($env_message -Join ' ')
}

task init -depends check-environment {
    # build-llvm has this logic built in when compiled on its own
    # but we must have LLVM installed prior to the wheels being built.

    # if an external LLVM is specified, make sure it exist and
    # skip further bootstapping
    if (Test-Path env:\MK_LLVM_EXTERNAL_DIR) {
        Use-ExternalLlvmInstallation
    }
    else {
        $packagePath = Resolve-InstallationDirectory
        if (Test-LlvmConfig $packagePath) {
            Write-BuildLog "LLVM target is already installed."
            # LLVM is already downloaded
            Use-LlvmInstallation $packagePath
        }
        else {
            Write-BuildLog "LLVM target is not installed."
            if (Test-AllowedToDownloadLlvm) {
                Write-BuildLog "Downloading LLVM target"
                Invoke-Task "install-llvm-from-archive"
            }
            else {
                Write-BuildLog "Downloading LLVM Disabled, building from source."
                # We don't have an external LLVM installation specified
                # We are not downloading LLVM
                # So we need to build it.
                Invoke-Task "install-llvm-from-source"
            }
            $installationDirectory = Resolve-InstallationDirectory
            Use-LlvmInstallation $installationDirectory
        }
    }
}

task install-llvm-from-archive {
    install-llvm $BuildLlvm download (Get-LLVMFeatureVersion)
    $installationDirectory = Resolve-InstallationDirectory
    Assert (Test-LlvmConfig $installationDirectory) "install-llvm-from-archive failed to install a usable LLVM installation"
}

task install-llvm-from-source -depends configure-sccache -postaction { Write-CacheStats } {
    if ($IsWindows) {
        Include vcvars.ps1
    }
    install-llvm $BuildLlvm build (Get-LLVMFeatureVersion)
    $installationDirectory = Resolve-InstallationDirectory
    Assert (Test-LlvmConfig $installationDirectory) "install-llvm-from-source failed to install a usable LLVM installation"
}

task package-manylinux-llvm -depends build-manylinux-container-image -preaction { Write-CacheStats } -postaction { Write-CacheStats } {
    # For any of the volumes mapped, if the dir doesn't exist,
    # docker will create it and it will be owned by root and
    # the caching/install breaks with permission errors.
    # New-Item is idempotent so we don't need to check for existence
    $cacheMount, $cacheEnv = Get-CCacheParams
    Write-BuildLog "Running container image: $ManylinuxTag"
    $ioVolume = "${Root}:$ManylinuxRoot"
    $userName = Get-LinuxContainerUserName

    Invoke-LoggedCommand {
        docker run --rm `
            --user $userName `
            --volume $ioVolume @cacheMount @cacheEnv `
            --workdir $ManylinuxRoot `
            --env MK_PKG_DEST=$ManylinuxRoot/target/manylinux `
            $ManylinuxTag `
            conda run --no-capture-output pwsh build.ps1 -t package-llvm
    }
}

task package-llvm {
    if ($IsWindows) {
        Include vcvars.ps1
    }
    $clear_pkg_dest_var = $false
    if (!(Test-Path env:\MK_PKG_DEST)) {
        $clear_pkg_dest_var = $true
        $env:MK_PKG_DEST = $Target
    }
    New-Item $env:MK_PKG_DEST -ItemType Directory -Force
    try {
        Invoke-LoggedCommand -workingDirectory $BuildLlvm {
            cargo build --release --no-default-features --features "package-llvm,$(Get-LLVMFeatureVersion)-no-llvm-linking" -vv
        }
    }
    finally {
        if ($clear_pkg_dest_var) {
            Remove-Item -Path Env:MK_PKG_DEST
        }
    }
}

task build-manylinux-container-image {
    Write-BuildLog "Building container image manylinux-llvm-builder"
    Invoke-LoggedCommand -workingDirectory (Join-Path $Root eng) {
        $user = Get-LinuxContainerUserName
        $uid = Get-LinuxContainerUserId
        $gid = Get-LinuxContainerGroupId
        Get-Content Dockerfile.manylinux | docker build `
            --build-arg USERNAME=$user `
            --build-arg USER_UID=$uid `
            --build-arg USER_GID=$gid `
            --build-arg RUST_VERSION=$RustVersion `
            --tag $ManylinuxTag `
            -
    }
}

task check-licenses {
    # Uses cargo-deny to verify that the linked components
    # only use approved licenses
    # https://github.com/EmbarkStudios/cargo-deny
    Invoke-LoggedCommand -wd $repo.root {
        cargo deny check licenses
    }
}

task update-noticefiles {
    # use cargo-about to generate a notice files
    # notice files are only for wheel distributions
    # as no bundled sources are in the sdist.

    # llvm special license is already in the template
    # as it is a hidden transitive dependency.
    # https://github.com/EmbarkStudios/cargo-about
    $config = Join-Path $Root notice.toml
    $template = Join-Path $Root notice.hbs
    $notice = Join-Path $Pykin NOTICE-WHEEL.txt
    Invoke-LoggedCommand -workingDirectory $Pykin {
        cargo about generate --config $config --all-features --output-file $notice $template
        $contents = Get-Content -Raw $notice
        [System.Web.HttpUtility]::HtmlDecode($contents) | Out-File $notice
    }
}

task configure-sccache -postaction { Write-CacheStats } {
    if (Test-CommandExists sccache) {
        Write-BuildLog "Starting sccache server"
        & { sccache --start-server } -ErrorAction SilentlyContinue
        Write-BuildLog "Started sccache server"
    }
}
