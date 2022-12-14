# Frankenstein with LibAFL

This repository contains the source code for Rust Homework 2.

The fuzzer to build with LibAFL is [frankenstein](https://github.com/seemoo-lab/frankenstein#basic-setup). Frankenstein provides a virtual environment to fuzz wireless firmwares. Firmwares can be hooked during runtime to extract their current state (i.e., xmitstate through InternalBlue). Then, they can be re-executed in a virtual environment for fuzzing. To do so, the firmware image needs to be reassembled to an ELF file that can be executed with QEMU. The firmware image reassembly is simplified by a web-based UI.

## Basic setup

Each firmware version is located in a different project stored in projects. A project contains the file `project.json`, which holds the symbol names and the memory layout including memory dumps. The available symbols can be used to generate patches in C as well as for firmware emulation. To build all patches and emulators for the CYW20735 evaluation board run:

```
make -C projects/CYW20735B1
```

In general, having the project built is sufficient to run emulation with QEMU. After rebuilding the project using make -C projects/CYW20735B1, the firmware state can be emulated, until the Idle thread is entered. For this, execute:
```
qemu-arm projects/CYW20735B1/gen/execute.exe
```

And this is where we ran into trouble. 

## Problem

Initially in order to have the latest QEMU environment, we built our own filesystem, linux kernel and QEMU from scratch. But when we tried emulation, qemu-arm always gave the same Segmentation fault as below:

```
qemu: uncaught target signal 11 (Segmentation fault) - core dumped
Segmentation fault (core dumped)
```

The debug information is very limited, but we still tried with gdb to see what happened down there:
```
gdb-multiarch -q -ex "target remote 127.0.0.1:1337" projects/CYW20735B1/gen/execute.exe
```

and got the following output:
```
Reading symbols from projects/CYW20735B1/gen/execute.exe...
(No debugging symbols found in projects/CYW20735B1/gen/execute.exe)
Remote debugging using 127.0.0.1:1337
0x0bef7ce4 in _start ()
(gdb) n
Single stepping until exit from function _start,
which has no line number information.

Program received signal SIGSEGV, Segmentation fault.
0x00210006 in _binary_segment_groups_default_Segment_0x200000_bin_start ()
(gdb) n
Single stepping until exit from function _binary_segment_groups_default_Segment_0x200000_bin_start,
which has no line number information.

Program terminated with signal SIGSEGV, Segmentation fault.
The program no longer exists.
```

`_start()_` is the entrance function for every frankenstein fuzzing script. Right after `_start()`, a signal SIGSEGV is received at 0x00210006 in `_binary_segment_groups_default_Segment_0x200000_bin_start ()`. This function can't be found in the frankenstein code, so it must be in the CYW20735 closed-source firmware which was dumped and accessed by frankenstein in segment binary files. 

We check the ELF file with `readelf -hS projects/CYW20735B1/gen/execute.exe`:
```
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 61 00 00 00 00 00 00 00 00 
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            ARM
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0xbef7ce4
  Start of program headers:          52 (bytes into file)
  Start of section headers:          6698228 (bytes into file)
  Flags:                             0x0
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         20
  Size of section headers:           40 (bytes)
  Number of section headers:         25
  Section header string table index: 24

Section Headers:
  [Nr] Name              Type            Addr     Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            00000000 000000 000000 00      0   0  0
  [ 1] .Segment_0x0      PROGBITS        00000000 010000 200000 00  WA  0   0  1
  [ 2] .Segment_0x200000 PROGBITS        00200000 210000 050000 00  WA  0   0  1
  [ 3] .Segment_0x270000 PROGBITS        00270000 260000 010000 00  WA  0   0  1
  [ 4] .Segment_0x280000 PROGBITS        00280000 270000 004000 00  WA  0   0  1
  [ 5] .Segment_0x300000 PROGBITS        00300000 280000 008000 00  WA  0   0  1
  [ 6] .Segment_0x310000 PROGBITS        00310000 290000 012000 00  WA  0   0  1
  [ 7] .Segment_0x326000 PROGBITS        00326000 2a6000 00a000 00  WA  0   0  1
  [ 8] .Segment_0x338000 PROGBITS        00338000 2b8000 030000 00  WA  0   0  1
  [ 9] .Segment_0x370000 PROGBITS        00370000 2f0000 010000 00  WA  0   0  1
  [10] .Segment_0x390000 PROGBITS        00390000 300000 008000 00  WA  0   0  1
  [11] .Segment_0x410000 PROGBITS        00410000 310000 004000 00  WA  0   0  1
  [12] .Segment_0x420000 PROGBITS        00420000 320000 004000 00  WA  0   0  1
  [13] .Segment_0x430000 PROGBITS        00430000 330000 004000 00  WA  0   0  1
  [14] .Segment_0x440000 PROGBITS        00440000 340000 004000 00  WA  0   0  1
  [15] .Segment_0x450000 PROGBITS        00450000 350000 004000 00  WA  0   0  1
  [16] .Segment_0x500000 PROGBITS        00500000 360000 100800 00  WA  0   0  1
  [17] .Segment_0x640000 PROGBITS        00640000 470000 000800 00  WA  0   0  1
  [18] .Segment_0x650000 PROGBITS        00650000 480000 000800 00  WA  0   0  1
  [19] .Segment_0xe00000 PROGBITS        e0000000 4b0000 100000 00  WA  0   0  1
  [20] .text             PROGBITS        0beef000 48f000 008d80 00  AX  0   0  8
  [21] .data             PROGBITS        0bef8000 498000 00c348 04 WAX  0   0  4
  [22] .symtab           SYMTAB          00000000 5b0000 04bb40 10     23 790  4
  [23] .strtab           STRTAB          00000000 5fbb40 067837 00      0   0  1
  [24] .shstrtab         STRTAB          00000000 663377 00017a 00      0   0  1
```

0x00210006 is located in `.Segment_0x200000`, which proves that `_binary_segment_groups_default_Segment_0x200000_bin_start ()` isn't accessible since we don't have the source code.

After that we spent more than a week working on frankenstein with almost no documentation available, thinking that there might be some mistakes messing up the environment. But there is almost no QEMU related code and instrumentation in frankenstein, so it is unclear why QEMU version affects. We found many hardcoded addresses in frankenstein source code:
```
// frankenstein-dev/projects/CYW20735B1/emulation/hci.h:
// the UART interface seems to have a DMA like receive
    char state = *(char *)(0x249f70 + 0xd); // rx_machine state
    print_var(state);
    if (state == 6)
    {
        void *data_ptr = *(void **)(0x249f70 + 0x10);  // rx_data_ptr
        uint32_t len = *(uint32_t *)(0x249f70 + 0x14); // rx_len
...

//frankenstein-dev/projects/CYW20735B1/emulation/common.h:
 *((void **)0x02003d4) = clean_exit; //Here is an alternative return from interrupt... XXX
```

These hardcode implementation are related to packet reception and interrupt handlers which are crucial. We suspect the lower layout of QEMU emulation may change, so the hardcode addresses trigger the core fault.

Until we don't know what to do at all, we removed the latest QEMU built from source code and reinstalled one through `apt`. And frankenstein magically came alive in the very outdated version of QEMU:
```
qemu-arm version 4.2.1 (Debian 1:4.2-3ubuntu6.24)
Copyright (c) 2003-2019 Fabrice Bellard and the QEMU Project developers
```

But it was reasonable because frankenstein was published in 2020. A lot of changes could happen to QEMU in the past 3 years.

## LibAFL QEMU

So we port the emulation environment into `libafl_qemu`. LibAFL checks the `CUSTOM_QEMU_DIR` to see whether a LibAFL QEMU is installed:
```
// LibAFL-main/libafl_qemu/build_linux.rs: pub fn build()
let qemu_path = if let Some(qemu_dir) = custum_qemu_dir.as_ref() {
        Path::new(&qemu_dir).to_path_buf()
    } else {
        let qemu_path = target_dir.join(QEMU_DIRNAME);

        let qemu_rev = target_dir.join("QEMU_REVISION");
        if qemu_rev.exists()
            && fs::read_to_string(&qemu_rev).expect("Failed to read QEMU_REVISION") != QEMU_REVISION
        {
            drop(fs::remove_dir_all(&qemu_path));
        }

        if !qemu_path.is_dir() {
            println!(
                "cargo:warning=Qemu not found, cloning with git ({})...",
                QEMU_REVISION
            );
```            

If not, it will git clone a QEMU 7.2 and checkout to its patched version with commit ID 
```
const QEMU_REVISION: &str = "ddb71cf43844f8848ae655ca696bdfc3fb7839f1";
```

Since LibAFL project was only established a year ago, it patches the latest __QEMU 7.2__ with Rust. frankenstein crashes in LibAFL QEMU as expected:
```
qemu: uncaught target signal 11 (Segmentation fault) - core dumped
**
ERROR:../accel/tcg/cpu-exec.c:978:cpu_exec: assertion failed: (cpu == current_cpu)
Bail out! ERROR:../accel/tcg/cpu-exec.c:978:cpu_exec: assertion failed: (cpu == current_cpu)
```

## Analysis

The QEMU environment is still at startup since the [code]() is really simple:
```
// Initialize QEMU
    // env::remove_var("LD_LIBRARY_PATH");
    let args = vec![
        "qemu-arm".to_string(),
        "../../../frankenstein-dev/projects/CYW20735B1/gen/execute.exe".to_string(),
    ];
    let env: Vec<(String, String)> = Vec::new();
    let emu = Emulator::new(&args, &env);

    emu.set_breakpoint(XML_RPC_CALL);
    unsafe {
        emu.run();
    }
    emu.remove_breakpoint(XML_RPC_CALL);

    println!("QEMU initialized");
```

And it failed with the same output with unpatched QEMU 7.2. __Currently, the only solution is to reimplement LibAFL patches on QEMU 4.2 which proved to work with frankenstein. But the workload would be extremely heavy and patching an outdated QEMU is meaningless despite finishing this homework.__ 

So we don't know what to do now. Anyone help us please!


We have finished the input generation part of frankenstein with LibAFL. 

## Reproduction

In order to run frankenstein, the following dependencies are needed:
```
apt install qemu-user gcc-arm-none-eabi gcc-multilib
```

Note that `qemu-user` must be installed through `apt` to acquire an outdated version. Out tested enviroment is 
```
qemu-arm version 4.2.1 (Debian 1:4.2-3ubuntu6.24)
Linux 5.4.0-135-generic
Ubuntu 20.04 LTS
gcc (Ubuntu 9.4.0-1ubuntu1~20.04.1) 9.4.0
Python 3.8.10
```

To reproduce the crash, under `LibAFL-main/fuzzers/fran` run:
```
cargo clean
cargo make build
cargo make run
```

## Reference

### LibAFL Related

[LibAFL Github repo](https://github.com/AFLplusplus/LibAFL)

[Documentation](https://aflplus.plus/libafl-book/libafl.html)

[Best-tested example: libfuzzer_libpng](https://github.com/AFLplusplus/LibAFL/tree/main/fuzzers/libfuzzer_libpng)

### LibAFL QEMU related

[Fuzzing101 with LibAFL - Part IV: Fuzzing LibTIFF](https://epi052.gitlab.io/notes-to-self/blog/2021-11-26-fuzzing-101-with-libafl-part-4/)

[Hacking TMNF: Part 1 - Fuzzing the game server](https://blog.bricked.tech/posts/tmnf/part1/#recap)