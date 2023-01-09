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
import { Example } from "../target/types/example";

describe("delegation-manager", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DelegateManager as Program<DelegateManager>;
  const example = anchor.workspace.Example as Program<Example>;
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

    const [delegation] = PublicKey.findProgramAddressSync(
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
        delegation,
        systemProgram: SystemProgram.programId,
      })
      .signers([master])
      .rpc();

    assert.deepEqual(await program.account.delegation.fetch(delegation), {
      master: master.publicKey,
      representative: representative.publicKey,
      authorised: false,
    });

    await program.methods
      .confirmDelegate()
      .accounts({
        representative: representative.publicKey,
        delegation,
        systemProgram: SystemProgram.programId,
      })
      .signers([representative])
      .rpc();

    assert.deepEqual(await program.account.delegation.fetch(delegation), {
      master: master.publicKey,
      representative: representative.publicKey,
      authorised: true,
    });

    await program.methods
      .cancelDelegate()
      .accounts({
        delegation,
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
      await program.account.delegation.fetch(delegation);
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

    const [delegation] = PublicKey.findProgramAddressSync(
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
        delegation,
        systemProgram: SystemProgram.programId,
      })
      .signers([master])
      .rpc();

    assert.deepEqual(await program.account.delegation.fetch(delegation), {
      master: master.publicKey,
      representative: representative.publicKey,
      authorised: false,
    });

    await program.methods
      .confirmDelegate()
      .accounts({
        representative: representative.publicKey,
        delegation,
        systemProgram: SystemProgram.programId,
      })
      .signers([representative])
      .rpc();

    assert.deepEqual(await program.account.delegation.fetch(delegation), {
      master: master.publicKey,
      representative: representative.publicKey,
      authorised: true,
    });

    await program.methods
      .cancelDelegate()
      .accounts({
        delegation,
        systemProgram: SystemProgram.programId,
      })
      .remainingAccounts([
        { pubkey: master.publicKey, isSigner: false, isWritable: true },
        { pubkey: representative.publicKey, isSigner: true, isWritable: false },
      ])
      .signers([representative])
      .rpc();

    try {
      await program.account.delegation.fetch(delegation);
      assert(false);
    } catch (error) {
      assert.ok(`${error}`.includes("Account does not exist or has no data"));
    }
  });

  it("Example program using delegate-manager", async () => {
    const master = Keypair.generate();
    const representative = Keypair.generate();
    const hacker = Keypair.generate();

    await connection.confirmTransaction(
      await connection.requestAirdrop(master.publicKey, LAMPORTS_PER_SOL)
    );
    await connection.confirmTransaction(
      await connection.requestAirdrop(
        representative.publicKey,
        LAMPORTS_PER_SOL
      )
    );
    await connection.confirmTransaction(
      await connection.requestAirdrop(hacker.publicKey, LAMPORTS_PER_SOL)
    );

    const [delegation] = PublicKey.findProgramAddressSync(
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
        delegation,
        systemProgram: SystemProgram.programId,
      })
      .signers([master])
      .rpc();

    assert.deepEqual(await program.account.delegation.fetch(delegation), {
      master: master.publicKey,
      representative: representative.publicKey,
      authorised: false,
    });

    await program.methods
      .confirmDelegate()
      .accounts({
        representative: representative.publicKey,
        delegation,
        systemProgram: SystemProgram.programId,
      })
      .signers([representative])
      .rpc();

    assert.deepEqual(await program.account.delegation.fetch(delegation), {
      master: master.publicKey,
      representative: representative.publicKey,
      authorised: true,
    });

    const [counterAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("counter-state")],
      example.programId
    );

    await example.methods
      .incrementCounter()
      .accounts({
        counter: counterAddress,
        payer: master.publicKey,
        authority: master.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([master])
      .rpc();

    try {
      await example.methods
        .incrementCounter()
        .accounts({
          counter: counterAddress,
          payer: representative.publicKey,
          authority: master.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([representative])
        .rpc();
    } catch (error) {
      assert.ok(
        error.logs[2].includes("Missing Delegation Account"),
        "Wrong error"
      );
    }

    await example.methods
      .incrementCounter()
      .accounts({
        counter: counterAddress,
        payer: representative.publicKey,
        authority: master.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .remainingAccounts([
        { pubkey: delegation, isSigner: false, isWritable: false },
      ])
      .signers([representative])
      .rpc();

    try {
      await example.methods
        .incrementCounter()
        .accounts({
          counter: counterAddress,
          payer: hacker.publicKey,
          authority: master.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .remainingAccounts([
          { pubkey: delegation, isSigner: false, isWritable: false },
        ])
        .signers([hacker])
        .rpc();
    } catch (error) {
      assert.ok(
        error.errorLogs[0].includes("RequireKeysEqViolated"),
        "Wrong error"
      );
    }

    assert.equal(
      (await example.account.counter.fetch(counterAddress)).count,
      2
    );
  });
});
