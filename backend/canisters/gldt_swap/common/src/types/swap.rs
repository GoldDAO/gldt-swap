use std::borrow::Cow;

use candid::{ CandidType, Decode, Encode, Nat, Principal };
use canister_time::{ timestamp_millis, HOUR_IN_MS, MINUTE_IN_MS };
use ic_ledger_types::{ AccountIdentifier, TransferError };
use ic_stable_structures::{ storable::Bound, Storable };
use icrc_ledger_types::{
    icrc1::{ account::{ Account, Subaccount }, transfer::TransferError as TransferErrorIcrc },
    icrc2::{ approve::ApproveError, transfer_from::TransferFromError },
};
use serde::{ Deserialize, Serialize };
use tracing::debug;
use types::{ Milliseconds, TimestampMillis };

use crate::{ gldt::{ GldtNumTokens, GLDT_TX_FEE }, nft::NftID };

#[cfg(feature = "inttest")]
pub const MAX_SWAP_INFO_BYTES_SIZE: u32 = 28500;

#[cfg(not(feature = "inttest"))]
pub const MAX_SWAP_INFO_BYTES_SIZE: u32 = 2000;

const MAX_SWAP_TYPE_BYTES_SIZE: u32 = 100;
const MAX_SWAP_ID_BYTES_SIZE: u32 = 100;
pub const STALE_SWAP_TIME_THRESHOLD_MINUTES: u64 = 3;

// ----------------------
//     CRON JOB INTERVALS & Retrys & delays
// ----------------------
pub const MANAGE_GLDT_SUPPLY_INTERVAL: Milliseconds = HOUR_IN_MS * 6;
pub const MANAGE_GLDT_SUPPLY_RETRY_DELAY: Milliseconds = MINUTE_IN_MS * 3;
pub const MANAGE_ARCHIVE_CYCLE_INTERVAL: Milliseconds = MINUTE_IN_MS * 10;
pub const MANAGE_NEW_ARCHIVES_INTERVAL: Milliseconds = MINUTE_IN_MS;
pub const MANAGE_OGY_FEE_ACCOUNTS_INTERVAL: Milliseconds = MINUTE_IN_MS;
pub const MANAGE_SERVICE_STATUS_INTERVAL: Milliseconds = MINUTE_IN_MS;
pub const MANAGE_STALE_SWAPS_INTERVAL: Milliseconds = MINUTE_IN_MS;

// -----------------
//     Shared
// -----------------

pub type SwapIndex = Nat;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SwapId(pub NftID, pub SwapIndex);

impl Storable for SwapId {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_SWAP_ID_BYTES_SIZE,
        is_fixed_size: false,
    };
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapInfo {
    Forward(SwapDetailForward),
    Reverse(SwapDetailReverse),
}

pub fn trace(msg: &str) {
    unsafe {
        ic0::debug_print(msg.as_ptr() as i32, msg.len() as i32);
    }
}

impl SwapInfo {
    pub fn new(swap_type: SwapType) -> Self {
        debug!("//// max swap info size: {MAX_SWAP_INFO_BYTES_SIZE}");
        trace(&format!("//// max swap info size: {MAX_SWAP_INFO_BYTES_SIZE}"));
        match swap_type {
            SwapType::Forward => Self::Forward(SwapDetailForward::default()),
            SwapType::Reverse => Self::Reverse(SwapDetailReverse::default()),
        }
    }

    pub fn get_status(&self) -> SwapStatus {
        match &self {
            SwapInfo::Forward(deets) => SwapStatus::Forward(deets.status.clone()),
            SwapInfo::Reverse(deets) => SwapStatus::Reverse(deets.status.clone()),
        }
    }

    pub fn get_user_principal(&self) -> Principal {
        match &self {
            SwapInfo::Forward(deets) => deets.gldt_receiver.owner,
            SwapInfo::Reverse(deets) => deets.user,
        }
    }

    pub fn get_nft_id(&self) -> NftID {
        match &self {
            SwapInfo::Forward(details) => details.nft_id.clone(),
            SwapInfo::Reverse(details) => details.nft_id.clone(),
        }
    }

    pub fn get_nft_canister(&self) -> Principal {
        match &self {
            SwapInfo::Forward(details) => details.nft_canister.clone(),
            SwapInfo::Reverse(details) => details.nft_canister.clone(),
        }
    }

