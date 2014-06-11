#import "CRAppDelegate.h"
#import "objcrust.h"

extern void run_rust_main();
extern void register_task(const char *);
extern void deregister_task();

int main(int argc, char *argv[]) {
    // Enable printing backtraces
    // It works correctly in Simulator, but doesn't work yet on the device
    setenv("RUST_BACKTRACE", "1", 1);

    // Initially registering task which is required
    // to get std libs working
    register_task("<main>");
    
    // While it is possible to call rust_main directly
    // it will crash the whole app as uses fail!()
    // run_rust_main just wraps it into unwinder
    run_rust_main();
    
    @autoreleasepool {
        return UIApplicationMain(argc, argv, nil, NSStringFromClass(CRAppDelegate.class));
    }
}
