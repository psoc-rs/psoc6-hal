/* Temporary memory placement */

MEMORY
{
  FLASH : ORIGIN = 0x10000000, LENGTH = 256K
  RAM :   ORIGIN = 0x08002000, LENGTH = 32K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
