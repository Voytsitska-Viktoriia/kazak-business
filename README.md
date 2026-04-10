🧙‍♂️ Kazak Business — Solana Smart Contract Project

 📌 Description

This project is a multi-program decentralized application built on Solana using the Anchor Framework.

It implements a game economy where players can:
* search for resources
* craft items (NFT)
* trade items on a marketplace
* earn MagicToken

The project demonstrates multi-program architecture and Cross-Program Invocation (CPI).

🏗 Architecture

The system consists of 6 on-chain programs:

| Program          | Description                                                    |
| ---------------- | -------------------------------------------------------------- |
| resource_manager | Manages minting and burning of game resources (SPL Token-2022) |
| search           | Allows players to search for resources with cooldown           |
| item_nft         | Handles minting and burning of NFT items                       |
| crafting         | Combines resources into NFTs                                   |
| marketplace      | Allows selling items                                           |
| magic_token      | Mints reward tokens                                            |


🧩 Program IDs

| Program          | Address                                      |
| ---------------- | -------------------------------------------- |
| resource_manager | 5CzubBHrnNHpqmvETAGqRBqkfSpR2jBdpGQGZGTzfG4o |
| search           | BnBincK5DvuXEJ1Weyw5tKGrUQJ372cooYQGuiuYnS1  |
| item_nft         | D3yh8ZJxF6T7Rb3gXdkduUjaiM7kgug9BsFPeqP6W2PG |
| crafting         | 9LWJAeYhzQRhiBEsMdsBg6yKDPJLvcGbjtzr9arFU4sn |
| marketplace      | 6ZsDYbMP5R6o8oAeqU6zwJvdLzTc96bTLvLjiRQNn59u |
| magic_token      | A2gokn42YMW7iFwgVc3sG2QMvJVF2Gmh4Zr1AQcmNYra |

---

    Core Mechanics

🔍 Search
* Player can search resources
* Cooldown stored on-chain (Player PDA)

⚒ Crafting
* Burns resources
* Mints NFT items

🛒 Marketplace
* Allows selling NFT items
* NFT is burned after sale
* Player receives MagicToken

 ✨ MagicToken
* Minted only via Marketplace (CPI)


    🔐 Security Implementation

This project implements core security mechanisms required for Solana programs, including PDA-based state management, authority validation, and controlled access to token operations.

🧩 Program Derived Addresses (PDA)
All critical on-chain state is stored in PDA accounts to ensure deterministic addressing and controlled access.
Implemented PDA accounts include:
* **Player** — tracks cooldown for resource search
* **ItemMetadata** — stores NFT ownership and metadata
* **MarketplaceConfig** — stores marketplace configuration
* **MagicConfig** — controls MagicToken minting authority
These accounts are derived using seeds and program IDs, preventing unauthorized access.

👤 Authority & Ownership Checks
All sensitive operations require signer validation and ownership verification:
* Transactions require a valid signer (`Signer<'info>`)
* PDA accounts store owner fields
* Before any state mutation, ownership is checked
Example logic:
```rust
require!(player.owner == user.key(), CustomError::Unauthorized);
```
This ensures that only the legitimate owner can perform actions such as:
* searching resources
* crafting items
* selling items

🔒 Controlled Minting & Burning
Direct interaction with the Token Program is restricted.
Instead:
* **Resource minting/burning** is handled via `resource_manager`
* **NFT minting/burning** is handled via `item_nft`
* **MagicToken minting** is allowed only through the `marketplace` program
This prevents unauthorized token creation or destruction.

🔗 Cross-Program Invocation (CPI)
Secure interaction between programs is implemented using CPI:
* `crafting` → calls `item_nft` to mint NFTs
* `marketplace` → calls `magic_token` to mint rewards
This ensures:
* separation of responsibilities
* controlled execution flow
* secure inter-program communication

🚫 Access Restrictions
* Direct mint/burn via SPL Token is not allowed
* All token operations must go through program logic
* PDA authority is used to restrict sensitive actions

🧪 Test Coverage
Due to time constraints, full gameplay logic (crafting and marketplace flows) was simplified.
Implemented tests cover:
- PDA initialization
- search cooldown logic
- program availability
- basic instruction calls
Full end-to-end scenarios (NFT crafting, marketplace trading, token minting flow) require additional implementation of business logic and are not fully covered in this submission.

 ⚠️ Notes
Some parts of the logic are simplified to focus on demonstrating:
* PDA usage
* CPI interaction
* access control patterns
However, the overall architecture follows standard Solana security practices.

🗂 Accounts

Player
pub struct Player {
    pub owner: Pubkey,
    pub last_search_timestamp: i64,
    pub bump: u8,
}
`

ItemMetadata
pub struct ItemMetadata {
    pub item_type: u8,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub bump: u8,
}


🚀 Setup & Run

1. Install dependencies - bash yarn install
2. Start local validator - bash solana-test-validator -r
3. Build - bash anchor build
4. Deploy - bash anchor deploy
5. Run tests - bash anchor test

🧪 Testing
Tests are implemented using:
* Anchor test framework
* TypeScript scripts

⚠️ Notes
* Project is configured for **localnet**
* Some warnings may appear during build (expected in Anchor)
* CPI is used between programs

🎯 Features Implemented
* Multi-program architecture
* CPI between programs
* PDA-based state management
* NFT minting logic (simplified)
* Token minting via controlled program

📚 Technologies
* Rust
* Anchor Framework
* Solana
* TypeScript
* SPL Token



🏗 Full Project Structure
kazak-business/
├── Anchor.toml                # Anchor configuration (program IDs, cluster)
├── Cargo.toml                 # Workspace configuration
├── package.json               # JS dependencies
├── tsconfig.json              # TypeScript config

├── migrations/
│   └── deploy.ts              # Deployment script (Anchor)

├── programs/
│   ├── resource_manager/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── instructions/
│   │       │   ├── mod.rs
│   │       │   ├── initialize_game.rs
│   │       │   ├── mint_resource.rs
│   │       │   └── burn_resource.rs
│   │       ├── state.rs
│   │       └── errors.rs
│
│   ├── search/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── instructions/
│   │       │   ├── mod.rs
│   │       │   ├── initialize_player.rs
│   │       │   └── search_resources.rs
│   │       ├── state.rs
│   │       └── errors.rs
│
│   ├── item_nft/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── instructions/
│   │       │   ├── mod.rs
│   │       │   ├── mint_item.rs
│   │       │   └── burn_item.rs
│   │       ├── state.rs
│   │       └── errors.rs
│
│   ├── crafting/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── instructions/
│   │       │   ├── mod.rs
│   │       │   └── craft_item.rs
│   │       ├── state.rs
│   │       └── errors.rs
│
│   ├── magic_token/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── instructions/
│   │       │   ├── mod.rs
│   │       │   ├── initialize_magic_config.rs
│   │       │   └── mint_magic.rs
│   │       ├── state.rs
│   │       └── errors.rs
│
│   └── marketplace/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── instructions/
│           │   ├── mod.rs
│           │   ├── initialize_marketplace.rs
│           │   └── sell_item.rs
│           ├── state.rs
│           └── errors.rs
│
├── target/                    # Build artifacts (auto-generated)
│   └── deploy/
│       ├── *.so
│       └── *-keypair.json
│
├── tests/
│   └── kazak_business.ts      # Integration tests (Anchor)

└── README.md
# kazak-business
