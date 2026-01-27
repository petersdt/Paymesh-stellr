use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)] // This is required for Soroban errors
pub enum Error {
    InvalidInput = 1,
    AlreadyExists = 2,
    NotFound = 3,
    InvalidTotalPercentage = 4,
    EmptyMembers = 5,
    DuplicateMember = 6,
    NotAuthorized = 7,
}
