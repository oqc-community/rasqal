## Building from Source

Prerequisites:

1. [Python 3.10](https://www.python.org/downloads/).
2. [Rust](https://www.rust-lang.org/tools/install).

With these installed then run:

[Linux]

`sudo apt install -y build-essential libffi-dev xz-utils powershell curl wget gnupg apt-transport-https`

[Windows]

[Winget](https://learn.microsoft.com/en-us/windows/package-manager/winget/) is the windows package manager and should be installed first.

`winget install build-essential libffi-dev xz-utils powershell curl wget gnupg apt-transport-https 7-zip`

[Mac]

Soon to come.

When these tools have been downloaded you run `build.ps1` at `/src/build.ps1`. This will initialize a Python venv, build the Rust projects, install the resultant wheel into that environment and run tests. 

From this point you can build the Rust project with cargo and deal with it seperately.
But if you need to redeploy the wheel and test things from Python you need to run the build script again.

If you have issues you can look at the [CI cross-OS build script](https://github.com/oqc-community/rasqal/blob/develop/.github/workflows/deploy-wheels.yaml) and see what might be missing or out of date from the documentation.

#### Building LLVM from source

If your system has no LLVM binaries available you can [build it yourself](https://llvm.org/docs/GettingStarted.html#getting-the-source-code-and-building-llvm).
You should only attempt this if you're familiar with LLVM already or have no binaries available, as it is rather involved and finicky on certain operating systems.

You can use these environmental variables to customize the LLVM build:
```bash
RSQL_LLVM_EXTERNAL_DIR=/path/to/llvm # Directory to locally-built LLVM.
RSQL_DOWNLOAD_LLVM=true # Whether to download anrd build LLVM.
RSQL_CACHE_DIR=/where/to/extract # Where to store the downloaded LLVM build. Defaults to target which gets cleared on clean.
...
```

#### Potential issues

[PyCharm]

To get PyCharm to recognize the LLVM file path you need to add  `LLVM_SYS_140_PREFIX={path_to_repo}/src/target/llvm14-0` to the environment variables for any Rust command. You can also use a config.toml with the same value.

[Windows]

Main issue is to do with path lengths. These two changes may be needed:

* Open the registry, go to `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\FileSystem` and set `LongPathsEnabled` to 1.
* Enable long file names via git by running this: `git config --system core.longpaths true`. This will set it for every user on the system, to be only your user use `--global`.
