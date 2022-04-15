# Creating a Rust library and calling it from Java/Kotlin

## One-time setup

### Android Studio

* Install [Android Studio](https://developer.android.com/studio).
* Open the [SDK manager](https://developer.android.com/studio/intro/update#sdk-manager) in Android
  Studio, and [install the NDK](https://developer.android.com/studio/projects/install-ndk).
* Open Preferences > Plugins in Android Studio, and install
  the [Rust plugin](https://plugins.jetbrains.com/plugin/8182-rust).

### Rust

* [Install Rust](https://www.rust-lang.org/tools/install).
* Add Android targets to
  Rust: `rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android`

