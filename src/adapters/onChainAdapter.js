"use strict";

const { SorobanRpc, Contract, xdr } = require("@stellar/stellar-sdk");

const RPC_URL = process.env.STELLAR_RPC_URL;
const CONTRACT_ID = process.env.GRANT_STREAM_CONTRACT_ID;

let server;
let contract;

function validateConfig() {
  const missing = [];
  if (!RPC_URL) missing.push("STELLAR_RPC_URL");
  if (!CONTRACT_ID) missing.push("GRANT_STREAM_CONTRACT_ID");
  if (missing.length > 0) {
    throw new Error(
      `Missing required environment variable(s): ${missing.join(", ")}. ` +
      "Set these in your .env file or process environment before starting the server."
    );
  }
}

function getServer() {
  if (!server) server = new SorobanRpc.Server(RPC_URL);
  return server;
}

function getContract() {
  if (!contract) contract = new Contract(CONTRACT_ID);
  return contract;
}

const onChainAdapter = {
  async getEscrow(escrowId) {
    if (!CONTRACT_ID) throw new Error("GRANT_STREAM_CONTRACT_ID not set");
    if (!RPC_URL) throw new Error("STELLAR_RPC_URL not set");
    const result = await getServer().simulateInvoke({
      contract: getContract().address(),
      method: "read_escrow",
      args: [xdr.ScVal.scvSymbol(escrowId)],
    });
    return result;
  },

  async getLegalHold(escrowId) {
    if (!CONTRACT_ID) throw new Error("GRANT_STREAM_CONTRACT_ID not set");
    if (!RPC_URL) throw new Error("STELLAR_RPC_URL not set");
    const result = await getServer().simulateInvoke({
      contract: getContract().address(),
      method: "get_legal_hold",
      args: [xdr.ScVal.scvSymbol(escrowId)],
    });
    return result;
  },

  async fundEscrow(escrowId, amount) {
    if (!CONTRACT_ID) throw new Error("GRANT_STREAM_CONTRACT_ID not set");
    if (!RPC_URL) throw new Error("STELLAR_RPC_URL not set");
    const result = await getServer().simulateInvoke({
      contract: getContract().address(),
      method: "fund_escrow",
      args: [xdr.ScVal.scvSymbol(escrowId), xdr.ScVal.scvString(amount)],
    });
    return result;
  },

  async releaseEscrow(escrowId) {
    if (!CONTRACT_ID) throw new Error("GRANT_STREAM_CONTRACT_ID not set");
    if (!RPC_URL) throw new Error("STELLAR_RPC_URL not set");
    const result = await getServer().simulateInvoke({
      contract: getContract().address(),
      method: "release_escrow",
      args: [xdr.ScVal.scvSymbol(escrowId)],
    });
    return result;
  },

  async withdrawFromEscrow(escrowId, amount) {
    if (!CONTRACT_ID) throw new Error("GRANT_STREAM_CONTRACT_ID not set");
    if (!RPC_URL) throw new Error("STELLAR_RPC_URL not set");
    const args = [xdr.ScVal.scvSymbol(escrowId)];
    if (amount) args.push(xdr.ScVal.scvString(amount));
    const result = await getServer().simulateInvoke({
      contract: getContract().address(),
      method: "withdraw_escrow",
      args,
    });
    return result;
  },
};

module.exports = { onChainAdapter, validateConfig };
