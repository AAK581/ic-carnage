use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use ic_stable_structures::storable::Bound;
use ic_cdk::api::call::call;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 5000;
const CRNG_LEDGER_CANISTER: &str = "uxrrr-q7777-77774-qaaaq-cai";

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

    fn into_bytes(self) -> Vec<u8> {
        Encode!(&self).unwrap()
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

// New CRNG token functions
#[ic_cdk::update]
async fn mint_crng_tokens(to_principal: String, amount: u64) -> Result<u64, String> {
    // Only allow minting from this canister (admin function)
    let caller = ic_cdk::caller();
    let this_canister = ic_cdk::id();
    
    if caller != this_canister {
        return Err("Unauthorized: Only this canister can mint tokens".to_string());
    }

    let ledger_id = Principal::from_text(CRNG_LEDGER_CANISTER)
        .map_err(|e| format!("Invalid ledger canister ID: {}", e))?;
    
    let to = Principal::from_text(to_principal)
        .map_err(|e| format!("Invalid recipient principal: {}", e))?;
    
    let mint_args = (
        Account {
            owner: to,
            subaccount: None,
        },
        amount,
    );

    let (result,): (Result<u64, String>,) = call(ledger_id, "icrc1_mint", mint_args)
        .await
        .map_err(|e| format!("Mint call failed: {:?}", e))?;
    
    result.map_err(|e| format!("Mint failed: {}", e))
}

#[ic_cdk::query]
async fn get_crng_balance(user_principal: String) -> Result<u64, String> {
    let ledger_id = Principal::from_text(CRNG_LEDGER_CANISTER)
        .map_err(|e| format!("Invalid ledger canister ID: {}", e))?;
    
    let user = Principal::from_text(user_principal)
        .map_err(|e| format!("Invalid user principal: {}", e))?;
        
    let balance_args = Account {
        owner: user,
        subaccount: None,
    };

    let (balance,): (u64,) = call(ledger_id, "icrc1_balance_of", (balance_args,))
        .await
        .map_err(|e| format!("Balance check failed: {:?}", e))?;
    
    Ok(balance)
}