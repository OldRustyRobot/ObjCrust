#!/bin/sh

set -e

function get_triple {
    local TARGET=""
    case $1 in
        armv7s*)
            TARGET="armv7s-apple-ios"
            ;;
        armv7*)
            TARGET="armv7-apple-ios"
            ;;
        arm64*)
            TARGET="aarch64-apple-ios"
            ;;
        i386*)
            TARGET="i386-apple-ios"
            ;;
        x86_64*)
            TARGET="x86_64-apple-ios"
            ;;
    esac
    eval "$2=$TARGET"
}

function get_target_dir {
    if [[ $CONFIGURATION == *Release* ]] ; then
        eval "$2=target/$1/release"
    else
        eval "$2=target/$1/debug"
    fi
}

function build_arch {
    local TARGET=$1
    local EXTRA_FLAGS=""

    local out_dir=''
    get_target_dir $TARGET out_dir

    if [[ $CONFIGURATION == *Release* ]] ; then
        EXTRA_FLAGS="--release --verbose"
    else
        EXTRA_FLAGS="--verbose"
    fi

    echo "building for arch: $target"
    # preparing a special environment as a workaround of some weird failures
    env -i PATH="$PATH" HOME="$HOME" BUILT_PRODUCTS_DIR="$BUILT_PRODUCTS_DIR" TARGET="$TARGET" cargo build --target="$TARGET" -j4 --lib $EXTRA_FLAGS 2>&1
}

function build {
    cd $CARGO_ROOT_PATH


    if [[ $ONLY_ACTIVE_ARCH == YES ]] ; then
        BUILD_ARCHS=$CURRENT_ARCH
    else
        BUILD_ARCHS=$ARCHS
    fi

    OUT_LIBS=""
    for arch in $BUILD_ARCHS ; do
        target=""
        get_triple $arch target
        build_arch $target
        out_dir=""
        get_target_dir $target out_dir
        OUT_LIBS="$OUT_LIBS $out_dir/$RUST_LIB_NAME"
    done

    echo "creating built products dir"
    mkdir -p $BUILT_PRODUCTS_DIR
    echo "creating fat library"
    lipo -create -output "$BUILT_PRODUCTS_DIR/$RUST_LIB_NAME" $OUT_LIBS
}

build
