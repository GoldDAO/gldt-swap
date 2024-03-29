use candid::{ CandidType, Nat, Principal };

use icrc_ledger_types::icrc1::{ account::{ Account, Subaccount }, transfer::{ BlockIndex, Memo } };
use serde::Serialize as Serialize_default;
use serde::Deserialize as Deserialize_default;
use std::collections::{ BTreeMap, btree_map, HashMap };
use serde::ser::{ Serialize, Serializer, SerializeMap };
use serde::de::{ self, Deserialize, Deserializer, MapAccess, Visitor };

use gldt_libs::types::{ NftId, GldtNumTokens, NftWeight };
use crate::records::{ GldtRecord, RecordType, RecordStatusInfo, RecordStatus };
use std::fmt;
use std::marker::PhantomData;

type GldNftCollectionId = Principal;

#[derive(CandidType, Clone, Debug, Hash, Default)]
pub struct Registry {
    registry: BTreeMap<(GldNftCollectionId, NftId), GldtRegistryEntry>,
}

impl Serialize for Registry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map = serializer.serialize_map(Some(self.registry.len()))?;
        for (k, v) in self.registry.clone() {
            map.serialize_entry(&format!("{}|{}", k.0.to_string(), k.1).clone(), &v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Registry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct RegistryVisitor {
            marker: PhantomData<fn() -> Registry>,
        }

        impl<'de> Visitor<'de> for RegistryVisitor {
            type Value = Registry;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "Expecting example : \"obapm-2iaaa-aaaak-qcgca-cai|gold-0-1g\":{ \
                    \"gldt_issue\":{ \
                        \"escrow_subaccount\":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], \
                        \"failed\":null, \
                        \"ledger_entry\":{\"Minted\":{\"block_height\":[],\"num_tokens\":{\"value\":[1410065408,2]}}}, \
                        \"nft_sale_id\":\"test_sale_id\", \
                        \"num_tokens\":{\"value\":[1410065408,2]}, \
                        \"receiving_account\":{\"owner\":\"2vxsx-fae\",\"subaccount\":null}, \
                        \"requested_memo\":[], \
                        \"swap_request_timestamp\":0, \
                        \"swapped\":{\"index\":[100],\"sale_id\":\"test_sale_id\"} \
                    }, \
                    \"gldt_redeem\":null, \
                    \"older_record\":null \
                } "
                )
            }

            fn visit_map<V>(self, mut map: V) -> Result<Registry, V::Error> where V: MapAccess<'de> {
                let mut my_map = BTreeMap::new();
                while let Some((key, value)) = map.next_entry::<String, GldtRegistryEntry>()? {
                    let parts: Vec<&str> = key.splitn(2, '|').collect();

                    if parts.len() != 2 {
                        return Err(
                            de::Error::invalid_value(
                                de::Unexpected::Str(&key),
                                &"a key with format 'GldNftCollectionId|NftId'"
                            )
                        );
                    }

                    let account: GldNftCollectionId = parts[0]
                        .parse::<GldNftCollectionId>()
                        .map_err(de::Error::custom)?;
                    let nft_sale_id: String = parts[1].to_owned();

                    let tuple = (account, nft_sale_id);
                    my_map.insert(tuple, value);
                }
                Ok(Registry { registry: my_map })
            }
        }

        deserializer.deserialize_map(RegistryVisitor { marker: PhantomData })
    }
}

impl Registry {
    #[cfg(test)]
    pub fn get(&self) -> &BTreeMap<(GldNftCollectionId, NftId), GldtRegistryEntry> {
        &self.registry
    }
    pub fn count_number_of_nfts_swapped_per_collection(&self) -> Vec<(GldNftCollectionId, usize)> {
        let mut count_map = HashMap::new();

        for ((collection_id, _), entry) in self.registry.iter() {
            if entry.get_status_of_swap() == SwappingStates::Swapped {
                *count_map.entry(*collection_id).or_insert(0) += 1;
            }
        }

        count_map.into_iter().collect()
    }
}
#[cfg(not(test))]
const MAX_HISTORY_REGISTRY: usize = 64;
#[cfg(not(test))]
const MAX_NUMBER_OF_ENTRIES: usize = 16000;
#[cfg(test)]
pub const MAX_HISTORY_REGISTRY: usize = 8;
#[cfg(test)]
pub const MAX_NUMBER_OF_ENTRIES: usize = 32;

