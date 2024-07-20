#[cfg(test)]
mod tests;

mod condition_code;
mod memory;
mod registers;

pub use condition_code::ConditionCode;
use memory::Memory;
pub use registers::{Gpr, Register};

use std::collections::BTreeMap;
use std::fmt;

#[derive(Default)]
pub struct Lc2 {
    // Registers
    general_purpose_register: [u16; 8],
    condition_code: ConditionCode,
    program_counter: u16,
    instruction_register: u16,

    // Memory
    memory_address_register: u16,
    memory_data_register: u16,
    memory: Memory,

    // Watchers
    register_watchers: BTreeMap<Register, Box<dyn Fn(u16)>>,
    memory_watchers: BTreeMap<u16, Box<dyn Fn(u16)>>,
    condition_code_watcher: Option<Box<dyn Fn(ConditionCode)>>,
}

impl Lc2 {
    #[must_use]
    pub fn new(initial_address: u16) -> Self {
        Self {
            program_counter: initial_address,
            ..Default::default()
        }
    }
}

impl fmt::Debug for Lc2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Lc2")
            .field("memory", &self.memory)
            .field("general_purpose_registers", {
                &self
                    .general_purpose_register
                    .iter()
                    .enumerate()
                    .map(|(i, value)| (Gpr::try_from(i).ok(), value))
                    .collect::<BTreeMap<_, _>>()
            })
            .field("condition_code", &self.condition_code)
            .field("program_counter", &self.program_counter)
            .field("instruction_register", &self.instruction_register)
            .field("memory_address_register", &self.memory_address_register)
            .field("memory_data_register", &self.memory_data_register)
            .field("register_watchers", &self.register_watchers.keys())
            .field("memory_watchers", &self.memory_watchers.keys())
            .field(
                "condition_code_watcher",
                &self.condition_code_watcher.is_some(),
            )
            .finish()
    }
}

impl crate::Architecture for Lc2 {
    type Address = u16;
    type Data = u16;
    type Register = Register;
    type ConditionCode = ConditionCode;

    #[must_use]
    fn get_memory(&mut self, address: Self::Address) -> Self::Data {
        self.memory_address_register = address;
        self.memory_data_register = self.memory[address];

        self.memory_data_register
    }

    fn set_memory(&mut self, address: Self::Address, data: Self::Data) {
        self.memory_address_register = address;
        self.memory_data_register = data;

        self.memory[address] = data;

        // If there is a watcher for this address, call it
        if let Some(function) = self.memory_watchers.get(&address) {
            function(data);
        }
    }

    fn load_bytes(
        &mut self,
        start_address: Self::Address,
        bytes: &[u8],
    ) -> Result<(), &'static str> {
        // Calculate the address of the last cell
        let data_size: usize = std::mem::size_of::<Self::Data>();
        let end_address: usize = start_address as usize
            + (bytes.len() / data_size)
            + usize::from(bytes.len() % data_size != 0);

        // Return an error if the byte array is too big
        if end_address > Self::Address::MAX as usize {
            return Err("The array of byte is too big");
        }

        // Save the Memory Address Register and the Memory Data Register
        let memory_address_register = self.get_register(&Register::MemoryAddressRegister);
        let memory_data_register = self.get_register(&Register::MemoryDataRegister);

        // Convert the list of bytes into a list of address-data tuple and save
        // them to memory
        bytes
            .chunks(2)
            .enumerate()
            .map(
                |(i, bytes): (usize, &[u8])| -> (Self::Address, Self::Data) {
                    // Put the high byte into the data word
                    let mut data = u16::from(bytes[0]) << 8;

                    // Put the low byte into the data word, if it exists
                    if bytes.len() == 2 {
                        data |= u16::from(bytes[1]);
                    }

                    // Return the address-data tuple
                    #[allow(clippy::cast_possible_truncation)]
                    (i as Self::Address + start_address, data)
                },
            )
            .for_each(|(address, data)| self.set_memory(address, data));

        // Restore the Memory Address Register and the Memory Data Register
        self.set_register(&Register::MemoryAddressRegister, memory_address_register);
        self.set_register(&Register::MemoryDataRegister, memory_data_register);

