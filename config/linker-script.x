STACK_SIZE = 0x2000;

SECTIONS
{
  /* Start address */
  . = 0x80000000;

  /* Output a text section, starting with the entry point */
  .entry_point : ALIGN(0x1000) {
     _start
    *(.entry_point)
  }
  . = ALIGN(0x4);
  _text_start = .;
  .text : {
    *(.text)
    *(.text.*)
  }
  _text_end = .;

  /* Output the rodata */
  . = ALIGN(0x1000);
  _rodata_start = .;
  .rodata : {
    KEEP(*(__*))
    *(.rodata)
    *(.rodata.*)
  }
  _rodata_end = .;

  /* Finally, all data                                         */
  /* NOTE: no need to page-align bss, both bss and data are RW */

  . = ALIGN(0x1000);
  _data_start = .;
  .data : {
    KEEP(*(__*))
    *(.data)
    *(.data.*)
  }
  _data_end = .;

  _bss_start = .;
  .bss : {
    *(.bss)
    *(.bss.*)
  }
  _bss_end = .;

  /* Then we allocate some stack space */
  . = ALIGN(0x1000);
  _stack_start = .;
  .stack :  {
   }
   _stack_end = .;
   _heap_start = .;
}

