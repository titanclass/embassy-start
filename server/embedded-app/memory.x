/* Note that this has been set up for the smallest board.   */
/* Flash = 1024K and RAM = 256K on the 52840 and 9160.      */
/* However, these settings will work across all our boards. */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 16K
}