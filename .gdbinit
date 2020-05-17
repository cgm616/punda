target remote :3333

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break UserHardFault
break rust_begin_unwind

# print demangled symbols by default
set print asm-demangle on

monitor arm semihosting enable

load
stepi
