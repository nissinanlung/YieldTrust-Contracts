// =============================================================================
// FILE: src/lib.rs  (Workspace root placeholder)
//
// This file is intentionally empty.
//
// YieldTrust-Contracts is a Cargo workspace (see root Cargo.toml). All
// deployable Soroban WASM contracts live under `contracts/` as workspace
// members:
//
//   contracts/grant_stream      — Core per-second token streaming
//   contracts/vesting_contracts — Cliff + linear vesting schedules
//   contracts/arbitration       — On-chain dispute resolution escrow
//   contracts/compliance        — Regulatory compliance / KYC-AML hooks
//   contracts/zk_kyc            — Zero-knowledge proof verification
//
// The root `src/` directory is NOT part of the workspace and is kept only
// as a workspace-level anchor for tooling that expects a root-level source
// tree.  No contract logic should be placed here.
//
// For integration tests and helper services, see the `tests/` directory.
// =============================================================================