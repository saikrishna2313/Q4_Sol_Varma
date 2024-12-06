"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const prompt_1 = __importDefault(require("prompt"));
const web3_js_1 = require("@solana/web3.js");
//Create a Solana devnet connection to claim 2 devnet SOL tokens
const connection = new web3_js_1.Connection("https://api.devnet.solana.com");
(async () => {
    // Start our prompt
    prompt_1.default.start();
    // Take in base58 string
    console.log('Enter your address and how much SOL to airdrop):');
    const { address, sol } = await prompt_1.default.get(['address', 'sol']);
    const wallet = new web3_js_1.PublicKey(address);
    try {
        const txhash = await connection.requestAirdrop(wallet, (web3_js_1.LAMPORTS_PER_SOL * parseInt(sol)));
        console.log(`Success! Check out your TX here:\nhttps://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    }
    catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();
