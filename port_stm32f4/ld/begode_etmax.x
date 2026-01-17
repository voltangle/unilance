/* STM32F405RG memory layout */
/* Generated with ChatGPT because fuck you */

MEMORY
{
/* FIXME: when the bootloader is implemented, adjust this one so it matches its layout */
  FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 1024K

  /* Main SRAM */
  SRAM1 (rwx) : ORIGIN = 0x20000000, LENGTH = 112K
  SRAM2 (rwx) : ORIGIN = 0x2001C000, LENGTH = 16K

  /* Core Coupled Memory (no DMA access) */
  CCM   (rwx) : ORIGIN = 0x10000000, LENGTH = 64K
}

/* Default RAM region used by Rust/C runtime */
REGION_ALIAS("RAM", SRAM1);

