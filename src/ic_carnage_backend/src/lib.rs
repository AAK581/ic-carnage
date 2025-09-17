use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use ic_stable_structures::storable::Bound;
use ic_cdk::api::call::call;
use std::{borrow::Cow, cell::RefCell};
use num_traits::ToPrimitive;
use ic_ledger_types::{AccountIdentifier, Tokens, DEFAULT_SUBACCOUNT};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 5000;
const CRNG_LEDGER_CANISTER: &str = "uxrrr-q7777-77774-qaaaq-cai";
const ICP_LEDGER_CANISTER: &str = "umunu-kh777-77774-qaaca-cai";
const CRNG_PER_ICP: u64 = 400000000; // 4 CRNG per ICP (both use 8 decimals)

#[derive(CandidType, Deserialize)]
struct TransferArgs {
    to: Account,
    amount: candid::Nat,
    fee: Option<candid::Nat>,
    memo: Option<Vec<u8>>,
    from_subaccount: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug)]
enum TransferError {
    BadFee { expected_fee: candid::Nat },
    BadBurn { min_burn_amount: candid::Nat },
    InsufficientFunds { balance: candid::Nat },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: candid::Nat },
    TemporarilyUnavailable,
    GenericError { error_code: candid::Nat, message: String },
}

#[derive(CandidType, Deserialize)]
struct UserBalance {
    principal: String,
    balance: u64,
}

impl Storable for UserBalance {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }


    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

#[derive(CandidType, Deserialize)]
struct Account {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static BALANCE_MAP: RefCell<StableBTreeMap<String, UserBalance, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
    );
}

#[ic_cdk::init]
fn init() {
    // Initialization is handled by stable structures
}

// Existing functions for in-game credits
#[ic_cdk::update]
fn deposit_currency(amount: u64) {
    let caller_principal = ic_cdk::caller().to_string();
    BALANCE_MAP.with(|map| {
        let mut map = map.borrow_mut();
        let balance = map.get(&caller_principal).unwrap_or(UserBalance {
            principal: caller_principal.clone(),
            balance: 0,
        });
        let new_balance = UserBalance {
            principal: caller_principal.clone(),
            balance: balance.balance + amount,
        };
        map.insert(caller_principal.clone(), new_balance);
    });
}

#[ic_cdk::query]
fn get_balance() -> u64 {
    let caller_principal = ic_cdk::caller().to_string();
    BALANCE_MAP.with(|map| {
        map.borrow()
            .get(&caller_principal)
            .map_or(0, |balance| balance.balance)
    })
}

#[ic_cdk::update]
fn reset_balance() {
    let caller_principal = ic_cdk::caller().to_string();
    BALANCE_MAP.with(|map| {
        let mut map = map.borrow_mut();
        map.insert(
            caller_principal.clone(),
            UserBalance {
                principal: caller_principal,
                balance: 0,
            },
        );
    });
}

#[ic_cdk::update]
async fn mint_crng_for_user(user_principal: String, crng_amount: u64) -> Result<u64, String> {
    let ledger_id = Principal::from_text(CRNG_LEDGER_CANISTER)
        .map_err(|e| format!("Invalid ledger canister ID: {}", e))?;
    
    let user = Principal::from_text(user_principal)
        .map_err(|e| format!("Invalid user principal: {}", e))?;

    // Create transfer args (mint by transferring from minting account)
    let transfer_args = TransferArgs {
        to: Account {
            owner: user,
            subaccount: None,
        },
        amount: candid::Nat::from(crng_amount),
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
    };

    // Call the ICRC-1 transfer function (mints tokens)
    let (result,): (Result<candid::Nat, TransferError>,) = call(ledger_id, "icrc1_transfer", (transfer_args,))
        .await
        .map_err(|e| format!("Transfer call failed: {:?}", e))?;

    match result {
        Ok(block_index) => {
            // Convert candid::Nat to u64
            Ok(block_index.0.to_u64().unwrap_or(0))
        },
        Err(e) => Err(format!("Transfer failed: {:?}", e))
    }
}

#[ic_cdk::update]
async fn get_crng_balance(user_principal: String) -> Result<u64, String> {
    let ledger_id = Principal::from_text(CRNG_LEDGER_CANISTER)
        .map_err(|e| format!("Invalid ledger canister ID: {}", e))?;
    
    let user = Principal::from_text(user_principal)
        .map_err(|e| format!("Invalid user principal: {}", e))?;
        
    let balance_args = Account {
        owner: user,
        subaccount: None,
    };

    let (balance,): (candid::Nat,) = call(ledger_id, "icrc1_balance_of", (balance_args,))
        .await
        .map_err(|e| format!("Balance check failed: {:?}", e))?;
    
    balance.0.to_u64().ok_or_else(|| "Balance too large to convert to u64".to_string())
}

// NEW: Customer calls this after sending ICP to purchase CRNG
#[ic_cdk::update] 
async fn purchase_crng_with_icp(expected_icp_amount: u64) -> Result<u64, String> {
    let customer_principal = ic_cdk::caller().to_string();
    
    ic_cdk::println!("Customer {} requesting CRNG for {} ICP", 
        customer_principal, expected_icp_amount);
    
    // Calculate CRNG to mint (1 ICP = 4 CRNG, both use 8 decimals)
    let crng_amount = expected_icp_amount * 4; // 4 CRNG per 1 ICP
    
    // Mint CRNG to customer
    let block_index = mint_crng_for_user(customer_principal, crng_amount).await?;
    
    ic_cdk::println!("Minted {} CRNG units (block {})", crng_amount, block_index);
    
    Ok(crng_amount)
}