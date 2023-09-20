//Specifically, we are going to emulate the Western Design Center's W65C02S 8-bit microprocessor.
pub struct W65C02S {
  /*
  Instruction Register (IR) and Decode:
  8-bit
  The Operation Code portion of the instruction is loaded into the Instruction Register from the
  Data Bus and is latched during the OpCode fetch cycle. The OpCode is then decoded, along with
  timing and interrupt signals, to generate various control signals for program execution
  */
  IR: u8,

  /*
  Accumulator Register
  8-bit
  The Accumulator Register is an 8-bit general purpose register which holds one of the operands
  and the result of arithmetic and logical operations. Reconfigured versions of this processor family
  could have additional accumulators
  */
  A: u8,

  /*
  Index Registers X&Y
  There are two 8-bit index registers, x and y, which  may be used as general purpose registers or
  to provide an index value for calculation of the effective address. When executing an instruction
  with indexed addressing, the microprocessor fetches the OpCode and the base address, and then modifies
  the address by adding the Index Register contents to the address prior to performing the desired operation.
  */

  X: u8,
  Y: u8,


  /*
  Program Counter Register
  16-bit
  The 16-bit program counter register provides the addresses which are used to step the microprocessor through sequential
  program instructions. This register is incremented each time an instructions or operand is fetched from program memory.
  */
  PC: u16,

  /*
  Stack Pointer Register
  8-bit
  The Stack Pointer Register is an 8-bit register which is used to indicate the next available location in the stack memory
  area. It serves as the effective address in stack addressing modes as well as subroutine and interrupt processing.
  */
  S: u8,

  /*
  Processor Status Register
  8-bit
  The 8-bit Processor Status Register contains status flags and mode select bits. The Carry, Negative, Overflow,
  and Zero status flags serve to report the status of ALU operations. These status flags are tested with conditional
  branch instructions. The Decimal and IRQB disable are used as mode select flags. These flags are set by the program
  to change Microprocessor operations. Bit 5 is available for a user status or mode bit.
  */
  //0
  P_C: bool,
  //1
  P_Z: bool,
  //2
  P_I: bool,
  //3
  P_D: bool,
  //4
  P_B: bool,
  //5
  P_Ignore: bool,
  //6
  P_V: bool,
  //7
  P_N: bool,

  /*
  I/O FUNCTIONS
  */

  /*
  Address Bus A0-A15
  16-bit
  The sixteen bit address bus formed by A0-A15, address memory and I/O registers that exchange data on the Data Bus.
  The address lines can be set to the high impedance state by the Bus Enable (BE) signal.
  */
  A_BUS: [bool;16],

  /*
  Bus Enable Pin
  1-bit
  The Bus Enable (BE) input signal provides external control of the Address, Data, and the RWB buffers. When Bus Enable
  is high, the Address, Data, and RWB buffers are active. When BE is low, these buffers are set to the high impedance status.
  Bus enable is an asynchronous signal.
  */
  BE: bool,

  /*
  Data Bus D0-D7
  8-bit
  The 8 data bus lines Do-D7 are used to provide instructions, data, and addresses to the microprocessor and exchange data
  with memory and I/O registers. These lines may be set to the high impedance state by the Bus Enable Signal.
  */
  D_BUS: [bool;8],

  /*
  Interrupt Request (IRQB)
  1-bit
  The Interrupt Request (IRQB) input signal is used to request that an interrupt sequence be initiated. The program counter
  (PC) and Processor Status Register (P) are pushed onto the stack and the IRQB disable (I) flag is set to a "1", disabling
  further interrupts before jumping to the interrupt handler. These values are used to return the processor to its original
  state prior to the IRQB interrupt. The IRQB low level should be held until the interrupt handler clears the interrupt request
  source. When Return from Interrupt (RTI) is executed the (I) flag is restored and a new interrupt can be handled.
  If the (I) flag is cleared in an interrupt handler, nested interrupts can occur. The Wait-For-Interrupt (WAI) instruction
  may be used to reduce power and synchronize with, as an example timer interrupt requests.
  */
  IRQB: bool,

  /*
  Memory Lock
  1-bit
  The Memory Lock (MLB) output may be used to ensure the integrity of Read-Modify-Write instructions in a multiprocessor
  system. Memory lock indicates the need to defer arbitration of the bus cycle when MLB is low. Memory lock is low during
  the last three cycles of ASL, DEC, INC, LSR, ROL, ROR, TRB, and TSB memory referencing instructions.
  */
  MLB: bool,

  /*
  Non-Maskable Interrupt (NMIB)
  1-bit
  A negative transition of the non-maskable interrupt (NMIB) input initiates an interrupt sequence after the current instruction
  is completed. Since NMIB is an edge-sensitive input, an interrupt will occur if there is a negative transition while servicing
  a previous interrupt. Also, after the edge interrupt occurs no further interrupts will occur if NMIB remains low. The NMIB
  signal going low causes the Progrma Counter (PC) and Processor Status Register information to be pushed onto the stack before
  jumping to the interrupt handler. These values are used to return the processor to its original state prior to the NMIB interrupt.
  */
  NMIB: bool,

