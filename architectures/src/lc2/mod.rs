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
        let data = self.memory[address];
        self.memory_data_register = data;
        data
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

    fn step_instruction(&mut self) {
        todo!();
    }

    fn interrupt(&mut self, _routine_address: Self::Address) {
        todo!();
    }
}
