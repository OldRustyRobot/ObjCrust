CROSS_IOS_RUST_ROOT=/Users/vhbit/projects/external/rust/build_latest/x86_64-apple-darwin/stage2
RUSTC ?= $(CROSS_IOS_RUST_ROOT)/bin/rustc
RUST_FLAGS=-O
TARGETS=arm-apple-ios i386-apple-ios
LIB_NAME=libobjcrust-$(1).a
ARCH_LIBS=$(foreach target,$(TARGETS),$(call LIB_NAME,$(target)))

all: libobjcrust.a

define ARCH_LIB
$(call LIB_NAME,$(1)): objcrust.rs
	$(RUSTC) $(RUST_FLAGS) --target $(1) $$< -o $$@
endef

libobjcrust.a: $(ARCH_LIBS)
	lipo -create -output $@ $(ARCH_LIBS)

$(foreach target,$(TARGETS), \
	$(eval $(call ARCH_LIB,$(target))))

clean:
	rm -f *.o *.a
