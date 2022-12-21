
INCLUDE gen/symbols.ld;
ENTRY(_start);
SECTIONS {
    . = 0xbeef000;
    .text : { *(.text) }
    .data : { *(.data) }
    .bss : { *(.bss) }
}
