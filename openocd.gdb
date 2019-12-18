# Example GDB configuration
#
# A gdbserver can be launched with the following command:
#
# ```
# pyocd gdbserver -S -t cy8c6xx7_nosmif --persist
# ```
#
# Ports to note:
# - 3333 -> M0+
# - 3334 -> M4

target extended-remote :3333

set print asm-demangle on

set backtrace limit 32

break DefaultHandler
break HardFault
break rust_begin_unwind
break main

monitor arm semihosting enable

load
stepi
