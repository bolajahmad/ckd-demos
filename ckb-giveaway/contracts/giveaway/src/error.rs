#[repr(i8)]
pub enum Error {
    InvalidTransition = 1,
    InvalidPrize,
    InvalidStatus,
    InvalidHost,
    InvalidAdmin,
    GiveawayStarted,
    GiveawayEnded,
    WrongWinnerCount,
    AlreadyFinalized,
    InvalidScriptArgs,
    InvalidDataLength,
    Encoding,
}
