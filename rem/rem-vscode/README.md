# REMVSCode

**REM VSCode**: A new Rust Refactoring Extension for Visual Studio Code

**REM VSCode** is an interface over the powerful REM extract and verifcation
toolchain for Rust development. It aims to provide a seamless experience for
Rust developers looking to perform correct extract method refactorings. It
combines the best aspects of Rust Analyzer's code analysis capabilities with the
refactoring capabilities of the REM toolchain. Additionally it leverages the
AENEAS toolchain to provide (optional) static verification of the refactored code.

Currently it is in the development phase, as part of a Research Project being
conducted at the Australian National University by:
- Matthew Britton (matt.britton@anu.edu.au)
- Alex Potanin

https://marketplace.visualstudio.com/items?itemName=MatthewBritton.remvscode&ssr=false#overview


## Installation

REM-VSCode requires a number of components to be installed on your system for it
to function correctly. As a general overview, the following components are required for the extension
to function, and the extension will prompt you to install them if they are not
already installed on your system:

- rem-server (from Cargo / crates.io)
- openssl-dev
- git
- make
- cargo (and by extension, the most recent nightly toolchain)
- rust-src rust-std rustc-dev llvm-tools-preview rust-analyzer-preview rustfmt
- pkg-config
- opam
- dune
- coq (8.18.0 minimum)
- CHARON (https://github.com/AeneasVerif/charon)
- AENEAS (https://github.com/AeneasVerif/aeneas)

Alternatively, you can install the required components manually. This will only
work if you DO NOT have nix installed on your system. If you do, the
installation of CHARON and AENEAS will fail, as their isntallation processes
currently fail if you have nix installed.

Alternatively, you can install AENEAS using this fork of the original
respository, that has been modified to ignore the presence of nix and only
install via rustup:

https://github.com/RuleBrittonica/aeneas.git

### MacOS Specific Installation Instructions

Both Homebrew and XCode Command Line Tools are required to be installed on your
system for the extension to install the required components. Homebrew is a
package manager for MacOS that allows you to install software from the command
line. XCode Command Line Tools are a set of tools that allow you to compile and
build tools from the command line. You can install Homebrew from the following
link:
https://brew.sh/
Once Homebrew is installed, you can install the XCode Command Line Tools by
running the following command in the terminal:

```bash
xcode-select --install
```


## Current Features

- **Extract Method Refactoring**: The extension provides a complete extract
  method refactoring tool, allowing you to extract code into a new function or
  method. This feature relies on an incremental cache of the current program
  workspace to provide fast and accurate refactorings, but does thus require the
  cache to be built first.
- **Lifetime Repair**: The Extract method  refactoring tool can be optionally
  extended to perform lifetime repair on the extracted method
- **Static Verification**: The extract method refactoring tool can be optionally
  extended to perform static verification of the refactored code using the AENEAS
  toolchain. This will verify that the extracted method is correct with respect
  to the original code. It is reccomended to use this feature if possible.


## Requirements

- **Visual Studio Code**: Version ^1.92.0 or higher.
- All other dependencies will be installed automatically by the extension, or
  you can install them manually as described above.

## Extension Settings

You may need to manually set the path to the following binaries if they are not
automatically detected by the extension. The extension notifies you on startup
if it cannot find these binaries.
- `rem-server`: Path to the rem-server binary.
- `aeneas`: Path to the aeneas binary.
- `charon`: Path to the charon binary.

## Known Issues

- Currently, there are no known issues. Please report any bugs or feature requests on the [GitHub repository](https://github.com/RuleBrittonica/rem-vscode).

## Release Notes

### 0.1.0

- Initial release of REM VSCode.
- Test of the build process, has NO REFACTORING CAPABILITIES.

### 0.2.0

- Added Extract Method Refactoring capability.
- Added Lifetime Repair option to Extract Method Refactoring.
- Added Static Verification option to Extract Method Refactoring.

**Enjoy using REM VSCode!**