MEMORY
{
  /*
   * Start after the bootloader stub, and stop before the bootloader proper
   * at 0xE0000
   */
  FLASH : ORIGIN = 0x00001000, LENGTH = 0xE0000 - 0x1000
  RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}
