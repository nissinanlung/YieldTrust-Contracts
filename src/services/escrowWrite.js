"use strict";

const { onChainAdapter } = require("../adapters/onChainAdapter");
const { validateEscrowId } = require("./escrowRead");

async function fundEscrow(escrowId, amount) {
  validateEscrowId(escrowId);
  if (typeof amount !== "string" || amount.length === 0) {
    const err = new Error("Invalid amount");
    err.statusCode = 400;
    throw err;
  }
  try {
    await onChainAdapter.fundEscrow(escrowId, amount);
  } catch (err) {
    if (err.statusCode) throw err;
    const wrapped = new Error("Failed to initiate funding");
    wrapped.statusCode = 502;
    throw wrapped;
  }
  return {
    status:    "pending",
    message:   "Funding initiated",
    escrow_id: escrowId,
    amount,
  };
}

async function releaseEscrow(escrowId) {
  validateEscrowId(escrowId);
  try {
    await onChainAdapter.releaseEscrow(escrowId);
  } catch (err) {
    if (err.statusCode) throw err;
    const wrapped = new Error("Failed to initiate release");
    wrapped.statusCode = 502;
    throw wrapped;
  }
  return {
    status:    "pending",
    message:   "Release initiated",
    escrow_id: escrowId,
  };
}

async function withdrawFromEscrow(escrowId, amount) {
  validateEscrowId(escrowId);
  if (amount !== undefined && (typeof amount !== "string" || amount.length === 0)) {
    const err = new Error("Invalid amount");
    err.statusCode = 400;
    throw err;
  }
  try {
    await onChainAdapter.withdrawFromEscrow(escrowId, amount);
  } catch (err) {
    if (err.statusCode) throw err;
    const wrapped = new Error("Failed to initiate withdrawal");
    wrapped.statusCode = 502;
    throw wrapped;
  }
  return {
    status:    "pending",
    message:   "Withdrawal initiated",
    escrow_id: escrowId,
    ...(amount !== undefined && { amount }),
  };
}

module.exports = { fundEscrow, releaseEscrow, withdrawFromEscrow };
