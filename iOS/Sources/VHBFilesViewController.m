//
//  VHBFilesViewController.m
//  ObjCrust
//
//  Created by Valerii Hiora on 14/02/15.
//  Copyright (c) 2015 Valerii Hiora. All rights reserved.
//

#import "VHBFilesViewController.h"
#import "../../Rust/objcrust.h"

int rw_file_native(char const *path);

@interface VHBFilesViewController ()
@property (weak, nonatomic) IBOutlet UITextView *debugOutput;
@end

int rw_file_native(char const *path) {
    int fd = open(path, O_RDWR | O_TRUNC | O_CREAT, S_IRUSR | S_IWUSR);
    if (fd < 0)
        return fd;

    char *buf = "HelloWorld";
    write(fd, buf, strlen(buf));
    close(fd);
    
    return 0;
}

@implementation VHBFilesViewController

- (void)viewDidLoad {
    [super viewDidLoad];
    // Do any additional setup after loading the view from its nib.

    UIBarButtonItem *refreshButton = [[UIBarButtonItem alloc] initWithBarButtonSystemItem:UIBarButtonSystemItemRefresh
                                                                                   target:self
                                                                                   action:@selector(rerunTest)];
    self.navigationItem.rightBarButtonItem = refreshButton;

    [self rerunTest];
}

- (void)rerunTest {
    NSMutableString *output = [NSMutableString string];

    NSString *docFolder = NSSearchPathForDirectoriesInDomains(NSDocumentDirectory, NSAllDomainsMask, YES)[0];
    NSFileManager *fileManager = [NSFileManager defaultManager];
    if (![fileManager fileExistsAtPath:docFolder] && ![fileManager createDirectoryAtPath:docFolder
                                                             withIntermediateDirectories:YES
                                                                              attributes:nil
                                                                                   error:nil]) {
        [output appendString:@"Failed to create dir folder\n"];
    }

    {
        [output appendString:@"Writing rust file:\n"];
        NSUUID *ioUuid = [NSUUID UUID];

        NSString *ioFile = [docFolder stringByAppendingPathComponent:ioUuid.UUIDString];
        if (rw_file_io(ioFile.UTF8String) < 0) {
            [output appendFormat:@"Failed to write through IO: %@\n", ioUuid.UUIDString];
        } else {
            [output appendString:[self fileInfo:ioFile]];
            [output appendString:@"\n\n"];
        }
    }


    {
        [output appendString:@"Writing rust raw:\n"];
        NSUUID *rawUuid = [NSUUID UUID];

        NSString *rawFile = [docFolder stringByAppendingPathComponent:rawUuid.UUIDString];
        if (rw_file_raw(rawFile.UTF8String) < 0) {
            [output appendFormat:@"Failed to write through raw: %@\n", rawUuid.UUIDString];
        } else {
            [output appendString:[self fileInfo:rawFile]];
            [output appendString:@"\n\n"];
        }
    }

    {
        [output appendString:@"Writing rust raw (vararg):\n"];
        NSUUID *rawVarArgUuid = [NSUUID UUID];

        NSString *rawVarArgFile = [docFolder stringByAppendingPathComponent:rawVarArgUuid.UUIDString];
        if (rw_file_raw_vararg(rawVarArgFile.UTF8String) < 0) {
            [output appendFormat:@"Failed to write through raw (vararg): %@\n", rawVarArgUuid.UUIDString];
        } else {
            [output appendString:[self fileInfo:rawVarArgFile]];
            [output appendString:@"\n\n"];
        }
    }


    {
        [output appendString:@"Writing native:\n"];
        NSUUID *nativeUUID = [NSUUID UUID];

        NSString *nativeFile = [docFolder stringByAppendingPathComponent:nativeUUID.UUIDString];
        if (rw_file_native(nativeFile.UTF8String) < 0) {
            [output appendFormat:@"Failed to write through native: %@\n", nativeUUID.UUIDString];
        } else {
            [output appendString:[self fileInfo:nativeFile]];
            [output appendString:@"\n\n"];
        }
    }


    _debugOutput.text = output;
}

- (NSString *)fileInfo:(NSString *)path {
    NSDictionary *attrs = [[NSFileManager defaultManager] attributesOfItemAtPath:path error:nil];

    return [NSString stringWithFormat:@"Name: %@, size: %@, permissions: %@",
                    [path lastPathComponent], attrs[NSFileSize], attrs[NSFilePosixPermissions]];
}

@end
