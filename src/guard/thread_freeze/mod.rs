use minhook_detours_sys::{
    MH_FREEZE_METHOD_FAST_UNDOCUMENTED, MH_FREEZE_METHOD_NONE_UNSAFE, MH_FREEZE_METHOD_ORIGINAL,
    MH_THREAD_FREEZE_METHOD,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ThreadFreezeMethod {
    /// Documentation at [SlimDetours](https://github.com/KNSoft/KNSoft.SlimDetours/blob/d5c4dddd85d67b961ca79bd11cc90f25313bc1b5/Source/SlimDetours/Transaction.c#L43) [[implementation](https://github.com/KNSoft/KNSoft.SlimDetours/blob/d5c4dddd85d67b961ca79bd11cc90f25313bc1b5/Source/SlimDetours/Thread.c#L189)]. Skips current thread.
    Original,
    /// When beginning a SlimDetours transaction, threads won't be frozen.
    None,
}

impl From<MH_THREAD_FREEZE_METHOD> for ThreadFreezeMethod {
    fn from(value: MH_THREAD_FREEZE_METHOD) -> Self {
        match value {
            MH_FREEZE_METHOD_ORIGINAL => Self::Original,
            MH_FREEZE_METHOD_FAST_UNDOCUMENTED => Self::Original,
            MH_FREEZE_METHOD_NONE_UNSAFE => Self::None,
            _ => unreachable!(),
        }
    }
}

impl Into<MH_THREAD_FREEZE_METHOD> for ThreadFreezeMethod {
    fn into(self) -> MH_THREAD_FREEZE_METHOD {
        match self {
            Self::Original => MH_FREEZE_METHOD_ORIGINAL,
            Self::None => MH_FREEZE_METHOD_NONE_UNSAFE,
        }
    }
}