  /*
  Phase 2 In (PHI2), Phase 2 Out (PHI2O) and Phase 1 Out (PHI1O)
  Phase 2 In (PHI2), is the system clock input to the microprocessor internal clock. During the low power standby mode, PHI2, can
  be held in either high or low state to preserve the contents of internal registers since the microprocessor is a fully static
  design. The Phase 2 Out (PHI2O) signal is generated from PHI2. Phase 1 Out (PHI1O) is the inverted PHI2 signal. An external oscillator
  is recommended for driving PHI2 and used for the main system clock. All production test timing is based on PHI2. PHI2O and PHI1O were
  used in older systems for system timing and internal oscillators when an external crystal was used.
  */
  PHI2: bool,
  PHI2O: bool,
  PHI1O: bool,

  /*
  Read/Write (RWB)
  1-bit
  The Read/Write (RWB) output signal is used to control data transfer. When in the high state, the microprocessor is reading data from
  memory or I/O. When in the low state, the Data Bus contains valid data to be written from the microprocessor and stored at the addressed
  memory or I/O location. The RWB signal is set to the high impedance state when Bus Enable (BE) is low.
  */
  RWB: bool,

  /*
  Ready (RDY)
  1-bit
  A low input logic level on the Ready (RDY) will halt the microprocessor in its current state. Returning RDY to the high state allows
  the microprocessor to continue operation following the next PHI2 negative transition. This bi-directional signal allows the user to
  single-cycle the microprocessor on all cycles including write cycles. A negative transition to the low state prior to the falling edge of
  PHI2 will halt the microprocessor with the output address lines reflecting the current address being fetched. The assumes the processor setup
  time is met. This condition will remain through a subsequent PHI2 in which the ready signal is low. This feature allows microprocessor interfacing
  with low-speed memory as well as direct memory access (DMA). The WAI instruction pulls RDY low, signaling the WAit-for-Interrupt condition
  , the RDY is a bi-directional pin.On the W65C02 hard core there is a WAIT otput signal that can be used in ASIC's thus removing the big-directional
  signal and RDY becomes only the input. In such a situation the WAI instructions will pull WAIT low and must be used external of the core to
  pull RDY low or the processor will continue as if the WAI never happened. The microprocessor will be released when RDY is high and a falling
  edge of PHI2 occurs. This again assumes the processor control setup time is met. The RDY pin no longer has an active pull up. It is suggested
  that a pull up resister be used on this pin when not being used. The RDY pin can still be wire ORed.
  */
  RDY: bool,

  /*
  Reset (RESB)
  1-bit
  The Reset (RESB) input is used to initialize the microprocessor and start program execution. The RESB signal must be held
  low for at least two clock cycles after VDD reaches operating voltage. Ready (RDY) has no effect while RESB is being held low.
  All registers are initialized by software except the Decimal and Interrupt disable mode select bits of the processor status register (P)
  are initialized by hardware. When a positive edge is detected, there will be a reset sequence lasting seven clock cycles. The program
  counter is loaded with the reset vector from locations FFFC (low byte) and FFFD (high byte). This is the start location for program control.
  RESB shouuld be held high after reset for normal operation.
  */
  RESB: bool,

  /*
  Set Overflow (SOB)
  1-bit
  A negative transition on the Set Overflow (SOB) pin sets the overflow bit (V) in the status code register. The signal is sampled on
  the rising edge of PHI2. SOB was originally intended for fast input recognition because it can be tested with a branch instruction; however
  , it is not recommended in new system design and was seldom used in the past.
  */
  SOB: bool,

  /*
  SYNChronize with OpCode fetch (SYNC)
  1-bit
  The OpCode fetch cycle of the microprocessor instruction is indicated with SYNC high. The SYNC output is provided to identify those cycles
  during which the microprocessor is fetching an OpCode. The SYNC line goes high during the clock cycle of an OpCode fetch and stays high
  for the entire cycle. If the RDY line is pulled low during the clock cycle in which SYNC went high, the processor will stop in its current
  state and will remain in the state until the RDY line goes high. In this manner, the SYNC signal can be used to control RDY to cause single
  instruction execution.
  */
  SYNC: bool,

  /*
  Power (VDD) and Ground (VSS)
  VDD is the positive power voltage and VSS is system logic ground
  */
  VDD:bool,
  VSS:bool,

  /*
  Vector Pull (VPB)
  1-bit
  The Vector Pull (VPB) output indicates that a vector location is being addressed during an interrupt sequence. VPB is low during
  the last interrupt sequence cycles, during which time the processor reads the interrupt vector. The VPB signal may be used to
  select and prioritize interrupts from several sources by modifying the vector addresses.
  */
  VPB: bool,

  /*INTERNALS*/
  memory: RAM
}

impl W65C02S {
  pub fn new(mems: RAM) -> W65C02S{
    W65C02S {
      memory: mems,
      IR: 0,
      A: 0,
      X: 0,
      Y: 0,
      PC: 0,
      P_C: false,
      P_Z: false,
      P_I: false,
      P_D: false,
      P_B: false,
      P_Ignore: true,
      P_V: false,
      P_N: false,
      S: 0,
      A_BUS: [false;16],
      BE: false,
      D_BUS: [false;8],
      IRQB: false,
      MLB: false,
      NMIB: false,
      PHI2: false,
      PHI2O: false,
      PHI1O: false,
      RWB: false,
      RDY: false,
      RESB: false,
      SOB: false,
      SYNC: false,
      VDD: false,
      VSS: false,
      VPB: false
    }
  }
}

pub struct RAM {
  pub address: [u8; 65536]
}

impl RAM {
  pub fn new() -> RAM {
    RAM{address: [0;65536]}
  }
}
