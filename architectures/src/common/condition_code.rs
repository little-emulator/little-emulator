#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConditionCode {
    Negative,
    #[default]
    Zero,
    Positive,
}

impl From<u16> for ConditionCode {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Zero,
            1..=0x7fff => Self::Positive,
            0x8000..=0xffff => Self::Negative,
        }
    }
}

impl From<ConditionCode> for u16 {
    fn from(condition_code: ConditionCode) -> Self {
        match condition_code {
            ConditionCode::Negative => 0x8000,
            ConditionCode::Zero => 0,
            ConditionCode::Positive => 0x0001,
        }
    }
}