/// Entry into the GLDT registry that keeps track of the NFTs that
/// have been swapped for GLDT.
#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
pub struct GldtRegistryEntry {
    /// The lifecycle of an NFT starts with the issuance of GLDT
    gldt_issue: SwapInfo,
    /// The lifecycle of an NFT ends with the burning of GLDT
    gldt_redeem: Option<SwapInfo>,
    /// Optional reference to a previous minting/burning pair for this
    /// NFT as a historial record.
    older_record: Option<Box<GldtRegistryEntry>>,
}

impl GldtRegistryEntry {
    pub fn new(swap_info: SwapInfo) -> Self {
        Self {
            gldt_issue: SwapInfo::new(
                swap_info.nft_sale_id,
                swap_info.escrow_subaccount,
                swap_info.receiving_account,
                swap_info.swap_request_timestamp,
                swap_info.num_tokens
            ),
            gldt_redeem: None,
            older_record: None,
        }
    }

    pub fn get_status_of_swap(&self) -> SwappingStates {
        if self.is_minted() {
            if self.is_swapped() {
                if self.is_burned() { SwappingStates::Burned } else { SwappingStates::Swapped }
            } else {
                SwappingStates::Minted
            }
        } else {
            SwappingStates::Initialised
        }
    }

    pub fn is_minted(&self) -> bool {
        self.gldt_issue.ledger_entry.is_some()
    }

    pub fn is_swapped(&self) -> bool {
        self.gldt_issue.swapped.is_some()
    }

    pub fn is_burned(&self) -> bool {
        // This logics needs to become more sophisticated once actual burning is implemented.
        self.gldt_redeem.is_some()
    }

    pub fn get_issue_info(&self) -> &SwapInfo {
        &self.gldt_issue
    }

    fn count_older_record(&self) -> usize {
        self.older_record.as_ref().map_or(0, |boxed_entry| 1 + boxed_entry.count_older_record())
    }
}

#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
pub enum UpdateType {
    Init,
    Mint,
    Swap,
    Failed,
    Burn,
}

///
#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
pub struct SwapInfo {
    /// The sale id of the NFT listing in the GLD NFT canister
    nft_sale_id: String,
    /// The escrow account where the GLDT tokens are sent to for the trade.
    escrow_subaccount: Subaccount,
    /// The receiving account for the swap
    receiving_account: Account,
    /// The timestamp when this entry was made
    swap_request_timestamp: u64,
    /// The number of tokens swapped.
    num_tokens: GldtNumTokens,
    /// The requested memo
    requested_memo: Memo,

    /// Filled when tokens are successfully minted or burned.
    ledger_entry: Option<GldtLedgerEntry>,
    /// Filled when NFT has been successfully swapped.
    swapped: Option<GldtSwapped>,
    /// Filled in case of errors during the minting and swapping process
    failed: Option<GldtError>,
}

impl SwapInfo {
    pub fn new(
        nft_sale_id: String,
        escrow_subaccount: Subaccount,
        receiving_account: Account,
        swap_request_timestamp: u64,
        num_tokens: GldtNumTokens
    ) -> Self {
        Self {
            nft_sale_id,
            escrow_subaccount,
            receiving_account,
            swap_request_timestamp,
            num_tokens,
            requested_memo: Memo::default(),
            ledger_entry: None,
            swapped: None,
            failed: None,
        }
    }
    pub fn is_failed(&self) -> bool {
        self.failed.is_some()
    }

    pub fn set_failed(&mut self, error: GldtError) {
        self.failed = Some(error);
    }
    pub fn set_ledger_entry(&mut self, ledger_entry: GldtLedgerEntry) {
        self.ledger_entry = Some(ledger_entry);
    }

    pub fn set_swapped(&mut self, swapped: GldtSwapped) {
        self.swapped = Some(swapped);
    }

    pub fn get_num_tokens(&self) -> GldtNumTokens {
        self.num_tokens.clone()
    }
    pub fn get_nft_sale_id(&self) -> String {
        self.nft_sale_id.clone()
    }
    pub fn get_escrow_subaccount(&self) -> Subaccount {
        self.escrow_subaccount
    }
    pub fn get_swap_request_timestamp(&self) -> u64 {
        self.swap_request_timestamp
    }
    pub fn get_receiving_account(&self) -> Account {
        self.receiving_account
    }
    pub fn get_requested_memo(&self) -> Memo {
        self.requested_memo.clone()
    }
    pub fn get_ledger_entry(&self) -> Option<GldtLedgerEntry> {
        self.ledger_entry.clone()
    }
}

