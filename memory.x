MEMORY
{
  /* ROM and RAM are mandatory memory regions */
  ROM (RX)  : ORIGIN = 0x00000000, LENGTH = 256K
  RAM (RWX) : ORIGIN = 0x20000000, LENGTH = 64K

  /* More memory regions can declared: for example this is a second RAM region */
  /* CCRAM : ORIGIN = 0x10000000, LENGTH = 8K */
}