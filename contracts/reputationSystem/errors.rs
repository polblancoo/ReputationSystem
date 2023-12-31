use scale::{Decode, Encode};

/// Possible erroneous results
#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    NotAdmin,
    VoterNotExist,
    CannotVoteItself,
    YouAreNotVoter,
    YouAreVoter,
    IncorrectRoll,
    NotVoterError,
    NftNotSent,
    InvalidTransfer,
    RoundInCourse,
    RoundFinish,
    NotRound,
    PosicionNoValida,
    FaltanGanadores,
    NotTransferredBalance,
    NotPricesFunds,
    TransferError,
    TransferError2
}