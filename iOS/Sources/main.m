#import "CRAppDelegate.h"
#import "objcrust.h"

extern void try_init();

int main(int argc, char *argv[]) {
    NSLog(@"Doc dir %@", [NSSearchPathForDirectoriesInDomains(NSApplicationDirectory, NSAllDomainsMask, YES) objectAtIndex:0]);
    try_init();
    
    @autoreleasepool {
        return UIApplicationMain(argc, argv, nil, NSStringFromClass(CRAppDelegate.class));
    }
}
