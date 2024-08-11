use super::*;

#[test]
fn get_and_set() {
    let mut cpu = Lc3::new(0x3000);

    cpu.set_condition_code(&ConditionCode::from(!5 + 1));
    assert_eq!(cpu.get_condition_code(), ConditionCode::Negative);

    cpu.set_condition_code(&ConditionCode::from(0));
    assert_eq!(cpu.get_condition_code(), ConditionCode::Zero);

    cpu.set_condition_code(&ConditionCode::from(5));
    assert_eq!(cpu.get_condition_code(), ConditionCode::Positive);
}

// #[test]
// fn set_through_gpr() {
//     todo!()
// }
//
// #[test]
// fn watcher_on_write() {
//     todo!()
// }
//
// #[test]
// fn watcher_on_read() {
//     todo!()
// }
