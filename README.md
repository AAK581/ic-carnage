# `IC Carnage`

Prepare to see carnage!

## Mainnet URLs
  Frontend canister via browser:
    ic_carnage_frontend: https://gbbo5-mqaaa-aaaai-atlva-cai.icp0.io/
  Backend canister via Candid interface:
    ic_carnage_backend: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=hmpkt-dyaaa-aaaai-atlsq-cai
    icrc1_ledger_canister: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=hfmbp-vqaaa-aaaai-atlta-cai
    nft_skins: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=gicfb-2yaaa-aaaai-atluq-cai

## Running the project locally

If you want to run the project:

Run the following commands from the project root folder:

```bash
# Starts the replica, running in the background
dfx start --background --clean

# Deploys the Internet Identity canister
dfx deploy internet_identity

# Deploying the ICRC-1 token canister
dfx deploy icrc1_ledger_canister --network ic --argument "(variant {Init = record { token_symbol = \"CRNG\"; token_name = \"Carnage Token\"; minting_account = record { owner = principal \"$(dfx canister id ic_carnage_backend --network ic)\" }; transfer_fee = 1000000; metadata = vec {}; feature_flags = opt record{icrc2 = true}; initial_balances = vec {}; archive_options = record { num_blocks_to_archive = 2000; trigger_threshold = 1000; controller_id = principal \"$(dfx identity get-principal)\"; cycles_for_archive_creation = opt 10000000000000; }; } })"

# Deploying the ICRC-7 canister
dfx deploy nft_skins --network ic --argument '(record {
    icrc7_args = opt record {
        symbol = opt "CRNG_SKINS";
        name = opt "IC Carnage Skins";
        description = opt "A Collection of IC Carnage Skin NFTs";
        logo = opt "https://drive.google.com/file/d/1GlViaF87xKvG6FCMvcBrtF12HLbj_rDw/view?usp=drive_link";
        supply_cap = null;
        allow_transfers = null;
        max_query_batch_size = opt 100;
        max_update_batch_size = opt 100;
        default_take_value = opt 1000;
        max_take_value = opt 10000;
        max_memo_size = opt 512;
        permitted_drift = null;
        tx_window = null;
        burn_account = null;
        deployer = principal "<YOUR PRINCIPAL>";
        supported_standards = null;
    };
    icrc37_args = null;
    icrc3_args = null;
})'

# Deploying frontend and backend canisters
dfx deploy
```

Then the game will be available via the resultant frontend link.