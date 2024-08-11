#[cfg(test)]
mod tests;

mod registers;
pub use registers::{Gpr, Register};

use crate::{
    common::{
        ConditionCode, ConditionCodeWatchersStorage, Memory16x16, MemoryWatchersStorage,
        RegisterWatchersStorage,
    },
    Architecture, WatcherType,
};

use std::collections::BTreeMap;
use std::fmt;

#[derive(Default)]
pub struct Lc3 {
    // Registers
    general_purpose_register: [u16; 8],
    program_counter: u16,
    instruction_register: u16,
    processor_status_register: u16,
    saved_usp: u16,
    saved_ssp: u16,

    // Memory
    memory_address_register: u16,
    memory_data_register: u16,
    memory: Memory16x16,

    // Watchers
    register_watchers: RegisterWatchersStorage<Register>,
    memory_watchers: MemoryWatchersStorage<u16>,
    condition_code_watchers: ConditionCodeWatchersStorage,
}

impl Lc3 {
    #[must_use]
    pub fn new(initial_address: u16) -> Self {
        Self {
            program_counter: initial_address,
            processor_status_register: 0x8002,
            saved_ssp: 0x3000,
            saved_usp: 0xfe00,
            ..Default::default()
        }
    }
}

impl fmt::Debug for Lc3 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gpr: &BTreeMap<Option<Gpr>, &u16> = &self
            .general_purpose_register
            .iter()
            .enumerate()
            .map(|(i, value)| (Gpr::try_from(i).ok(), value))
            .collect();

        let condition_code = match self.processor_status_register & 0b111 {
            0b100 => ConditionCode::Negative,
            0b010 => ConditionCode::Zero,
            0b001 => ConditionCode::Positive,
            _ => todo!("Condition Code is not valid"),
        };

        let condition_code_watchers: &Vec<WatcherType> = &self
            .condition_code_watchers
            .iter()
            .enumerate()
            .filter_map(|(i, watcher)| {
                if watcher.is_some() {
                    Some(if i == 0 {
                        WatcherType::OnWrite
                    } else {
                        WatcherType::OnRead
                    })
                } else {
                    None
                }
            })
            .collect();

        fmt.debug_struct("Lc3")
            .field("memory", &self.memory)
            .field("general_purpose_registers", gpr)
            .field("program_counter", &self.program_counter)
            .field("instruction_register", &self.instruction_register)
            .field("processor_status_register", &self.processor_status_register)
            .field("condition_code", &condition_code)
            .field("saved_usp", &self.saved_usp)
            .field("saved_ssp", &self.saved_ssp)
            .field("memory_address_register", &self.memory_address_register)
            .field("memory_data_register", &self.memory_data_register)
            .field("register_watchers", &self.register_watchers.keys())
            .field("memory_watchers", &self.memory_watchers.keys())
            .field("condition_code_watchers", &condition_code_watchers)
            .finish()
    }
}

impl Architecture for Lc3 {
    type Address = u16;
    type Data = u16;
    type Register = Register;
    type RegisterData = u16;
    type ConditionCode = ConditionCode;

    #[must_use]
    fn get_memory(&mut self, address: Self::Address) -> Self::Data {
        // TODO: Implement the Access Control Violation (ACV) exception

        let data = self.memory[address];

        self.memory_address_register = address;
        self.memory_data_register = data;

        // If there is a watcher for this address, call it
        if let Some(function) = self.memory_watchers.get(&(address, WatcherType::OnRead)) {
            function(data);
        }

        data
    }

