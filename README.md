<div align="center">

# simls

![Downloads][downloads-badge]

simls is a **simulator/emulator manager** that provides an easy to use CLI interface for managing iOS simulators and Android emulators.

[How it works](#how-it-works) •
[Installation](#installation)

![Example image][image]

</div>

## How it works

simls makes use of the cli tools of Xcode for the iOS simulators and Android studio for the android emulators.

### iOS Simulators

For iOS simulators makes use of the `xcrun simctl` cli that comes with Xcode. This exposes a series of commands that we use to manage the simulators.

### Android Emulators

For android emulators we use the `emulator` cli that gets shipped with Android Studio.

## Installation

simls is currently only available through Cargo and Homebrew.

### Cargo

```shell
cargo install simls
```

### Homebrew

```shell
brew tap jeroenzeelmaekers/tap
brew install simls
```

[downloads-badge]: https://img.shields.io/github/downloads/jeroenzeelmaekers/simls/total?color=green&style=flat-sq
[image]: .assets/demo.gif
