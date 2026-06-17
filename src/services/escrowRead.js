"use strict";

const { onChainAdapter } = require("../adapters/onChainAdapter");

// ─── Validation ───────────────────────────────────────────────────────────────

const ESCROW_ID_RE = /^[a-zA-Z0-9_-]{1,64}$/;
const ADDRESS_RE = /^0x[a-fA-F0-9]{40}$/;
const BALANCE_RE = /^[0-9]+$/;

/**
 * Validate an escrow ID string.
 * @param {string} id
 * @throws {Error} with `statusCode = 400` if invalid.
 */
function validateEscrowId(id) {
  if (typeof id !== "string" || !ESCROW_ID_RE.test(id)) {
    const err = new Error("Invalid escrow ID");
    err.statusCode = 400;
    throw err;
  }
}

/**
 * Validate the raw payload shape from the on-chain adapter.
 * @param {object} raw
 * @throws {Error} with `statusCode = 502` if malformed.
 */
function validateRawPayload(raw) {
  if (!raw || typeof raw !== "object") {
    const err = new Error("Escrow not found");
    err.statusCode = 404;
    throw err;
  }

  // Validate `balance` — must be a non-negative numeric string when present
  if (raw.balance !== undefined && raw.balance !== null) {
    if (typeof raw.balance !== "string" || !BALANCE_RE.test(raw.balance)) {
      const err = new Error("Malformed escrow balance: expected numeric string");
      err.statusCode = 502;
      throw err;
    }
  }

  // Validate `recipient` — must be a valid address when present
  if (raw.recipient !== undefined && raw.recipient !== null) {
    if (typeof raw.recipient !== "string" || !ADDRESS_RE.test(raw.recipient)) {
      const err = new Error("Malformed escrow recipient: expected valid address");
      err.statusCode = 502;
      throw err;
    }
  }

  // Validate `status` — must be a non-empty string when present
  if (raw.status !== undefined && raw.status !== null) {
    if (typeof raw.status !== "string" || raw.status.length === 0) {
      const err = new Error("Malformed escrow status: expected non-empty string");
      err.statusCode = 502;
      throw err;
    }
  }

  // Validate `legal_hold` — must be a boolean when present
  if (raw.legal_hold !== undefined && raw.legal_hold !== null) {
    if (typeof raw.legal_hold !== "boolean") {
      const err = new Error("Malformed legal_hold: expected boolean");
      err.statusCode = 502;
      throw err;
    }
  }
}

// ─── Core read ────────────────────────────────────────────────────────────────

/**
 * Fetch and normalise escrow state for `escrowId`.
 *
 * @param {string} escrowId
 * @returns {Promise<{
 *   escrow_id:  string,
 *   balance:    string,
 *   recipient:  string,
 *   status:     string,
 *   legal_hold: boolean
 * }>}
 */
async function readEscrow(escrowId) {
  validateEscrowId(escrowId);

  let raw;
  try {
    raw = await onChainAdapter.getEscrow(escrowId);
  } catch (err) {
    // Re-throw validation errors as-is; wrap everything else
    if (err.statusCode) throw err;
    const wrapped = new Error("Failed to fetch escrow data");
    wrapped.statusCode = 503;
    throw wrapped;
  }

  validateRawPayload(raw);
  return normalise(escrowId, raw);
}

// ─── Normalisation ────────────────────────────────────────────────────────────

/**
 * Map raw on-chain data to the canonical escrow shape.
 *
 * `legal_hold` defaults to `true` (safe-fail) when the field is **intentionally
 * absent** from a valid contract payload.  If the field is present but malformed
 * (not a boolean), `validateRawPayload` will have already thrown — so this
 * function only handles the "field missing from valid payload" case.
 *
 * @param {string} escrowId
 * @param {object} raw — pre-validated payload
 * @returns {object}
 */
function normalise(escrowId, raw) {
  const legalHold =
    typeof raw.legal_hold === "boolean"
      ? raw.legal_hold
      : raw.legalHold === true
        ? true
        : raw.legal_hold === false || raw.legalHold === false
          ? false
          : true; // safe default: treat unknown as held

  return {
    escrow_id:  escrowId,
    balance:    String(raw.balance   ?? "0"),
    recipient:  String(raw.recipient ?? ""),
    status:     String(raw.status    ?? "unknown"),
    legal_hold: legalHold,
  };
}

// ─── Exports ──────────────────────────────────────────────────────────────────

module.exports = { readEscrow, validateEscrowId, validateRawPayload, normalise };