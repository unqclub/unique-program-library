import { ConfirmOptions, Connection, Keypair, PublicKey } from '@solana/web3.js';
import { AnchorProvider, Wallet } from '@project-serum/anchor/dist/cjs/provider';
import { Buffer } from 'buffer';
import { Program } from '@project-serum/anchor';
import { IDL } from '../idl/udm-idl';
import { emptyWallet } from './utils';

export const SOLANA_PROGRAM_ID = new PublicKey('UPLdquGEBVnVK5TmccSue5gyPkxSRT4poezHShoEzg8');

export const authorizeSeed = Buffer.from('authorize');

export const programFactory = (
  connection: Connection,
  wallet?: Wallet,
  confirmOptions?: ConfirmOptions
) => {
  return new Program(
    IDL,
    SOLANA_PROGRAM_ID,
    new AnchorProvider(
      connection,
      wallet ?? emptyWallet(Keypair.generate().publicKey),
      confirmOptions ?? {}
    )
  );
};