#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
pub enum GldtLedgerEntry {
    Minted(GldtLedgerInfo),
    Burned(GldtLedgerInfo),
}

#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
pub struct Error {
    error_code: Nat,
    error_message: String,
}

#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
pub enum GldtError {
    /// The minting of GLDT failed.
    MintingError(Option<Error>),
    /// The swapping of NFT for GLDT failed.
    SwappingError(Option<Error>),
    /// Extensible error types
    Other(Option<Error>),
}

impl Default for GldtError {
    fn default() -> Self {
        Self::Other(
            Some(Error { error_code: Nat::from(0u8), error_message: "unknown error".to_string() })
        )
    }
}

/// Record of information about an NFT for which GLDT has been minted.
#[derive(
    CandidType,
    Serialize_default,
    Deserialize_default,
    Clone,
    Debug,
    Hash,
    PartialEq,
    Default
)]
pub struct GldtLedgerInfo {
    /// Block height when this entry was made. Must be non-zero and
    /// point to a block with a minting or burning transaction with the right
    /// number of tokens and subaccount.
    block_height: BlockIndex,

    /// The number of tokens that were part of this transaction.
    /// It should alway be 1g : 100 GLDT
    num_tokens: GldtNumTokens,
}

impl GldtLedgerInfo {
    pub fn new(block_height: BlockIndex, num_tokens: GldtNumTokens) -> Self {
        Self {
            block_height,
            num_tokens,
        }
    }
    pub fn get_block_height(&self) -> BlockIndex {
        self.block_height.clone()
    }
}

/// Record of information about an NFT that has been successfully swapped for GLDT.
#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
pub struct GldtSwapped {
    /// Sale ID of the successful sale
    sale_id: String,
    /// Index of the bid
    index: Nat,
}

impl GldtSwapped {
    pub fn new(sale_id: String, index: Nat) -> Self {
        Self {
            sale_id,
            index,
        }
    }
}

#[derive(CandidType, Serialize_default, Deserialize_default, Clone, Debug, Hash, PartialEq)]
pub enum SwappingStates {
    Initialised,
    Minted,
    Swapped,
    Burned,
}

impl Registry {
    pub fn get_entry(&self, key: &(Principal, NftId)) -> Option<&GldtRegistryEntry> {
        self.registry.get(key)
    }
    pub fn init(&mut self, key: &(Principal, NftId), entry: SwapInfo) -> Result<(), String> {
        Self::explicit_sequence_check(&self, key, UpdateType::Init)?;
        let num_entries = self.registry.len();

        match self.registry.entry(key.clone()) {
            btree_map::Entry::Vacant(v) => {
                if num_entries >= MAX_NUMBER_OF_ENTRIES {
                    return Err(
                        format!("Swap NFT limit reached, limit is set to {MAX_NUMBER_OF_ENTRIES}.")
                    );
                }

                v.insert(GldtRegistryEntry::new(entry));
                Ok(())
            }
            btree_map::Entry::Occupied(mut o) => {
                // If there is already an entry when initialising, it may be due to
                // a failed previous swap or because the tokens have been burned.
                // If not, then there may be an attempt to double mint and the
                // procedure is cancelled.
                if o.get().is_burned() || o.get().gldt_issue.is_failed() {
                    if o.get().count_older_record() >= MAX_HISTORY_REGISTRY {
                        return Err(
                            format!(
                                "Swap limit reached for this NFT, limit is set to {MAX_HISTORY_REGISTRY}."
                            )
                        );
                    }

                    o.insert(GldtRegistryEntry {
                        gldt_issue: SwapInfo::new(
                            entry.nft_sale_id,
                            entry.escrow_subaccount,
                            entry.receiving_account,
                            entry.swap_request_timestamp,
                            entry.num_tokens
                        ),
                        gldt_redeem: None,
                        older_record: Some(Box::new(o.get().clone())),
                    });
                    Ok(())
                } else {
                    Err(
                        format!(
                            "There is already an active entry for NFT: {}. Canceling new minting of tokens.",
                            key.1
                        )
                    )
                }
            }
        }
    }

