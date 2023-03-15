import type { PublicKey, Transaction } from '@solana/web3.js';
import { Wallet } from '@project-serum/anchor/dist/cjs/provider';

export const emptyWallet = (publicKey: PublicKey): Wallet => ({
  signTransaction: async (tx: Transaction) => new Promise(() => tx),
  signAllTransactions: async (txs: Transaction[]) => new Promise(() => txs),
  publicKey: publicKey,
});