        Ok(())
    }

    #[must_use]
    fn get_register(&self, register: &Self::Register) -> Self::Data {
        match register {
            Register::Gpr(gpr) => self.general_purpose_register[u8::from(gpr.clone()) as usize],
            Register::ProgramCounter => self.program_counter,
            Register::InstructionRegister => self.instruction_register,
            Register::MemoryAddressRegister => self.memory_address_register,
            Register::MemoryDataRegister => self.memory_data_register,
        }
    }

    fn set_register(&mut self, register: &Self::Register, data: Self::Data) {
        // Get a mutable pointer to the register
        let register_pointer: &mut u16 = match register {
            Register::Gpr(gpr) => {
                // When the register to update is a General Purpose Register
                // update the condition code
                self.set_condition_code(&ConditionCode::from(data));

                &mut self.general_purpose_register[u8::from(gpr.clone()) as usize]
            }
            Register::ProgramCounter => &mut self.program_counter,
            Register::InstructionRegister => &mut self.instruction_register,
            Register::MemoryAddressRegister => &mut self.memory_address_register,
            Register::MemoryDataRegister => &mut self.memory_data_register,
        };

        // Update the register
        *register_pointer = data;

        // If there is a watcher for this register, call it
        if let Some(function) = self.register_watchers.get(register) {
            function(data);
        }
    }

    #[must_use]
    fn get_condition_code(&self) -> Self::ConditionCode {
        self.condition_code.clone()
    }

    fn set_condition_code(&mut self, condition_code: &Self::ConditionCode) {
        self.condition_code = condition_code.clone();

        // If there is a watcher for the collection code, call it
        if let Some(function) = &self.condition_code_watcher {
            function(condition_code.clone());
        }
    }

    fn add_memory_watcher<F>(&mut self, address: Self::Address, function: F)
    where
        F: Fn(Self::Data) + 'static,
    {
        self.memory_watchers.insert(address, Box::new(function));
    }

    fn remove_memory_watcher(&mut self, address: Self::Address) {
        self.memory_watchers.remove(&address);
    }

    fn add_register_watcher<F>(&mut self, register: &Self::Register, function: F)
    where
        F: Fn(Self::Data) + 'static,
    {
        self.register_watchers
            .insert(register.clone(), Box::new(function));
    }

    fn remove_register_watcher(&mut self, register: &Self::Register) {
        self.register_watchers.remove(register);
    }

    fn add_condition_code_watcher<F>(&mut self, function: F)
    where
        F: Fn(Self::ConditionCode) + 'static,
    {
        self.condition_code_watcher = Some(Box::new(function));
    }

    fn remove_condition_code_watcher(&mut self) {
        self.condition_code_watcher = None;
    }

    #[allow(clippy::too_many_lines)]
    fn step_instruction(&mut self) {
        // Get the next instruction
        let instruction = self.get_memory(self.get_register(&Register::ProgramCounter));

        // Update the Instruction Register
        self.set_register(&Register::InstructionRegister, instruction);

        // Increment the Program Counter
        self.set_register(
            &Register::ProgramCounter,
            self.get_register(&Register::ProgramCounter).wrapping_add(1),
        );

        // Match the operation
        let opcode = instruction >> 12;
        match opcode {
            // Add and And, both with register and immediate
            0b0001 | 0b0101 => {
                // Get the destination register and the first source register
                // content
                let dest: Register = reg_from_instr(instruction, 9);
                let src1: u16 = self.get_register(&reg_from_instr(instruction, 6));

                // If the 6th bit is a 0 then do the operation on a register
                let src2: u16 = if (instruction >> 5) & 1 == 0 {
                    self.get_register(&reg_from_instr(instruction, 0))
                }
                // Else the operation is to be done on a immediate value
                else {
                    let mut src2 = instruction & 0b11111;

                    // Sign-extend the immediate value
                    if (src2 >> 4) & 1 == 1 {
                        src2 |= 0xffe0;
                    }

                    src2
                };

                self.set_register(
                    &dest,
                    // Decide if the operation is an ADD or an AND
                    if (opcode) == 0b0001 {
                        src1.wrapping_add(src2)
                    } else {
                        src1 & src2
                    },
                );
            }

            // Branch (BR)
            0b0000 => {
                // TODO: Refactor
                if (((instruction >> 11) & 1) == 1
                    && (self.condition_code == ConditionCode::Negative))
                    || (((instruction >> 10) & 1) == 1
                        && (self.condition_code == ConditionCode::Zero))
                    || (((instruction >> 9) & 1) == 1
                        && (self.condition_code == ConditionCode::Positive))
                {
                    // Get the new address by combining the current Program
                    // Counter with the page offset
                    let address = (self.get_register(&Register::ProgramCounter) & 0xfe00)
                        + (instruction & 0x01ff);

                    // Set the Program Counter to the new address
                    self.set_register(&Register::ProgramCounter, address);
                }
            }

            // Jump (JMP) and Jump to Subroutine (JSR), both immediate and
            // through registers
            0b0100 | 0b1100 => {
                // If the 12th bit is set then save the current Program Counter
                // into R7
                if (instruction >> 11) & 1 == 1 {
                    self.set_register(
                        &Register::Gpr(Gpr::R7),
                        self.get_register(&Register::ProgramCounter),
                    );
                }

                // If the instruction is a Jump (or Jump to Subroutine), get the
                // new address by combining the current Program Counter with the
                // page offset
                let address = if opcode == 0b0100 {
                    (self.get_register(&Register::ProgramCounter) & 0xfe00) + (instruction & 0x01ff)
                }
                // If the instruction is a Jump (or Jump to Subroutine) through
                // Register, get the content of the register and add the last 6
                // bits of the instruction
                else {
                    self.get_register(&reg_from_instr(instruction, 6)) + (instruction & 0x003f)
                };

                // Set the Program Counter to the new address
                self.set_register(&Register::ProgramCounter, address);
            }

            // Memory Operations:
            //   - Load (LD)
            //   - Load Indirect (LDI)
            //   - Load through Register (LDR)
            //   - Load Effective Address (LEA)
            //   - Store (ST)
            //   - Store Indirect (STI)
            //   - Store through Register (STR)
            0b0010 | 0b1010 | 0b0110 | 0b1110 | 0b0011 | 0b1011 | 0b0111 => {
                // Get the source/destination register
                let register = reg_from_instr(instruction, 9);

                // If the operation is an operation "through Register", get
                // address by combining the content of the register and add the
                // last 6 bits of the instruction
                let address = if opcode >> 1 == 0b011 {
                    self.get_register(&reg_from_instr(instruction, 6)) + (instruction & 0x003f)
                }
                // Else get the address of the data by combining the current
                // Program Counter with the page offset
                else {
                    let address = (self.get_register(&Register::ProgramCounter) & 0xfe00)
                        + (instruction & 0x01ff);

                    // If the operation is an indirect memory operation, use the
                    // address to get the real address from memory
                    if opcode >> 1 == 0b101 {
                        self.get_memory(address)
                    } else {
                        address
                    }
                };

                // If the operation is a Store operation, save the source
                // register into the memory address
                if opcode & 1 == 1 {
                    self.set_memory(address, self.get_register(&register));
                }
                // Else if the operation is a Load operation, save the memory
                // cell pointed by the address into the destination register
                else {
                    // If the operation is a Load Effective Address, the data is
                    // the address itself
                    let data = if opcode == 0b1110 {
                        address
                    } else {
                        self.get_memory(address)
                    };

                    // Put the data in the destination register
                    self.set_register(&register, data);
                }
            }

            // Not
            0b1001 => {
                self.set_register(
                    &reg_from_instr(instruction, 9),
                    !self.get_register(&reg_from_instr(instruction, 6)),
                );
            }

            // Return (RET)
            0b1101 => {
                // Set the Program Counter to the value saved in R7
                self.set_register(
                    &Register::ProgramCounter,
                    self.get_register(&Register::Gpr(Gpr::R7)),
                );
            }

            // Return from Interrupt (RTI)
            0b1000 => {
                // Get the stack pointer register
                let register = Register::Gpr(Gpr::R6);

                // Get the Condition Code from the stack
                let condition_code = self.get_memory(self.get_register(&register));
                self.set_register(&register, self.get_register(&register).wrapping_sub(1));

                // Get the return address from the stack
                let address = self.get_memory(self.get_register(&register));
                self.set_register(&register, self.get_register(&register).wrapping_sub(1));

                // Set the Condition Code to the value popped from the stack
                self.set_condition_code(&ConditionCode::from(condition_code));

                // Set the Program Counter to the value popped from the stack
                self.set_register(&Register::ProgramCounter, address);
            }

            // Trap
            0b1111 => {
                // Save the Program Counter into R7
                self.set_register(
                    &Register::Gpr(Gpr::R7),
                    self.get_register(&Register::ProgramCounter),
                );

                // Load into the Program Counter the address pointed by the trap
                // vector
                let address = self.get_memory(instruction & 0x00ff);
                self.set_register(&Register::ProgramCounter, address);
            }

            0b10000..=u16::MAX => unreachable!(),
        }
    }

    fn interrupt(&mut self, routine_address: Self::Address) {
        // Save the Condition Code as a u16
        let condition_code = u16::from(self.get_condition_code());

        // Get the stack pointer register
        let register = Register::Gpr(Gpr::R6);

        // Push the Program Counter to the stack
        self.set_register(&register, self.get_register(&register).wrapping_add(1));
        self.set_memory(
            self.get_register(&register),
            self.get_register(&Register::ProgramCounter),
        );

        // Push the Condition Code on the stack
        self.set_register(&register, self.get_register(&register).wrapping_add(1));
        self.set_memory(self.get_register(&register), condition_code);

        // Set the Program Counter to the interrupt routine address
        self.set_register(&Register::ProgramCounter, routine_address);
    }
}

fn reg_from_instr(instruction: u16, offset: u8) -> Register {
    Register::Gpr(
        Gpr::try_from((instruction >> offset) as usize & 0b111)
            .expect("Any number & 0b111 should be smaller than 8"),
    )
}