    pub fn is_swap_over_time_threshold(&self) -> bool {
        let now = timestamp_millis();

        match self {
            // although a swap can be technically stuck in our system. we will never re-process a stuck forward swap
            // because the nft canister will release funds and cancel sale after 1 minute
            SwapInfo::Forward(details) => {
                let threshold =
                    details.created_at + MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES;
                let is_old = now > threshold;

                return is_old;
            }
            SwapInfo::Reverse(details) => {
                let threshold =
                    details.created_at + MINUTE_IN_MS * STALE_SWAP_TIME_THRESHOLD_MINUTES;
                let is_old = now > threshold;

                return is_old;
            }
        }
    }

    pub fn get_swap_id(&self) -> SwapId {
        match &self {
            SwapInfo::Forward(deets) => SwapId(deets.nft_id.clone(), deets.index.clone()),
            SwapInfo::Reverse(deets) => SwapId(deets.nft_id.clone(), deets.index.clone()),
        }
    }
}

impl Storable for SwapInfo {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_SWAP_INFO_BYTES_SIZE,
        is_fixed_size: false,
    };
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SwapType {
    Forward,
    Reverse,
}

impl Storable for SwapType {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_SWAP_TYPE_BYTES_SIZE,
        is_fixed_size: false,
    };
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum BlockFailReason {
    InvalidOperation,
    NotFound,
    QueryRequestFailed,
    ReceiverNotCorrectAccountId(Subaccount),
    SenderNotPrincipalDefaultSubaccount(AccountIdentifier),
    AmountTooSmall,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum BurnFailReason {
    TransferError(TransferError),
    CallError(String),
    TokenBalanceAndSwapRequestDontMatch,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum TransferFailReason {
    TransferFromError(TransferFromError),
    TransferError(TransferErrorIcrc),
    CallError(String),
}

#[derive(Serialize, Deserialize, Debug, CandidType, Clone, PartialEq, Eq)]
pub enum ImpossibleErrorReason {
    PrincipalNotFound,
    AmountNotFound,
    NFTResponseInvalid,
}

// -----------------
//     Forward swap
// -----------------

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct SwapDetailForward {
    pub sale_id: String,
    pub index: Nat,
    pub nft_id: NftID,
    pub nft_id_string: String,
    pub status: SwapStatusForward,
    pub created_at: TimestampMillis,
    pub tokens_to_mint: GldtNumTokens,
    pub escrow_sub_account: Subaccount,
    pub gldt_receiver: Account,
    pub nft_canister: Principal,
}

impl Default for SwapDetailForward {
    fn default() -> Self {
        Self {
            sale_id: Default::default(),
            index: SwapIndex::default(),
            nft_id: Default::default(),
            nft_id_string: String::default(),
            status: SwapStatusForward::Init,
            created_at: Default::default(),
            tokens_to_mint: Default::default(),
            escrow_sub_account: Default::default(),
            gldt_receiver: Account {
                owner: Principal::anonymous(),
                subaccount: None,
            },
            nft_canister: Principal::anonymous(),
        }
    }
}

impl SwapDetailForward {
    pub fn update_escrow_account(&mut self, subaccount: Subaccount) {
        self.escrow_sub_account = subaccount;
    }

    pub fn update_sale_id(&mut self, sale_id: String) {
        self.sale_id = sale_id;
    }
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapStatusForward {
    Init,
    NotificationInProgress,
    NotificationFailed(NotificationError),
    MintRequest,
    MintInProgress,
    MintFailed(MintError),
    BidRequest,
    BidInProgress,
    BidFail(BidFailError),
    BurnFeesRequest,
    BurnFeesInProgress,
    BurnFeesFailed(BurnFeesError),
    DepositRecoveryRequest(Box<SwapStatusForward>),
    DepositRecoveryInProgress(Box<SwapStatusForward>),
    DepositRecoveryFailed(Box<SwapStatusForward>, DepositRecoveryError),
    Complete,
    Failed(SwapErrorForward),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum MintError {
    TransferFailed(TransferFailReason),
    UnexpectedError(ImpossibleErrorReason),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum BurnFeesError {
    TransferFailed(TransferFailReason),
    UnexpectedError(ImpossibleErrorReason),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum BidFailError {
    TransferFailed(String),
    CallError(String),
    UnexpectedError(String),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum DepositRecoveryError {
    CantRecover(String),
    CallError(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapErrorForward {
    NotificationFailed(NotificationError),
    MintFailed(MintError),
    BidFailed(BidFailError),
    UnexpectedError(ImpossibleErrorReason),
    DepositRecoveryFailed(DepositRecoveryError),
    Expired(Box<SwapStatusForward>),
}

#[derive(Serialize, Deserialize, Debug, CandidType, Clone, PartialEq, Eq)]
pub enum NotificationError {
    OrigynStringIdDoesNotMatch(String),
    CollectionDoesNotMatch(String),
    SellerAndReceiverDoesNotMatch(String),
    InvalidEscrowSubaccount(String),
    InvalidTokenSpec,
    InvalidTokenAmount,
    InvalidSaleSubaccount,
    SellerIsNotPrincipalOrAccount(String),
    TooManyPrincipalsInAllowList,
    AllowListDoesNotContainCorrectPrincipal,
    InvalidCustomAskFeature,
    InvalidPricingConfig,
    TimeoutInvalid(String),
    SaleIDStringTooLong(String),
}
// -----------------
//     Reverse swap
// -----------------

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct SwapDetailReverse {
    pub index: Nat,
    pub nft_id: NftID,
    pub nft_id_string: String,
    pub nft_canister: Principal,
    pub status: SwapStatusReverse,
    pub created_at: TimestampMillis,
    pub tokens_to_receive: GldtNumTokens,
    pub swap_fee: Nat,
    pub transfer_fees: Nat,
    pub user: Principal,
}

impl Default for SwapDetailReverse {
    fn default() -> Self {
        Self {
            index: SwapIndex::default(),
            nft_id: Default::default(),
            nft_id_string: String::default(),
            nft_canister: Principal::anonymous(),
            status: SwapStatusReverse::Init,
            created_at: Default::default(),
            tokens_to_receive: GldtNumTokens::default(),
            swap_fee: Nat::from(100_000_000u64),
            user: Principal::anonymous(),
            transfer_fees: Nat::from(GLDT_TX_FEE * 2),
        }
    }
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapStatusReverse {
    Init,
    EscrowRequest,
    EscrowRequestInProgress,
    EscrowFailed(EscrowError),
    NftTransferRequest,
    NftTransferRequestInProgress,
    NftTransferFailed(NftTransferError),
    RefundRequest,
    RefundRequestInProgress,
    RefundFailed(RefundError),
    BurnRequest,
    BurnRequestInProgress,
    BurnFailed(BurnError),
    FeeTransferRequest,
    FeeTransferRequestInProgress,
    FeeTransferFailed(FeeTransferError),
    Complete,
    Failed(SwapErrorReverse),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapErrorReverse {
    NftValidationFailed(Vec<NftValidationError>),
    LockFailed(LockError),
    EscrowFailed(EscrowError),
    NftTransferFailed(NftTransferError),
    BurnFailed(BurnError),
    FeeTransferFailed(FeeTransferError),
    Refunded(Box<SwapStatusReverse>),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum FeeTransferError {
    TransferError(TransferErrorIcrc),
    CallError(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum EscrowError {
    ApproveError(ApproveError),
    RequestFailed(String),
    TransferFailed(TransferFailReason),
    UnexpectedError(ImpossibleErrorReason),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum LockError {
    NftAlreadyLocked(Vec<NftID>),
    NftNotLocked,
    UnexpectedError(),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum NftValidationError {
    InvalidNftWeight,
    WeightParseError,
    CanisterInvalid,
    InvalidGldtTokensFromWeight,
    CantGetOrigynID(String),
    NotOwnedBySwapCanister,
    CantVerifySwapCanisterOwnsNft,
    NftIdStringTooLong(String),
    UserDoesNotHaveTheRequiredGLDT(String),
    CantValidateUserBalanceOfGLDT(String),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum NftTransferError {
    InvalidFee(String),
    ApprovalError(ApproveError),
    ApprovalCallError(String),
    TransferFailed(String),
    UnexpectedError(ImpossibleErrorReason),
    FailedToGetOgyFeeAllowance(String),
    CallError(String),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum BurnError {
    CallError(String),
}
#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum RefundError {
    TransferFailed(TransferErrorIcrc),
    CallError(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum SwapStatus {
    Forward(SwapStatusForward),
    Reverse(SwapStatusReverse),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServiceStatus {
    Up,
    Down(ServiceDownReason),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServiceDownReason {
    Initializing,
    ArchiveRelated(ArchiveDownReason),
    ActiveSwapCapacityFull,
    LowOrigynToken(String),
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArchiveStatus {
    Up,
    Down(ArchiveDownReason),
    Upgrading,
    Initializing,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArchiveDownReason {
    NewArchiveError(NewArchiveError), //
    Upgrading, //
    UpgradingArchivesFailed(String), //
    ActiveSwapCapacityFull,
    NoArchiveCanisters(String), //
    LowOrigynToken(String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum NewArchiveError {
    FailedToSerializeInitArgs(String),
    CreateCanisterError(String),
    InstallCodeError(String),
    CantFindControllers(String),
}
