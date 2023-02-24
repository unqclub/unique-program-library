import { PublicKey } from '@solana/web3.js';

export interface IDelegation {
  // The creator of the delegation
  master: PublicKey;
  // The wallet who delegates
  representative: PublicKey;
  // Confirmation flag
  authorised: boolean;
}

export interface ITokenAccountInfo {
  numberOfRepresentativeTokenAccounts: number;
  numberOfMasterTokenAccounts?: number;
}
