MEMORY {
  /* Standalone Code uses the first flash bank, nonsecure alias */
  FLASH (rx): ORIGIN = 0x08000000, LENGTH = 2M
  /* Standalone Code uses SRAM1, nonsecure alias */
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 768K
}
