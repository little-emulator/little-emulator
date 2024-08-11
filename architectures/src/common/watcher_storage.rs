use crate::{common::ConditionCode, WatcherType};
use std::collections::BTreeMap;

pub type RegisterWatchersStorage<T> = BTreeMap<(T, WatcherType), Box<dyn Fn(u16)>>;
pub type MemoryWatchersStorage<T> = BTreeMap<(T, WatcherType), Box<dyn Fn(u16)>>;

// 0 => WatcherType::OnWrite,
// 1 => WatcherType::OnRead,
pub type ConditionCodeWatchersStorage = [Option<Box<dyn Fn(ConditionCode)>>; 2];
