pub mod lc2;
pub mod lc3;

pub mod common;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum WatcherType {
    OnRead,
    OnWrite,
}

pub trait Architecture {
    type Address;
    type Data;
    type Register;
    type RegisterData;
    type ConditionCode;

    #[must_use]
    fn get_memory(&mut self, address: Self::Address) -> Self::Data;
    fn set_memory(&mut self, address: Self::Address, data: Self::Data);

    /// # Errors
    ///
    /// This method will return an `Err` if an attempt is made to insert a
    /// quantity of bytes exceeding `Self::Address`
    fn load_bytes(
        &mut self,
        start_address: Self::Address,
        bytes: &[u8],
    ) -> Result<(), &'static str>;

    #[must_use]
    fn get_register(&self, register: &Self::Register) -> Self::RegisterData;
    fn set_register(&mut self, register: &Self::Register, data: Self::RegisterData);

    #[must_use]
    fn get_condition_code(&self) -> Self::ConditionCode;
    fn set_condition_code(&mut self, condition_code: &Self::ConditionCode);

    fn add_memory_watcher<F>(
        &mut self,
        address: Self::Address,
        watcher_type: WatcherType,
        function: F,
    ) where
        F: Fn(Self::Data) + 'static;
    fn remove_memory_watcher(&mut self, address: Self::Address, watcher_type: WatcherType);

    fn add_register_watcher<F>(
        &mut self,
        register: &Self::Register,
        watcher_type: WatcherType,
        function: F,
    ) where
        F: Fn(Self::RegisterData) + 'static;
    fn remove_register_watcher(&mut self, register: &Self::Register, watcher_type: WatcherType);

    fn add_condition_code_watcher<F>(&mut self, watcher_type: WatcherType, function: F)
    where
        F: Fn(Self::ConditionCode) + 'static;
    fn remove_condition_code_watcher(&mut self, watcher_type: WatcherType);

    fn step_instruction(&mut self);

    fn interrupt(&mut self, routine_address: Self::Address);
}
