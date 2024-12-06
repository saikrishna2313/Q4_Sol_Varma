"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const bs58_1 = __importDefault(require("bs58"));
const prompt_1 = __importDefault(require("prompt"));
(async () => {
    // Start our prompt
    prompt_1.default.start();
    // Take in base58 string
    console.log('Enter your base58-encoded private key:');
    const { privkey } = await prompt_1.default.get(['privkey']);
    // Decode private key
    const wallet = bs58_1.default.decode(privkey);
    // Print out wallet
    console.log(`Your wallet file is:\n[${wallet}]`);
})();
