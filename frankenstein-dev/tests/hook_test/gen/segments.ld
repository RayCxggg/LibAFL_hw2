INCLUDE gen/symbols.ld;
ENTRY(_start);
MEMORY {
  Segment_0x0 (rwx)  : ORIGIN = 0x0, LENGTH = 0x9ea0
  Segment_0x19e9c (rwx)  : ORIGIN = 0x19e9c, LENGTH = 0x3450
}
SECTIONS {
  .Segment_0x0 0x0:{ gen/hook_test.exe/Segment_0x0.segment.o} > Segment_0x0
  .Segment_0x19e9c 0x19e9c:{ gen/hook_test.exe/Segment_0x19e9c.segment.o} > Segment_0x19e9c
  .text 0xbeef000:{ gen/src.o (.text)}
  .data ALIGN(LOADADDR(.text)+SIZEOF(.text), 0x1000):{ * (*)}
}
