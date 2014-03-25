all: libobjcrust.a

ARCH_LIBS = objcrust-i386.ia \
	objcrust-armv7.ia \
	objcrust-armv7s.ia

objcrust-%.bc: objcrust.rs
	rustc --emit bc -g --target $*-apple-darwin objcrust.rs -o $@

objcrust-%.o: objcrust-%.bc
	clang -arch $* -c $<

objcrust-%.ia: objcrust-%.o
	ar -cr $@ $<

libobjcrust.a: $(ARCH_LIBS)
	lipo -create -output libobjcrust.a $(ARCH_LIBS)

clean:
	rm -f *.bc *.o *.a *.ia
