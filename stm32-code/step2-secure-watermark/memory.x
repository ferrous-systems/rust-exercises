MEMORY {
  /* Secure Code uses the first flash bank, secure alias */
  /* NB: Do not change this without adjusting -section-start=.gnu.sgstubs in build.rs */
  FLASH (rx): ORIGIN = 0x0C000000, LENGTH = 2M
  /* Secure Code uses SRAM1, secure alias */
  RAM (rwx) : ORIGIN = 0x30000000, LENGTH = 768K
}