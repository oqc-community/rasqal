# Copyright (c) Microsoft Corporation.
# Licensed under the MIT License.

# SPDX-License-Identifier: BSD-3-Clause
# Copyright (c) 2024 Oxford Quantum Circuits Ltd

Include utils.ps1

Properties {
    $Root = Resolve-Path (Split-Path -Parent $PSScriptRoot)
    $BuildLlvm = Join-Path $Root build-llvm
    $Munchkin = Join-Path $Root munchkin
    $Target = Join-Path $Root target
    $Wheels = Join-Path $Target wheels
    $CargoConfigToml = Join-Path $Root .cargo config.toml
    $RustVersion = "1.64.0"
    $Python = Resolve-Python
}

task default -depends build
task build -depends build-llvm, build-munchkin, test-munchkin
task check -depends check-licenses
task format -depends format-rust
task all -depends build, check, format

task format-rust {
    Invoke-LoggedCommand -workingDirectory $Root {
        cargo fmt --all
    }
}

# task clippy {
#     Invoke-LoggedCommand -workingDirectory $Root {
#         cargo clippy --fix -- --no-deps
#     }
# }

task build-llvm -depends init {
    Invoke-LoggedCommand -workingDirectory $BuildLlvm { cargo test --release @(Get-CargoArgs) }
    Invoke-LoggedCommand -workingDirectory $BuildLlvm { cargo build --release @(Get-CargoArgs) }
}

task build-munchkin -depends init {
    $env:MATURIN_PEP517_ARGS = (Get-CargoArgs) -Join " "
    Get-Wheels munchqin | Remove-Item -Verbose
    Invoke-LoggedCommand { pip --verbose wheel --no-deps --wheel-dir $Wheels $Munchkin }

    if (Test-CommandExists auditwheel) {
        $unauditedWheels = Get-Wheels munchqin
        Invoke-LoggedCommand { auditwheel repair --wheel-dir $Wheels $unauditedWheels }
        $unauditedWheels | Remove-Item
    }

    # Force reinstall the package if it exists, but not its dependencies.
    $packages = Get-Wheels munchqin
    Invoke-LoggedCommand -workingDirectory $Root {
        pip install $packages
        pip install --force-reinstall --no-deps $packages
    }
}

task test-munchkin -depends build-munchkin {
    Invoke-LoggedCommand -workingDirectory $Munchkin {
        cargo test --release @(Get-CargoArgs)
    }

    Invoke-LoggedCommand -workingDirectory $Root {
        pip install pytest
        pytest .
    }
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

task install-llvm-from-source {
    if ($IsWindows) {
        Include vcvars.ps1
    }
    install-llvm $BuildLlvm build (Get-LLVMFeatureVersion)
    $installationDirectory = Resolve-InstallationDirectory
    Assert (Test-LlvmConfig $installationDirectory) "install-llvm-from-source failed to install a usable LLVM installation"
}

task check-licenses {
    # Uses cargo-deny to verify that the linked components
    # only use approved licenses
    # https://github.com/EmbarkStudios/cargo-deny
    Invoke-LoggedCommand -wd $Root {
        cargo install cargo-deny
        cargo deny check licenses
    }

    Invoke-LoggedCommand -wd $Root {
        pip install pip-licenses
        pip-licenses --allow-only="BSD License;Apache Software License;MIT License;MIT No Attribution License (MIT-0);BSD-3-Clause;Mozilla Public License 2.0 (MPL 2.0);MIT OR Apache-2.0;MIT"
    }
}

# task update-noticefiles {
#     # use cargo-about to generate a notice files
#     # notice files are only for wheel distributions
#     # as no bundled sources are in the sdist.
#
#     # llvm special license is already in the template
#     # as it is a hidden transitive dependency.
#     # https://github.com/EmbarkStudios/cargo-about
#     $config = Join-Path $Root notice.toml
#     $template = Join-Path $Root notice.hbs
#     $notice = Join-Path $Munchkin NOTICE-WHEEL.txt
#     Invoke-LoggedCommand -workingDirectory $Munchkin {
#         cargo about generate --config $config --all-features --output-file $notice $template
#         $contents = Get-Content -Raw $notice
#         [System.Web.HttpUtility]::HtmlDecode($contents) | Out-File $notice
#     }
# }
