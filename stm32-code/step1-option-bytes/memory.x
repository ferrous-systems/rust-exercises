MEMORY {
  /* Running from start of flash (because TZEN=0) */
  FLASH (rx): ORIGIN = 0x08000000, LENGTH = 128K
  /* Use start of RAM too */
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 128K
}
