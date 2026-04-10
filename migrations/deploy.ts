import * as anchor from "@coral-xyz/anchor";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction
} from "@solana/web3.js";

import {
  TOKEN_2022_PROGRAM_ID,
  ExtensionType,
  getMintLen,
  createInitializeMintInstruction,
  createInitializeMetadataPointerInstruction,
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountIdempotentInstruction,
  ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";

const RESOURCE_NAMES = [
  "WOOD",
  "IRON",
  "GOLD",
  "LEATHER",
  "STONE",
  "DIAMOND"
];

const ITEM_PRICES = [
  new anchor.BN(10),
  new anchor.BN(20),
  new anchor.BN(30),
  new anchor.BN(40)
];

// Token-2022 mint
async function createToken2022Mint(
  provider: anchor.AnchorProvider,
  payer: Keypair,
  authority: PublicKey
): Promise<PublicKey> {
  const mint = Keypair.generate();

  const extensions = [ExtensionType.MetadataPointer];
  const mintLen = getMintLen(extensions);

  const lamports =
    await provider.connection.getMinimumBalanceForRentExemption(mintLen);

  const tx = new Transaction();

  tx.add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mint.publicKey,
      space: mintLen,
      lamports,
      programId: TOKEN_2022_PROGRAM_ID
    })
  );

  tx.add(
    createInitializeMetadataPointerInstruction(
      mint.publicKey,
      authority,
      mint.publicKey,
      TOKEN_2022_PROGRAM_ID
    )
  );

  tx.add(
    createInitializeMintInstruction(
      mint.publicKey,
      0, 
      authority,
      null,
      TOKEN_2022_PROGRAM_ID
    )
  );

  await sendAndConfirmTransaction(
    provider.connection,
    tx,
    [payer, mint],
    { commitment: "confirmed" }
  );

  return mint.publicKey;
}

// ATA
async function createAta(
  provider: anchor.AnchorProvider,
  payer: Keypair,
  mint: PublicKey,
  owner: PublicKey
): Promise<PublicKey> {
  const ata = getAssociatedTokenAddressSync(
    mint,
    owner,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  const tx = new Transaction().add(
    createAssociatedTokenAccountIdempotentInstruction(
      payer.publicKey,
      ata,
      owner,
      mint,
      TOKEN_2022_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    )
  );

  await sendAndConfirmTransaction(provider.connection, tx, [payer]);

  return ata;
}

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;
  const payer = wallet.payer;

  const resourceManager = anchor.workspace.ResourceManager;

  console.log("Wallet:", wallet.publicKey.toBase58());

  // PDA
  const [gameConfig] = PublicKey.findProgramAddressSync(
    [Buffer.from("game-config")],
    resourceManager.programId
  );

  const [resourceAuthority] = PublicKey.findProgramAddressSync(
    [Buffer.from("resource-auth")],
    resourceManager.programId
  );

  const [magicAuthority] = PublicKey.findProgramAddressSync(
    [Buffer.from("magic-auth")],
    anchor.workspace.MagicToken.programId
  );

  console.log("GameConfig:", gameConfig.toBase58());
  console.log("ResourceAuthority:", resourceAuthority.toBase58());
  console.log("MagicAuthority:", magicAuthority.toBase58());

  const resourceMints: PublicKey[] = [];

  for (const name of RESOURCE_NAMES) {
    const mint = await createToken2022Mint(
      provider,
      payer,
      resourceAuthority
    );
    resourceMints.push(mint);

    console.log(`${name} mint:`, mint.toBase58());
  }

  // MagicToken
  const magicMint = await createToken2022Mint(
    provider,
    payer,
    magicAuthority
  );

  console.log("MagicToken mint:", magicMint.toBase58());

  // ATA для гравця
  for (const mint of resourceMints) {
    await createAta(provider, payer, mint, wallet.publicKey);
  }

  await createAta(provider, payer, magicMint, wallet.publicKey);

  console.log("ATA created");

  // initialize game
  try {
    const tx = await resourceManager.methods
      .initializeGame(ITEM_PRICES)
      .accounts({
        admin: wallet.publicKey,
        gameConfig,
        resourceMint0: resourceMints[0],
        resourceMint1: resourceMints[1],
        resourceMint2: resourceMints[2],
        resourceMint3: resourceMints[3],
        resourceMint4: resourceMints[4],
        resourceMint5: resourceMints[5],
        magicTokenMint: magicMint,
        systemProgram: SystemProgram.programId
      })
      .rpc();

    console.log("initializeGame tx:", tx);
  } catch (e) {
    console.log("initializeGame already executed or failed");
  }

  console.log("DONE ✅");
}

main().catch(console.error);