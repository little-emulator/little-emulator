#[rustfmt::skip]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpr { R0, R1, R2, R3, R4, R5, R6, R7 }

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Register {
    Gpr(Gpr),
    ProgramCounter,
    InstructionRegister,
    ProcessorStatusRegister,
    MemoryAddressRegister,
    MemoryDataRegister,
}

impl TryFrom<usize> for Gpr {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::R0),
            1 => Ok(Self::R1),
            2 => Ok(Self::R2),
            3 => Ok(Self::R3),
            4 => Ok(Self::R4),
            5 => Ok(Self::R5),
            6 => Ok(Self::R6),
            7 => Ok(Self::R7),
            _ => Err("Only numbers between 0 and 7 can be converted into registers!"),
        }
    }
}

impl From<Gpr> for u8 {
    fn from(value: Gpr) -> Self {
        match value {
            Gpr::R0 => 0,
            Gpr::R1 => 1,
            Gpr::R2 => 2,
            Gpr::R3 => 3,
            Gpr::R4 => 4,
            Gpr::R5 => 5,
            Gpr::R6 => 6,
            Gpr::R7 => 7,
        }
    }
}
