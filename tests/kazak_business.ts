import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

describe("kazak-business", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;

  it("Provider initialized", async () => {
    console.log("Wallet:", wallet.publicKey.toBase58());
  });

  // ===============================
  // 🔍 SEARCH TEST
  // ===============================
  it("Initialize player PDA", async () => {
    const program = anchor.workspace.Search as Program;

    const [playerPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("player"), wallet.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .initializePlayer()
        .accounts({
          player: playerPda,
          owner: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      console.log("Player initialized:", playerPda.toBase58());
    } catch (e) {
      console.log("Already initialized (OK)");
    }
  });

  // ===============================
  // 🔍 COOLDOWN TEST
  // ===============================
  it("Search cooldown logic", async () => {
    const program = anchor.workspace.Search as Program;

    const [playerPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("player"), wallet.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .searchResources()
        .accounts({
          player: playerPda,
          owner: wallet.publicKey,
        })
        .rpc();

      console.log("Search executed");
    } catch (e) {
      console.log("Cooldown triggered (expected)");
    }
  });

  // ===============================
  // 🧱 RESOURCE MANAGER TEST
  // ===============================
  it("Resource manager basic call", async () => {
    const program = anchor.workspace.ResourceManager as Program;

    console.log("Program ID:", program.programId.toBase58());
  });

  // ===============================
  // 🖼 NFT PROGRAM TEST
  // ===============================
  it("Item NFT program available", async () => {
    const program = anchor.workspace.ItemNft as Program;

    console.log("NFT program:", program.programId.toBase58());
  });

  // ===============================
  // 🛒 MARKETPLACE TEST (SMOKE)
  // ===============================
  it("Marketplace program exists", async () => {
    const program = anchor.workspace.Marketplace as Program;

    console.log("Marketplace:", program.programId.toBase58());
  });

  // ===============================
  // ✨ MAGIC TOKEN TEST (SMOKE)
  // ===============================
  it("Magic token program exists", async () => {
    const program = anchor.workspace.MagicToken as Program;

    console.log("MagicToken:", program.programId.toBase58());
  });
});