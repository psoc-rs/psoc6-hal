# pyOCD configuration
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

target remote :3333
load
stepi
