ENTRY(_start)
SECTIONS {
  .text 0x100 : { *(.text) }
  .data 0x200 : { *(.data) }
}
