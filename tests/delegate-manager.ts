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
    const master = Keypair.generate();
    const representative = Keypair.generate();

    await connection.confirmTransaction(
      await connection.requestAirdrop(master.publicKey, LAMPORTS_PER_SOL)
    );
    await connection.confirmTransaction(
      await connection.requestAirdrop(
        representative.publicKey,
        LAMPORTS_PER_SOL
      )
    );

    const [representation] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("authorize"),
        master.publicKey.toBuffer(),
        representative.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .initializeDelegate()
      .accounts({
        master: master.publicKey,
        representative: representative.publicKey,
        representation,
        systemProgram: SystemProgram.programId,
      })
      .signers([master])
      .rpc();

    assert.deepEqual(
      await program.account.representation.fetch(representation),
      {
        master: master.publicKey,
        representative: representative.publicKey,
        authorised: false,
      }
    );

    await program.methods
      .confirmDelegate()
      .accounts({
        representative: representative.publicKey,
        representation,
        systemProgram: SystemProgram.programId,
      })
      .signers([representative])
      .rpc();

    assert.deepEqual(
      await program.account.representation.fetch(representation),
      {
        master: master.publicKey,
        representative: representative.publicKey,
        authorised: true,
      }
    );

    await program.methods
      .cancelDelegate()
      .accounts({
        representation,
        systemProgram: SystemProgram.programId,
      })
      .remainingAccounts([
        { pubkey: master.publicKey, isSigner: true, isWritable: true },
        {
          pubkey: representative.publicKey,
          isSigner: false,
          isWritable: false,
        },
      ])
      .signers([master])
      .rpc();

    try {
      await program.account.representation.fetch(representation);
      assert(false);
    } catch (error) {
      assert.ok(`${error}`.includes("Account does not exist or has no data"));
    }
  });

  it("Initialize, confirm, cancel by delegate", async () => {
    const master = Keypair.generate();
    const representative = Keypair.generate();

    await connection.confirmTransaction(
      await connection.requestAirdrop(master.publicKey, LAMPORTS_PER_SOL)
    );
    await connection.confirmTransaction(
      await connection.requestAirdrop(
        representative.publicKey,
        LAMPORTS_PER_SOL
      )
    );

    const [representation] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("authorize"),
        master.publicKey.toBuffer(),
        representative.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .initializeDelegate()
      .accounts({
        master: master.publicKey,
        representative: representative.publicKey,
        representation,
        systemProgram: SystemProgram.programId,
      })
      .signers([master])
      .rpc();

    assert.deepEqual(
      await program.account.representation.fetch(representation),
      {
        master: master.publicKey,
        representative: representative.publicKey,
        authorised: false,
      }
    );

    await program.methods
      .confirmDelegate()
      .accounts({
        representative: representative.publicKey,
        representation,
        systemProgram: SystemProgram.programId,
      })
      .signers([representative])
      .rpc();

    assert.deepEqual(
      await program.account.representation.fetch(representation),
      {
        master: master.publicKey,
        representative: representative.publicKey,
        authorised: true,
      }
    );

    await program.methods
      .cancelDelegate()
      .accounts({
        representation,
        systemProgram: SystemProgram.programId,
      })
      .remainingAccounts([
        { pubkey: master.publicKey, isSigner: false, isWritable: true },
        { pubkey: representative.publicKey, isSigner: true, isWritable: false },
      ])
      .signers([representative])
      .rpc();

    try {
      await program.account.representation.fetch(representation);
      assert(false);
    } catch (error) {
      assert.ok(`${error}`.includes("Account does not exist or has no data"));
    }
  });
});
