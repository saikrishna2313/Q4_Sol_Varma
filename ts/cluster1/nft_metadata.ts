import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://devnet.irys.xyz/eS3oDWKzVNLtX9eBSwYce3EQoE8eKeztf5avC1mi8Do";
        const metadata = {
          name: "DEAN",
          symbol: "This is DEAN",
          description: "I MOG PEOPLE",
          image: "https://devnet.irys.xyz/eS3oDWKzVNLtX9eBSwYce3EQoE8eKeztf5avC1mi8Do",
          attributes: [
            { trait_type: "RUST", value: "99"},
            { trait_type: "Teaching", value: "99"}
        ],
          properties: {
            files: [
              {
                type: "image/png",
                uri: "https://devnet.irys.xyz/eS3oDWKzVNLtX9eBSwYce3EQoE8eKeztf5avC1mi8Do",
              },
            ],
          },
          creators: [
          ],
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri.replace("arweave.net", "devnet.irys.xyz"));
      } catch (error) {
        console.log("Oops.. Something went wrong", error);
      }
    })();