    pub fn update_minted(
        &mut self,
        key: &(Principal, NftId),
        entry: SwapInfo
    ) -> Result<(), String> {
        Self::sanity_check_inputs(self, key, &entry)?;
        Self::explicit_sequence_check(&self, key, UpdateType::Mint)?;
        // check that the input to the function is as expected
        // we are expecting the the ledger_entry is of type Minted
        match entry.ledger_entry.clone() {
            None => {
                return Err(
                    format!(
                        "There is no ledger entry for NFT: {}. Cannot update minting of tokens.",
                        key.1
                    )
                );
            }
            Some(ledger_entry) =>
                match ledger_entry {
                    GldtLedgerEntry::Minted(_) => (), // this is the happy path
                    GldtLedgerEntry::Burned(_) => {
                        return Err(
                            format!(
                                "Burning not implemented yet. There is no valid ledger entry for NFT: {}. Cannot update minting of tokens.",
                                key.1
                            )
                        );
                    }
                }
        }
        match self.registry.entry(key.clone()) {
            btree_map::Entry::Vacant(_) => {
                Err(
                    format!(
                        "There is no active entry for NFT: {}. Cannot update minting of tokens.",
                        key.1
                    )
                )
            }
            btree_map::Entry::Occupied(mut o) => {
                match &o.get().gldt_issue.ledger_entry {
                    Some(_) => {
                        Err(
                            format!(
                                "There is already a ledger entry for NFT: {}. Cannot update minting of tokens.",
                                key.1
                            )
                        )
                    }
                    None => {
                        // This is the happy path when tokens are minted
                        o.get_mut().gldt_issue.ledger_entry = entry.ledger_entry;
                        Ok(())
                    }
                }
            }
        }
    }

    pub fn update_swapped(
        &mut self,
        key: &(Principal, NftId),
        entry: SwapInfo
    ) -> Result<(), String> {
        Self::sanity_check_inputs(self, key, &entry)?;
        Self::explicit_sequence_check(&self, key, UpdateType::Swap)?;
        // check that the input to the function is as expected
        // we are expecting the key `swapped` to be Some
        match entry.swapped.clone() {
            None => {
                return Err(
                    format!(
                        "There is no swap info for NFT: {}. Cannot update swapping of tokens.",
                        key.1
                    )
                );
            }
            Some(_) => (),
        }
        match self.registry.entry(key.clone()) {
            btree_map::Entry::Vacant(_) => {
                Err(
                    format!(
                        "There is no active entry for NFT: {}. Cannot update swapping of tokens.",
                        key.1
                    )
                )
            }
            btree_map::Entry::Occupied(mut o) => {
                match &o.get().gldt_issue.swapped {
                    Some(_) => {
                        Err(
                            format!(
                                "There is already a swap info for NFT: {}. Cannot update swapping of tokens.",
                                key.1
                            )
                        )
                    }
                    None => {
                        // This is the happy path when tokens are swapped
                        o.get_mut().gldt_issue.swapped = entry.swapped;
                        Ok(())
                    }
                }
            }
        }
    }

    pub fn update_failed(
        &mut self,
        key: &(Principal, NftId),
        entry: SwapInfo
    ) -> Result<(), String> {
        Self::explicit_sequence_check(&self, key, UpdateType::Failed)?;
        match self.registry.get_mut(key) {
            Some(r) => {
                r.gldt_issue.set_failed(entry.failed.unwrap_or_default());
            }
            None => {
                return Err(
                    format!(
                        "There is no active entry for NFT: {}. Cannot update failed minting of tokens.",
                        key.1
                    )
                );
            }
        }
        Ok(())
    }

