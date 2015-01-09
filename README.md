# ObjCrust

A modified ObjCrust which uses Rust
cross-compiler. [Cross-compiler](https://github.com/vhbit/Rust/tree/ios)
needs to be built first (note: it is on a separate branch now, so
don't forget to checkout it before building).

Requires Xcode 5 and the iOS 7 SDK.

Based on [doublec/rust-from-c-example](https://github.com/doublec/rust-from-c-example)

## Usage

1. `git clone https://github.com/vhbit/ObjCrust.git`
2. change CROSS_IOS_RUST_ROOT to your local cross compiler root
3. `cd ObjCrust/Rust`
4. `make`
5. `cd ../iOS`
6. `open ObjCrust.xcodeproj`
7. Build and run in Xcode
