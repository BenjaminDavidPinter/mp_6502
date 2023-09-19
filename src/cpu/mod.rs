/*

Step 1;
    Mock the registers in the CPU.

There are 6 Registers on the 6502;

# Program Counter [16-bit]
  The program counter is a 16 bit register which points to the next instruction to be executed.
The value of program counter is modified automatically as instructions are executed.
  The value of the program counter can be modified by executing a jump, a relative branch or
a subroutine call to another memory address or by returning from a subroutine or interrupt.

# Stack Pointer [8-bit]
  The processor supports a 256 byte stack located between $0100 and $01FF. The stack pointer
is an 8 bit register and holds the low 8 bits of the next free location on the stack. The
location of the stack is fixed and cannot be moved.
  Pushing bytes to the stack causes the stack pointer to be decremented. Conversely pulling
bytes causes it to be incremented.
  The CPU does not detect if the stack is overflowed by excessive pushing or pulling operations
and will most likely result in the program crashing.

# Accumulator [8-bit]
  The 8 bit accumulator is used all arithmetic and logical operations (with the exception of
increments and decrements). The contents of the accumulator can be stored and retrieved either
from memory or the stack.
  Most complex operations will need to use the accumulator for arithmetic and efficient optimisation
of its use is a key feature of time critical routines.

# Index Register x/y [8-bit]
  The 8 bit index register is most commonly used to hold counters or offsets for accessing memory.
The value of the X register can be loaded and saved in memory, compared with values held in memory
or incremented and decremented.
  The X register has one special function. It can be used to get a copy of the stack pointer or
change its value.

# Processor Status Flag [8-bit]
*/

//Specifically, we are going to emulate the Western Design Center's W65C02S 8-bit microprocessor.
pub struct W65C02S {
  program_counter: u8,
  stack_pointer: u8,
  accumulator: u8,
  index_register_x: u8,
  index_register_y: u8,
  processor_status:u8
}

#[cfg(test)]
pub mod W65C02S_Tests{
  use Super::*;
}