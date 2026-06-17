/**
 * escrow.js — Express router
 * ──────────────────────────
 * Endpoints:
 *
 *   GET  /escrow/:escrowId          → read escrow state (includes legal_hold)
 *   POST /escrow/:escrowId/fund     → fund escrow (blocked if legal_hold)
 *   POST /escrow/:escrowId/release  → release escrow (blocked if legal_hold)
 *   POST /escrow/:escrowId/withdraw → withdraw from escrow (blocked if legal_hold)
 */

"use strict";

const { Router }                           = require("express");
const { readEscrow }                       = require("../services/escrowRead");
const { fundEscrow, releaseEscrow,
        withdrawFromEscrow }              = require("../services/escrowWrite");
const legalHoldGate                        = require("../middleware/legalHoldGate");

const router = Router({ mergeParams: true });

// GET /escrow/:escrowId
router.get("/:escrowId", async (req, res, next) => {
  try {
    const escrow = await readEscrow(req.params.escrowId);
    return res.status(200).json(escrow);
  } catch (err) {
    if (err.statusCode) {
      return res.status(err.statusCode).json({ error: err.message });
    }
    return next(err);
  }
});

// POST /escrow/:escrowId/fund
router.post("/:escrowId/fund", legalHoldGate, async (req, res, next) => {
  try {
    const { amount } = req.body;
    if (!amount) {
      return res.status(400).json({ error: "Missing amount" });
    }
    const result = await fundEscrow(req.params.escrowId, amount);
    return res.status(200).json(result);
  } catch (err) {
    if (err.statusCode) {
      return res.status(err.statusCode).json({ error: err.message });
    }
    return next(err);
  }
});

// POST /escrow/:escrowId/release
router.post("/:escrowId/release", legalHoldGate, async (req, res, next) => {
  try {
    const result = await releaseEscrow(req.params.escrowId);
    return res.status(200).json(result);
  } catch (err) {
    if (err.statusCode) {
      return res.status(err.statusCode).json({ error: err.message });
    }
    return next(err);
  }
});

// POST /escrow/:escrowId/withdraw
router.post("/:escrowId/withdraw", legalHoldGate, async (req, res, next) => {
  try {
    const { amount } = req.body;
    const result = await withdrawFromEscrow(req.params.escrowId, amount);
    return res.status(200).json(result);
  } catch (err) {
    if (err.statusCode) {
      return res.status(err.statusCode).json({ error: err.message });
    }
    return next(err);
  }
});

module.exports = router;
