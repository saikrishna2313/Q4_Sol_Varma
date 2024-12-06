"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const web3_js_1 = require("@solana/web3.js");
const dev_wallet_json_1 = __importDefault(require("./dev-wallet.json"));
// We're going to import our keypair from the wallet file
const keypair = web3_js_1.Keypair.fromSecretKey(new Uint8Array(dev_wallet_json_1.default));
//Create a Solana devnet connection to claim 2 devnet SOL tokens
const connection = new web3_js_1.Connection("https://api.devnet.solana.com");
(async () => {
    try {
        const txhash = await connection.requestAirdrop(keypair.publicKey, 2 * web3_js_1.LAMPORTS_PER_SOL);
        console.log(`Success! Check out your TX here: 
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    }
    catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();
