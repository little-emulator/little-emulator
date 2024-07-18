pub mod lc2;

pub trait Architecture {
    type Address;
    type Data;
    type Register;
    type ConditionCode;

    #[must_use]
    fn get_memory(&mut self, address: Self::Address) -> Self::Data;
    fn set_memory(&mut self, address: Self::Address, data: Self::Data);

    #[must_use]
    fn get_register(&self, register: &Self::Register) -> Self::Data;
    fn set_register(&mut self, register: &Self::Register, data: Self::Data);

    #[must_use]
    fn get_condition_code(&self) -> Self::ConditionCode;
    fn set_condition_code(&mut self, condition_code: &Self::ConditionCode);

    fn add_memory_watcher<F>(&mut self, address: Self::Address, function: F)
    where
        F: Fn(Self::Data) + 'static;
    fn remove_memory_watcher(&mut self, address: Self::Address);

    fn add_register_watcher<F>(&mut self, register: &Self::Register, function: F)
    where
        F: Fn(Self::Data) + 'static;
    fn remove_register_watcher(&mut self, register: &Self::Register);

    fn add_condition_code_watcher<F>(&mut self, function: F)
    where
        F: Fn(Self::ConditionCode) + 'static;
    fn remove_condition_code_watcher(&mut self);

    fn step_instruction(&mut self);

    fn interrupt(&mut self, routine_address: Self::Address);
}
