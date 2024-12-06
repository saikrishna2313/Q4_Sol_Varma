"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const web3_js_1 = require("@solana/web3.js");
const anchor_1 = require("@project-serum/anchor");
const wba_prereq_1 = require("../programs/wba_prereq");
const wba_wallet_json_1 = __importDefault(require("../wba-wallet.json"));
// We're going to import our keypair from the wallet file
const keypair = web3_js_1.Keypair.fromSecretKey(new Uint8Array(wba_wallet_json_1.default));
// Create a devnet connection
const connection = new web3_js_1.Connection("https://api.devnet.solana.com");
// Github account
const github = Buffer.from("testaccount", "utf8");
// Create our anchor provider
const provider = new anchor_1.AnchorProvider(connection, new anchor_1.Wallet(keypair), { commitment: "confirmed" });
// Create our program
const program = new anchor_1.Program(wba_prereq_1.IDL, "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1", provider);
// Create the PDA for our enrollment account
const enrollment_seeds = [Buffer.from("prereq"), keypair.publicKey.toBuffer()];
const [enrollment_key, _bump] = web3_js_1.PublicKey.findProgramAddressSync(enrollment_seeds, program.programId);
// Execute our enrollment transaction
(async () => {
    try {
        const txhash = await program.methods
            .complete(github)
            .accounts({
            signer: keypair.publicKey,
            prereq: enrollment_key,
            systemProgram: web3_js_1.SystemProgram.programId,
        })
            .signers([
            keypair
        ]).rpc();
        console.log(`Success! Check out your TX here: 
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    }
    catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();
