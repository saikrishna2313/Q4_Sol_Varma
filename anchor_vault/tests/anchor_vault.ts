import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";

describe("anchor_vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;

  it("Is initialized!", async () => {
    // Init test
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
  it("deposit", async () => {
    const tx = await program.methods.deposit(new anchor.BN(0.2)).rpc();
    console.log("Your transaction signature", tx);
  });
  it("withdraw", async () => {
    const tx = await program.methods.withdraw(new anchor.BN(0.1)).rpc();
    console.log("Your transaction signature", tx);
  });
});

// anchor_vault
// Your transaction signature 3y5HmNqeA5pPEaxP6zqCUVJrq5Ne7NXHBC5thGsZGEXSsjaYB9L2d5iAeorGU7rbv33MvGnrAZyY7juwN2vaYQm5
//     ✔ Is initialized! (434ms)
// Your transaction signature 4gnBCbg6xVqGKVzTBxLHuo79uhQLzJLWdFe3Nx8MyVNA9K6pbYpKHdYWnxCXrdvAShdZF6uvjFb5AaZbCGYKoaLc
//     ✔ deposit (483ms)
// Your transaction signature 3tC7qBNNKn9Mh5m5V3WJkyK1AnR6pL2Je56gvCQL4U87TKGKBR5WeVidG6r9z4Sy58jxFi5YvPeY6DPpvFd8RapC
//     ✔ withdraw (493ms)


//   3 passing (1s)

// ✨  Done in 4.83s.

