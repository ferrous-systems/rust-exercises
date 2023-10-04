MEMORY
{
  /* Bootloader is split in 2 parts: the first part lives in the range 
     0..0x1000; the second part lives at the end of the 1 MB Flash. The range
     selected here collides with neither */ 
  FLASH : ORIGIN = 0x1000, LENGTH = 0x7f000

  /* The bootloader uses the first 8 bytes of RAM to preserve state so don't
     touch them */
  RAM   : ORIGIN = 0x20000008, LENGTH = 0x3fff8
}
