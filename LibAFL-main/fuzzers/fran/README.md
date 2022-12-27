# frankenstein with LibAFL

This folder contains the frankenstein fuzzer for Bluetooth embedded firmware.

## Tutorial

### Link Manager Protocol fuzzing

```
cat /dev/urandom | qemu-arm projects/CYW20735B1/gen/lmp_fuzz.exe
```

When a testcase triggers CVE-2019-11516, qemu catches a Segmentation fault and stops fuzzing:
```
qemu: uncaught target signal 11 (Segmentation fault) - core dumped
Segmentation fault (core dumped)
```

### ACL mode fuzzing

```
cat /dev/urandom | qemu-arm projects/CYW20735B1/gen/acl_fuzz.exe
```

### Host Controller Interface fuzzing

```
cat /dev/urandom | qemu-arm projects/CYW20735B1/gen/hci_attach.exe
```


## Troubleshooting

During build patches and emulators `make -C projects/CYW20735B1`, output:
```
In file included from /usr/include/sys/ioctl.h:26,
                 from ../../include/frankenstein/syscalls.h:33,
                 from ../../include/frankenstein/utils.h:107,
                 from emulation/heap.c:1:
/usr/include/bits/ioctls.h:23:10: fatal error: asm/ioctls.h: No such file or directory
   23 | #include <asm/ioctls.h>
      |          ^~~~~~~~~~~~~~
compilation terminated.
```
You need to link `/asm-generic` toolchain to dir.
```
ln -s /usr/include/asm-generic /usr/include/asm
```