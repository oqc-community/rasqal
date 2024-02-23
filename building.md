## Building from Source

Prerequisites:

1. LLVM. Our build scripts can download a binary, or you can [build it yourself](https://llvm.org/docs/GettingStarted.html#getting-the-source-code-and-building-llvm).
2. [Python 3.9](https://www.python.org/downloads/).
3. [Rust](https://www.rust-lang.org/tools/install).
4. [Powershell 7](https://learn.microsoft.com/en-us/powershell/scripting/install/installing-powershell?view=powershell-7.4).

When these tools have been downloaded you run `build.ps1` in the root project folder: `/src/munchkin/build.ps1`. This will initialize a Python venv, build the Rust projects, install the resultant wheel into that environment and run tests. 

#### LLVM

If you want to customize how LLVM is built/found, the script has environment variables for a variety of ways to do so. The main ones are:

```bash
MK_LLVM_EXTERNAL_DIR=/path/to/llvm # Directory to locally-built LLVM.
MK_DOWNLOAD_LLVM=true # Whether to download and build LLVM.
MK_CACHE_DIR=/where/to/extract # Where to store the downloaded LLVM build. Defaults to target which gets cleared on clean.
...
```

#### Potential issues

[PyCharm]

To get PyCharm to recognize the LLVM file path you need to add  `LLVM_SYS_140_PREFIX={path_to_repo}/src/munchkin/target/llvm14-0` to the environment variables for any Rust command. You can also use a config.toml with the same value.

[Windows]

Main issue is to do with path lengths. These two changes may be needed:

* Open the registry, go to `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\FileSystem` and set `LongPathsEnabled` to 1.
* Enable long file names via git by running this: `git config --system core.longpaths true`. This will set it for every user on the system, to be only your user use `--global`.
