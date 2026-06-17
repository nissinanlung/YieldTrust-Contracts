"use strict";

describe("onChainAdapter configuration validation", () => {
  const OLD_ENV = { ...process.env };

  beforeEach(() => {
    jest.resetModules();
  });

  afterEach(() => {
    process.env = { ...OLD_ENV };
  });

  describe("validateConfig()", () => {
    it("throws when STELLAR_RPC_URL is missing", () => {
      delete process.env.STELLAR_RPC_URL;
      process.env.GRANT_STREAM_CONTRACT_ID = "C-test-id";
      const { validateConfig } = require("../src/adapters/onChainAdapter");
      expect(() => validateConfig()).toThrow(/STELLAR_RPC_URL/);
    });

    it("throws when GRANT_STREAM_CONTRACT_ID is missing", () => {
      process.env.STELLAR_RPC_URL = "https://soroban-testnet.stellar.org";
      delete process.env.GRANT_STREAM_CONTRACT_ID;
      const { validateConfig } = require("../src/adapters/onChainAdapter");
      expect(() => validateConfig()).toThrow(/GRANT_STREAM_CONTRACT_ID/);
    });

    it("throws when both env vars are missing", () => {
      delete process.env.STELLAR_RPC_URL;
      delete process.env.GRANT_STREAM_CONTRACT_ID;
      const { validateConfig } = require("../src/adapters/onChainAdapter");
      expect(() => validateConfig()).toThrow(
        /STELLAR_RPC_URL.*GRANT_STREAM_CONTRACT_ID|GRANT_STREAM_CONTRACT_ID.*STELLAR_RPC_URL/
      );
    });

    it("does not throw when both env vars are set", () => {
      process.env.STELLAR_RPC_URL = "https://soroban-testnet.stellar.org";
      process.env.GRANT_STREAM_CONTRACT_ID = "C-test-id";
      const { validateConfig } = require("../src/adapters/onChainAdapter");
      expect(() => validateConfig()).not.toThrow();
    });
  });
});
