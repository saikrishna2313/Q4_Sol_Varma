"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const web3_js_1 = require("@solana/web3.js");
const anchor_1 = require("@coral-xyz/anchor");
const wba_vault_1 = require("./programs/wba_vault");
const wba_wallet_json_1 = __importDefault(require("./wallet/wba-wallet.json"));
// Import our keypair from the wallet file
const keypair = web3_js_1.Keypair.fromSecretKey(new Uint8Array(wba_wallet_json_1.default));
// Commitment
const commitment = "finalized";
// Create a devnet connection
const connection = new web3_js_1.Connection("https://api.devnet.solana.com");
// Create our anchor provider
const provider = new anchor_1.AnchorProvider(connection, new anchor_1.Wallet(keypair), {
    commitment,
});
// Create our program
const program = new anchor_1.Program(wba_vault_1.IDL, "<address>", provider);
// Create a random keypair
const vaultState = new web3_js_1.PublicKey("<address>");
// Create the PDA for our enrollment account
// Seeds are "auth", vaultState
// const vaultAuth = ???
// Create the vault key
// Seeds are "vault", vaultAuth
// const vault = ???
// Mint address
const mint = new web3_js_1.PublicKey("<address>");
// Execute our enrollment transaction
(async () => {
    try {
        const metadataProgram = new web3_js_1.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
        const metadataAccount = web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("metadata"), metadataProgram.toBuffer(), mint.toBuffer()], metadataProgram)[0];
        const masterEdition = web3_js_1.PublicKey.findProgramAddressSync([
            Buffer.from("metadata"),
            metadataProgram.toBuffer(),
            mint.toBuffer(),
            Buffer.from("edition"),
        ], metadataProgram)[0];
        // Get the token account of the fromWallet address, and if it does not exist, create it
        // const ownerAta = await getOrCreateAssociatedTokenAccount(
        //     ???
        // );
        // Get the token account of the fromWallet address, and if it does not exist, create it
        // const vaultAta = await getOrCreateAssociatedTokenAccount(
        //     ???
        // );
        // const signature = await program.methods
        // .withdrawNft()
        // .accounts({
        //    ???
        // })
        // .signers([
        //     keypair
        // ]).rpc();
        // console.log(`Deposit success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
    }
    catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();
