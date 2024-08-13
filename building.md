## Building from Source

Prerequisites:

1. [Python 3.10](https://www.python.org/downloads/).
2. [Rust](https://www.rust-lang.org/tools/install).

With these installed then run:

[Linux]

`sudo apt-get install -y ninja-build`

[Windows]

[Chocolatey](https://chocolatey.org/install) is a windows package manager that we use to organize some installs.

`choco install --accept-license -y ninja`

Optional: 
`choco uninstall -y llvm` if you are having problems with the correct LLVM install being found. 

[Mac]

`brew install ccache ninja`

When this is all done run `build.ps1` at `/src/build.ps1`. 

This will install and build LLVM, build all the Rust projects, build a wheel and install it into the local venv, perform style formatting and more.

Once this has been done you can open up the Rust projects and run cargo like normal as LLVM has been built and cached. You cna also use the built venv to run the Python as well.

You'll have to re-run the build script if you want to build a new wheel to be available from the Python, but beyond that you can develop in whatever environment most suits you.

#### Potential issues

[PyCharm]

To get PyCharm to recognize the LLVM file path you need to add  `LLVM_SYS_150_PREFIX={path_to_repo}/src/target/llvm15-0` to the environment variables for any Rust command. You can also use a config.toml with the same value.

[Windows]

Main issue is to do with path lengths. These two changes may be needed:

* Open the registry, go to `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\FileSystem` and set `LongPathsEnabled` to 1.
* Enable long file names via git by running this: `git config --system core.longpaths true`. This will set it for every user on the system, to be only your user use `--global`.
