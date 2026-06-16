# AgriTrust-Contracts

Smart contracts for managing trust streams with milestone completion proof hashing and integrated dispute resolution system on Stellar (Soroban WASM) and Ethereum/L2s (Solidity).

## 🚀 Key Features

- **Per-Second Streaming Accrual:** High-precision streaming logic using scaling factors on Soroban.
- **Legal Anchoring & Escrow:** Restricts fund streaming until legal documents are cryptographically signed on-chain, alongside an integrated arbitration escrow.
- **Multi-Chain Smart Contracts:** Soroban-based smart contract implementation alongside a Foundry/Solidity implementation supporting ZK proof verification.

## 📂 Workspace Structure

This is a Cargo workspace. Each directory under `contracts/` is an independent deployable Soroban WASM package.

| Package             | Path                          | Description                                                                                                                                            |
| ------------------- | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `grant_stream`      | `contracts/grant_stream`      | Core per-second token streaming contract. Handles milestone gating, circuit breakers, clawback, yield integration, dispute resolution, and governance. |
| `vesting_contracts` | `contracts/vesting_contracts` | Cliff and linear vesting schedules for team and investor token allocations.                                                                            |
| `arbitration`       | `contracts/arbitration`       | On-chain arbitration escrow. Supports multi-party dispute adjudication and escrow release logic.                                                       |
| `compliance`        | `contracts/compliance`        | Regulatory compliance screening layer — KYC/AML hooks, sanctions checks, and tax reporting utilities.                                                  |
| `zk_kyc`            | `contracts/zk_kyc`            | Zero-knowledge proof verification for privacy-preserving identity and KYC attestations.                                                                |

All packages share a single `soroban-sdk` version declared in the root `Cargo.toml` under `[workspace.dependencies]`. Individual crates reference it via `soroban-sdk = { workspace = true }`. To upgrade the SDK across the entire workspace, change the version in the root `Cargo.toml` only, then run `cargo update && cargo test --workspace`.

## 🛠️ Tech Stack

- **Language/Framework:** Rust / Soroban WASM, Solidity / Foundry
- **Key Dependencies:** `soroban-sdk`, `foundry-rs`

## 📦 Getting Started

### Prerequisites

Ensure you have the required toolchains installed:

- Rust toolchain (cargo, rustc)
- Stellar CLI / Soroban CLI
- Foundry (forge)

### Installation & Local Setup

```bash
# Clone the repository (if running manually)
git clone https://github.com/AgriTrust-Protocol/AgriTrust-Contracts

# Build Soroban contracts
stellar contract build

# Run cargo tests
cargo test

# Build Solidity contracts
forge build

# Run foundry tests
forge test
```

## 🤝 Contributing

Contributions are highly welcome. Please ensure your commits are cryptographically signed using GPG or SSH keys. For major structural changes, please open an issue first to discuss your proposal.
