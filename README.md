# Frankenstein with LibAFL

This repository contains the source code for Rust Homework 2.

The fuzzer to build with LibAFL is [frankenstein](https://github.com/seemoo-lab/frankenstein#basic-setup).

## Basic setup

Each firmware version is located in a different project stored in projects. A project contains the file `project.json`, which holds the symbol names and the memory layout including memory dumps. The available symbols can be used to generate patches in C as well as for firmware emulation. To build all patches and emulators for the CYW20735 evaluation board run:

```
make -C projects/CYW20735B1
```

## Reference

[LibAFL Github repo](https://github.com/AFLplusplus/LibAFL)

[Documentation](https://aflplus.plus/libafl-book/libafl.html)

[Best-tested example: libfuzzer_libpng](https://github.com/AFLplusplus/LibAFL/tree/main/fuzzers/libfuzzer_libpng)
