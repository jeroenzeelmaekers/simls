<div align="center">

# simls

![Downloads][downloads-badge]
![License][license-badge]
![Version][version-badge]

simls is a **simulator/emulator manager** that provides an easy to use CLI interface for managing iOS simulators and Android emulators.

[How it works](#how-it-works) •
[Installation](#installation) •
[Wiki](https://github.com/jeroenzeelmaekers/simls/wiki)

![Example image][image]

</div>

## How it works

simls makes use of the cli tools of Xcode for the iOS simulators and Android studio for the android emulators.

### iOS Simulators

For iOS simulators makes use of the `xcrun simctl` cli that comes with Xcode. This exposes a series of commands that we use to manage the simulators.

### Android Emulators

For android emulators we use the `emulator` cli that gets shipped with Android Studio.

## Installation

simls is currently only available through Cargo and Homebrew. Additional system configuration steps are required and can be found in the [wiki](https://github.com/jeroenzeelmaekers/simls/wiki/Installation).

### Cargo

```shell
cargo install simls
```

### Homebrew

```shell
brew tap jeroenzeelmaekers/tap
brew install simls
```

[downloads-badge]: https://img.shields.io/github/downloads/jeroenzeelmaekers/simls/total?color=green&style=flat
[license-badge]: https://img.shields.io/github/license/jeroenzeelmaekers/simls?color=red&style=flat
[version-badge]: https://img.shields.io/github/v/release/jeroenzeelmaekers/simls?display_name=tag&style=flat&color=yellow

[image]: .assets/demo.gif
