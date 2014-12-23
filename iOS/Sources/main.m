#import "CRAppDelegate.h"
#import "objcrust.h"
#import <execinfo.h>

extern void rust_main();

int main(int argc, char *argv[]) {
    // Enable printing backtraces
    setenv("RUST_BACKTRACE", "1", 1);

    rust_main();
    
    @autoreleasepool {
        return UIApplicationMain(argc, argv, nil, NSStringFromClass(CRAppDelegate.class));
    }
}
