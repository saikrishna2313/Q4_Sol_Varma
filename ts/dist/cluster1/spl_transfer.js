"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const web3_js_1 = require("@solana/web3.js");
const wba_wallet_json_1 = __importDefault(require("../wba-wallet.json"));
// We're going to import our keypair from the wallet file
const keypair = web3_js_1.Keypair.fromSecretKey(new Uint8Array(wba_wallet_json_1.default));
//Create a Solana devnet connection
const commitment = "confirmed";
const connection = new web3_js_1.Connection("https://api.devnet.solana.com", commitment);
// Mint address
const mint = new web3_js_1.PublicKey("<mint address>");
// Recipient address
const to = new web3_js_1.PublicKey("<receiver address>");
(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        // Get the token account of the toWallet address, and if it does not exist, create it
        // Transfer the new token to the "toTokenAccount" we just created
    }
    catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();
