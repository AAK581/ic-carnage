use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use ic_stable_structures::storable::Bound;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 5000;

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