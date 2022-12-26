import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { assert } from "chai";
import { DelegateManager } from "../target/types/delegate_manager";

describe("delegate-manager", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DelegateManager as Program<DelegateManager>;
  const connection = anchor.getProvider().connection;

  it("Initialize, confirm, cancel by authority", async () => {
    const authority = Keypair.generate();
    const delegate = Keypair.generate();

    await connection.confirmTransaction(
      await connection.requestAirdrop(authority.publicKey, LAMPORTS_PER_SOL)
    );
    await connection.confirmTransaction(
      await connection.requestAirdrop(delegate.publicKey, LAMPORTS_PER_SOL)
    );

    const [delegation] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("authorize"),
        authority.publicKey.toBuffer(),
        delegate.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .initializeDelegate()
      .accounts({
        authority: authority.publicKey,
        delegator: delegate.publicKey,
        delegation,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    assert.deepEqual(await program.account.delegation.fetch(delegation), {
      authority: authority.publicKey,
      delegator: delegate.publicKey,
      authorised: false,
    });

    await program.methods
      .confirmDelegate()
      .accounts({
        delegator: delegate.publicKey,
        delegation,
        systemProgram: SystemProgram.programId,
      })
      .signers([delegate])
      .rpc();

    assert.deepEqual(await program.account.delegation.fetch(delegation), {
      authority: authority.publicKey,
      delegator: delegate.publicKey,
      authorised: true,
    });

    await program.methods
      .cancelDelegate()
      .accounts({ delegation, systemProgram: SystemProgram.programId })
      .remainingAccounts([
        { pubkey: authority.publicKey, isSigner: true, isWritable: true },
        { pubkey: delegate.publicKey, isSigner: false, isWritable: false },
      ])
      .signers([authority])
      .rpc();

    try {
      await program.account.delegation.fetch(delegation);
      assert(false);
    } catch (error) {
      assert.ok(`${error}`.includes("Account does not exist or has no data"));
    }
  });

  it("Initialize, confirm, cancel by delegate", async () => {
    const authority = Keypair.generate();
    const delegate = Keypair.generate();

    await connection.confirmTransaction(
      await connection.requestAirdrop(authority.publicKey, LAMPORTS_PER_SOL)
    );
    await connection.confirmTransaction(
      await connection.requestAirdrop(delegate.publicKey, LAMPORTS_PER_SOL)
    );

    const [delegation] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("authorize"),
        authority.publicKey.toBuffer(),
        delegate.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .initializeDelegate()
      .accounts({
        authority: authority.publicKey,
        delegator: delegate.publicKey,
        delegation,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    assert.deepEqual(await program.account.delegation.fetch(delegation), {
      authority: authority.publicKey,
      delegator: delegate.publicKey,
      authorised: false,
    });

    await program.methods
      .confirmDelegate()
      .accounts({
        delegator: delegate.publicKey,
        delegation,
        systemProgram: SystemProgram.programId,
      })
      .signers([delegate])
      .rpc();

    assert.deepEqual(await program.account.delegation.fetch(delegation), {
      authority: authority.publicKey,
      delegator: delegate.publicKey,
      authorised: true,
    });

    await program.methods
      .cancelDelegate()
      .accounts({ delegation, systemProgram: SystemProgram.programId })
      .remainingAccounts([
        { pubkey: authority.publicKey, isSigner: false, isWritable: true },
        { pubkey: delegate.publicKey, isSigner: true, isWritable: false },
      ])
      .signers([delegate])
      .rpc();

    try {
      await program.account.delegation.fetch(delegation);
      assert(false);
    } catch (error) {
      assert.ok(`${error}`.includes("Account does not exist or has no data"));
    }
  });
});