    fn set_memory(&mut self, address: Self::Address, data: Self::Data) {
        // TODO: Implement the Access Control Violation (ACV) exception

        self.memory_address_register = address;
        self.memory_data_register = data;

        self.memory[address] = data;

        // If there is a watcher for this address, call it
        if let Some(function) = self.memory_watchers.get(&(address, WatcherType::OnWrite)) {
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
    fn get_register(&self, register: &Self::Register) -> Self::RegisterData {
        let data = match register {
            Register::Gpr(gpr) => self.general_purpose_register[u8::from(gpr.clone()) as usize],
            Register::ProgramCounter => self.program_counter,
            Register::InstructionRegister => self.instruction_register,
            Register::ProcessorStatusRegister => self.processor_status_register,
            Register::MemoryAddressRegister => self.memory_address_register,
            Register::MemoryDataRegister => self.memory_data_register,
        };

        // If there is a watcher for this register, call it
        if let Some(function) = self
            .register_watchers
            .get(&(register.clone(), WatcherType::OnRead))
        {
            function(data);
        }

        data
    }

    fn set_register(&mut self, register: &Self::Register, data: Self::RegisterData) {
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

            // Check if the condition code is valid
            Register::ProcessorStatusRegister => {
                if (data & 0b111).count_ones() != 1 {
                    todo!("Condition Code is not valid");
                }

                &mut self.processor_status_register
            }

            Register::MemoryAddressRegister => &mut self.memory_address_register,
            Register::MemoryDataRegister => &mut self.memory_data_register,
        };

        // Update the register
        *register_pointer = data;

        // If there is a watcher for this register, call it
        if let Some(function) = self
            .register_watchers
            .get(&(register.clone(), WatcherType::OnWrite))
        {
            function(data);
        }
    }

    #[must_use]
    fn get_condition_code(&self) -> Self::ConditionCode {
        let condition_code = match self.processor_status_register & 0b111 {
            0b100 => Self::ConditionCode::Negative,
            0b010 => Self::ConditionCode::Zero,
            0b001 => Self::ConditionCode::Positive,
            _ => todo!("Condition Code is not valid"),
        };

        // If there is a watcher for the condition code, call it
        if let Some(function) = &self.condition_code_watchers[1] {
            function(condition_code.clone());
        }

        condition_code
    }

    fn set_condition_code(&mut self, condition_code: &Self::ConditionCode) {
        self.processor_status_register = (self.processor_status_register & 0xfff8)
            | match condition_code {
                Self::ConditionCode::Negative => 0b100,
                Self::ConditionCode::Zero => 0b010,
                Self::ConditionCode::Positive => 0b001,
            };

        // If there is a watcher for the condition code, call it
        if let Some(function) = &self.condition_code_watchers[0] {
            function(condition_code.clone());
        }
    }

    fn add_memory_watcher<F>(
        &mut self,
        address: Self::Address,
        watcher_type: WatcherType,
        function: F,
    ) where
        F: Fn(Self::Data) + 'static,
    {
        self.memory_watchers
            .insert((address, watcher_type), Box::new(function));
    }

    fn remove_memory_watcher(&mut self, address: Self::Address, watcher_type: WatcherType) {
        self.memory_watchers.remove(&(address, watcher_type));
    }

    fn add_register_watcher<F>(
        &mut self,
        register: &Self::Register,
        watcher_type: WatcherType,
        function: F,
    ) where
        F: Fn(Self::RegisterData) + 'static,
    {
        self.register_watchers
            .insert((register.clone(), watcher_type), Box::new(function));
    }

    fn remove_register_watcher(&mut self, register: &Self::Register, watcher_type: WatcherType) {
        self.register_watchers
            .remove(&(register.clone(), watcher_type));
    }

    fn add_condition_code_watcher<F>(&mut self, watcher_type: WatcherType, function: F)
    where
        F: Fn(Self::ConditionCode) + 'static,
    {
        let idx = usize::from(watcher_type != WatcherType::OnWrite);
        self.condition_code_watchers[idx] = Some(Box::new(function));
    }

    fn remove_condition_code_watcher(&mut self, watcher_type: WatcherType) {
        let idx = usize::from(watcher_type != WatcherType::OnWrite);
        self.condition_code_watchers[idx] = None;
    }

    #[allow(clippy::too_many_lines)]
    fn step_instruction(&mut self) {
        todo!()
    }

    fn interrupt(&mut self, data: Self::Data) {
        // Check if the priority of the interrupt is greater of the current
        // program priority
        if (data >> 8) & 0b111 <= (self.processor_status_register >> 8) & 0b111 {
            return;
        }

        // Save the current Process Status Register into a temp variable
        let temp = self.get_register(&Register::ProcessorStatusRegister);

        // Set the privilege mode to "Supervisor" and set the correct priority
        self.set_register(
            &Register::ProcessorStatusRegister,
            (temp & 0x78ff) | (data & 0x0700),
        );

        // Get the stack pointer register
        let register = Register::Gpr(Gpr::R6);

        // If the interrupted process is in "User" privilege mode then save the
        // current stack pointer into the "Saved USP" and load the "Saved SSP"
        if temp >> 15 == 1 {
            self.saved_usp = self.get_register(&register);
            self.set_register(&register, self.saved_ssp);
        }

        // Push the Process Status Register into the stack
        self.set_register(&register, self.get_register(&register).wrapping_sub(1));
        self.set_memory(self.get_register(&register), temp);

        // Push the Program Counter to the stack
        self.set_register(&register, self.get_register(&register).wrapping_sub(1));
        self.set_memory(
            self.get_register(&register),
            self.get_register(&Register::ProgramCounter),
        );

        // Set the Program Counter to the interrupt routine address
        let routine_address = self.get_memory(0x0100 | (data & 0x00ff));
        self.set_register(&Register::ProgramCounter, routine_address);
    }
}