    fn explicit_sequence_check(
        &self,
        key: &(Principal, NftId),
        update_type: UpdateType
    ) -> Result<(), String> {
        // Proper sequence of entries needs to be ensured.
        // Possible sequences currently are
        // 1. Init -> Mint -> Swap
        // 2. Any -> Failed

        match self.registry.get(key) {
            None => {
                if let UpdateType::Init = update_type {
                    // Only init is allowed when there is no entry yet.
                    Ok(())
                } else {
                    Err(
                        format!(
                            "There is no active entry for NFT: {}. Cannot perform sequence check.",
                            key.1
                        )
                    )
                }
            }
            Some(r) => {
                let previous_status = r.get_status_of_swap();
                match update_type {
                    UpdateType::Init => {
                        // Init is further validated in the init function
                        return Ok(());
                    }
                    UpdateType::Mint => {
                        // Mint can only come after Init
                        if previous_status == SwappingStates::Initialised {
                            return Ok(());
                        }
                    }
                    UpdateType::Swap => {
                        // Swap can only come after Mint
                        if previous_status == SwappingStates::Minted {
                            return Ok(());
                        }
                    }
                    UpdateType::Failed => {
                        // Failed needs no validation as it can come from any state
                        return Ok(());
                    }
                    UpdateType::Burn => {
                        return Err("Burning not implemented yet.".to_string());
                    }
                }
                Err(
                    format!(
                        "Invalid sequence of updates for NFT: {}. Previous status was {previous_status:?}, new status was supposed to be {update_type:?}.",
                        key.1
                    )
                )
            }
        }
    }

    fn sanity_check_inputs(
        &self,
        key: &(Principal, NftId),
        entry: &SwapInfo
    ) -> Result<(), String> {
        match self.registry.get(key) {
            None => {
                Err(
                    format!(
                        "There is no active entry for NFT: {}. Cannot perform sanity check.",
                        key.1
                    )
                )
            }
            Some(r) => {
                let gldt_issue = r.gldt_issue.clone();
                let mut problems = Vec::new();

                if gldt_issue.nft_sale_id != entry.nft_sale_id {
                    problems.push(
                        format!(
                            "NFT sale ID - recorded: {:?}, expected: {:?}",
                            gldt_issue.nft_sale_id,
                            entry.nft_sale_id
                        )
                    );
                }
                if gldt_issue.receiving_account != entry.receiving_account {
                    problems.push(
                        format!(
                            "Receiving account - recorded: {:?}, expected: {:?}",
                            gldt_issue.receiving_account,
                            entry.receiving_account
                        )
                    );
                }
                if gldt_issue.num_tokens != entry.num_tokens {
                    problems.push(
                        format!(
                            "Number of tokens - recorded: {:?}, expected: {:?}",
                            gldt_issue.num_tokens,
                            entry.num_tokens
                        )
                    );
                }
                if gldt_issue.requested_memo != entry.requested_memo {
                    problems.push(
                        format!(
                            "memo - recorded: {:?}, expected: {:?}",
                            gldt_issue.requested_memo,
                            entry.requested_memo
                        )
                    );
                }
                if gldt_issue.escrow_subaccount != entry.escrow_subaccount {
                    problems.push(
                        format!(
                            "escrow subaccount - recorded: {:?}, expected: {:?}",
                            gldt_issue.escrow_subaccount,
                            entry.escrow_subaccount
                        )
                    );
                }
                if gldt_issue.swap_request_timestamp != entry.swap_request_timestamp {
                    problems.push(
                        format!(
                            "timestamp - recorded: {}, expected: {}",
                            gldt_issue.swap_request_timestamp,
                            entry.swap_request_timestamp
                        )
                    );
                }
                if !problems.is_empty() {
                    // If there are problems, it is most likely the
                    // case that the response we are handing is
                    // spurious, i.e., not corresponding to the
                    // request made.
                    let msg = format!(
                        "ERROR: ignoring canister response because request state does not match response state: problems {}, record {:?}",
                        problems.join("; "),
                        entry
                    );
                    return Err(msg);
                }
                Ok(())
            }
        }
    }

    pub fn get_ongoing_swaps_by_user(&self, account: Account) -> Vec<GldtRecord> {
        let mut result = Vec::new();
        for ((gld_nft_canister_id, nft_id), entry) in &self.registry {
            if entry.gldt_issue.receiving_account == account {
                if entry.is_swapped() || entry.gldt_issue.is_failed() {
                    continue;
                }
                result.push(
                    GldtRecord::new(
                        RecordType::Mint,
                        entry.gldt_issue.get_swap_request_timestamp(),
                        entry.gldt_issue.get_receiving_account(),
                        *gld_nft_canister_id,
                        (*nft_id).clone(),
                        entry.gldt_issue.get_escrow_subaccount(),
                        entry.gldt_issue.get_nft_sale_id(),
                        0 as NftWeight,
                        entry.gldt_issue.get_num_tokens(),
                        Nat::from(0u8),
                        RecordStatusInfo {
                            status: RecordStatus::Ongoing,
                            message: None,
                        }
                    )
                );
            }
        }
        result
    }
}
