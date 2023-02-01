"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const anchor = __importStar(require("@project-serum/anchor"));
const web3_js_1 = require("@solana/web3.js");
const chai_1 = require("chai");
describe("delegation-manager", () => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace
        .DelegationManager;
    const example = anchor.workspace.Example;
    const connection = anchor.getProvider().connection;
    it("Initialize, confirm, cancel by authority", () => __awaiter(void 0, void 0, void 0, function* () {
        const master = web3_js_1.Keypair.generate();
        const representative = web3_js_1.Keypair.generate();
        yield connection.confirmTransaction(yield connection.requestAirdrop(master.publicKey, web3_js_1.LAMPORTS_PER_SOL));
        yield connection.confirmTransaction(yield connection.requestAirdrop(representative.publicKey, web3_js_1.LAMPORTS_PER_SOL));
        const [delegation] = web3_js_1.PublicKey.findProgramAddressSync([
            Buffer.from("authorize"),
            master.publicKey.toBuffer(),
            representative.publicKey.toBuffer(),
        ], program.programId);
        yield program.methods
            .initializeDelegate()
            .accounts({
            master: master.publicKey,
            representative: representative.publicKey,
            delegation,
            systemProgram: web3_js_1.SystemProgram.programId,
        })
            .signers([master])
            .rpc();
        chai_1.assert.deepEqual(yield program.account.delegation.fetch(delegation), {
            master: master.publicKey,
            representative: representative.publicKey,
            authorised: false,
        });
        yield program.methods
            .confirmDelegate()
            .accounts({
            representative: representative.publicKey,
            delegation,
            systemProgram: web3_js_1.SystemProgram.programId,
        })
            .signers([representative])
            .rpc();
        chai_1.assert.deepEqual(yield program.account.delegation.fetch(delegation), {
            master: master.publicKey,
            representative: representative.publicKey,
            authorised: true,
        });
        yield program.methods
            .cancelDelegate()
            .accounts({
            delegation,
            systemProgram: web3_js_1.SystemProgram.programId,
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
            yield program.account.delegation.fetch(delegation);
            (0, chai_1.assert)(false);
        }
        catch (error) {
            chai_1.assert.ok(`${error}`.includes("Account does not exist or has no data"));
        }
    }));
    it("Initialize, confirm, cancel by delegate", () => __awaiter(void 0, void 0, void 0, function* () {
        const master = web3_js_1.Keypair.generate();
        const representative = web3_js_1.Keypair.generate();
        yield connection.confirmTransaction(yield connection.requestAirdrop(master.publicKey, web3_js_1.LAMPORTS_PER_SOL));
        yield connection.confirmTransaction(yield connection.requestAirdrop(representative.publicKey, web3_js_1.LAMPORTS_PER_SOL));
        const [delegation] = web3_js_1.PublicKey.findProgramAddressSync([
            Buffer.from("authorize"),
            master.publicKey.toBuffer(),
            representative.publicKey.toBuffer(),
        ], program.programId);
        yield program.methods
            .initializeDelegate()
            .accounts({
            master: master.publicKey,
            representative: representative.publicKey,
            delegation,
            systemProgram: web3_js_1.SystemProgram.programId,
        })
            .signers([master])
            .rpc();
        chai_1.assert.deepEqual(yield program.account.delegation.fetch(delegation), {
            master: master.publicKey,
            representative: representative.publicKey,
            authorised: false,
        });
        yield program.methods
            .confirmDelegate()
            .accounts({
            representative: representative.publicKey,
            delegation,
            systemProgram: web3_js_1.SystemProgram.programId,
        })
            .signers([representative])
            .rpc();
        chai_1.assert.deepEqual(yield program.account.delegation.fetch(delegation), {
            master: master.publicKey,
            representative: representative.publicKey,
            authorised: true,
        });
        yield program.methods
            .cancelDelegate()
            .accounts({
            delegation,
            systemProgram: web3_js_1.SystemProgram.programId,
        })
            .remainingAccounts([
            { pubkey: master.publicKey, isSigner: false, isWritable: true },
            { pubkey: representative.publicKey, isSigner: true, isWritable: false },
        ])
            .signers([representative])
            .rpc();
        try {
            yield program.account.delegation.fetch(delegation);
            (0, chai_1.assert)(false);
        }
        catch (error) {
            chai_1.assert.ok(`${error}`.includes("Account does not exist or has no data"));
        }
    }));
    it("Example program using delegate-manager", () => __awaiter(void 0, void 0, void 0, function* () {
        const master = web3_js_1.Keypair.generate();
        const representative = web3_js_1.Keypair.generate();
        const hacker = web3_js_1.Keypair.generate();
        yield connection.confirmTransaction(yield connection.requestAirdrop(master.publicKey, web3_js_1.LAMPORTS_PER_SOL));
        yield connection.confirmTransaction(yield connection.requestAirdrop(representative.publicKey, web3_js_1.LAMPORTS_PER_SOL));
        yield connection.confirmTransaction(yield connection.requestAirdrop(hacker.publicKey, web3_js_1.LAMPORTS_PER_SOL));
        const [delegation] = web3_js_1.PublicKey.findProgramAddressSync([
            Buffer.from("authorize"),
            master.publicKey.toBuffer(),
            representative.publicKey.toBuffer(),
        ], program.programId);
        yield program.methods
            .initializeDelegate()
            .accounts({
            master: master.publicKey,
            representative: representative.publicKey,
            delegation,
            systemProgram: web3_js_1.SystemProgram.programId,
        })
            .signers([master])
            .rpc();
        chai_1.assert.deepEqual(yield program.account.delegation.fetch(delegation), {
            master: master.publicKey,
            representative: representative.publicKey,
            authorised: false,
        });
        yield program.methods
            .confirmDelegate()
            .accounts({
            representative: representative.publicKey,
            delegation,
            systemProgram: web3_js_1.SystemProgram.programId,
        })
            .signers([representative])
            .rpc();
        chai_1.assert.deepEqual(yield program.account.delegation.fetch(delegation), {
            master: master.publicKey,
            representative: representative.publicKey,
            authorised: true,
        });
        const [counterAddress] = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("counter-state")], example.programId);
        yield example.methods
            .incrementCounter()
            .accounts({
            counter: counterAddress,
            payer: master.publicKey,
            authority: master.publicKey,
            systemProgram: web3_js_1.SystemProgram.programId,
        })
            .signers([master])
            .rpc();
        try {
            yield example.methods
                .incrementCounter()
                .accounts({
                counter: counterAddress,
                payer: representative.publicKey,
                authority: master.publicKey,
                systemProgram: web3_js_1.SystemProgram.programId,
            })
                .signers([representative])
                .rpc();
        }
        catch (error) {
            chai_1.assert.ok(error.logs[2].includes("Missing Delegation Account"), "Wrong error");
        }
        yield example.methods
            .incrementCounter()
            .accounts({
            counter: counterAddress,
            payer: representative.publicKey,
            authority: master.publicKey,
            systemProgram: web3_js_1.SystemProgram.programId,
        })
            .remainingAccounts([
            { pubkey: delegation, isSigner: false, isWritable: false },
        ])
            .signers([representative])
            .rpc();
        try {
            yield example.methods
                .incrementCounter()
                .accounts({
                counter: counterAddress,
                payer: hacker.publicKey,
                authority: master.publicKey,
                systemProgram: web3_js_1.SystemProgram.programId,
            })
                .remainingAccounts([
                { pubkey: delegation, isSigner: false, isWritable: false },
            ])
                .signers([hacker])
                .rpc();
        }
        catch (error) {
            chai_1.assert.ok(error.errorLogs[0].includes("RequireKeysEqViolated"), "Wrong error");
        }
        chai_1.assert.equal((yield example.account.counter.fetch(counterAddress)).count, 2);
    }));
});
