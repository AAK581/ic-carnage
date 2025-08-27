# `IC Carnage`

Prepare to see carnage!

## Running the project locally

If you want to run the project:

1- Download the game files from here:
https://drive.google.com/file/d/1Ick6y5tykke_Tkbi1e58qSeiU48Gbovf/view?usp=sharing

Then extract this file and copy the 2 folders inside into ic_Carnage/src/ic_carnage_frontend/assets

2- Run the following commands from the project root folder:

```bash
# Starts the replica, running in the background
dfx start --background --clean

# Deploys the Internet Identity canister
dfx deploy internet_identity

# Deploys all canisters
dfx deploy
```

Then the game will be available via the resultant frontend link.