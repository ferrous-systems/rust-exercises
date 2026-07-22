MEMORY {
  /* Nonecure Code uses the second flash bank, nonsecure alias */
  FLASH (rx): ORIGIN = 0x08200000, LENGTH = 2M
  /* Nonecure Code uses SRAM3, nonsecure alias */
  RAM (rwx) : ORIGIN = 0x200D0000, LENGTH = 832K
}
