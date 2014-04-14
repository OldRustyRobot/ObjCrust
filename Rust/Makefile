CROSS_IOS_RUST_ROOT=/Users/vhbit/projects/external/rust/build_ios/x86_64-apple-darwin/stage2
RUSTC=$(CROSS_IOS_RUST_ROOT)/bin/rustc
RUST_FLAGS=-C relocation_model=pic -g --cfg debug --target arm-apple-ios

all: libobjcrust.a

libobjcrust.a: objcrust.rs
	$(RUSTC) $(RUST_FLAGS) $< -o $@

clean:
	rm -f *.o *.a
