import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Token2022Pumpfun } from "../target/types/token_2022_pumpfun";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { createAssociatedTokenAccount, createMint, getAssociatedTokenAddress, mintTo, NATIVE_MINT, NATIVE_MINT_2022, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { ComputeBudgetProgram, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { BN } from "bn.js";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("token-2022-pumpfun", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const connection = new Connection("http://localhost:8899")
  // const connection = new Connection("https://devnet.helius-rpc.com/?api-key=")

  const payer = Keypair.fromSecretKey(bs58.decode(""))
  const feeAccount = new PublicKey("")
  const program = anchor.workspace.Token2022Pumpfun as Program<Token2022Pumpfun>;

  console.log("payer." , payer.publicKey.toBase58());

  it("Is initialized!", async () => {

    const [globalConfiguration] = PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId)
    const [feeAccount] = PublicKey.findProgramAddressSync([Buffer.from("pumpfun_fee")], program.programId)

    const initializeArgu = {
      swapFee: new BN(2.0),
      bondingCurveLimitation: new BN(8 * LAMPORTS_PER_SOL),
      solAmountForDexAfterBc: new BN(5 * LAMPORTS_PER_SOL),
      solAmountForPumpfunAfterBc: new BN(2 * LAMPORTS_PER_SOL),
      solAmountForTokenCreatorAfterBc: new BN(1 * LAMPORTS_PER_SOL),
      initialVirtualSol: new BN(3 * LAMPORTS_PER_SOL),
    }

    // Add your test here.
    const tx = await program.methods
      .initialize(initializeArgu)
      .accounts({
        globalConfiguration: globalConfiguration,
        feeAccount: feeAccount
      })
      .signers([payer])
      .transaction();

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(sig);

    console.log(await program.account.initializeConfiguration.fetch(globalConfiguration))
  });

  let mintAddr: Keypair, userAta, userNativeAta;

  it("token mint", async () => {
    userNativeAta = await getAssociatedTokenAddress( NATIVE_MINT , payer.publicKey )
    mintAddr = Keypair.generate()
    const mintsig = await createMint(connection, payer, payer.publicKey, null, 9, mintAddr, { commitment: "finalized" }, TOKEN_2022_PROGRAM_ID);
    console.log(mintsig.toBase58());

    console.log("token mint = ", bs58.encode(mintAddr.secretKey));

    userAta = await createAssociatedTokenAccount(connection, payer, mintAddr.publicKey, payer.publicKey, { commitment: "finalized" }, TOKEN_2022_PROGRAM_ID);
    console.log("userAta");

    const mintto_sig = await mintTo(connection, payer, mintAddr.publicKey, userAta, payer, 10 ** 15, [], { commitment: "finalized" }, TOKEN_2022_PROGRAM_ID)
    console.log(mintto_sig);
  })

  it("create pool", async () => {
    const [globalConfiguration] = PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId)
    const [bondingCurve] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("bonding_curve")], program.programId)
    const [solPool] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("sol_pool")], program.programId)
    const tokenPool = await getAssociatedTokenAddress(mintAddr.publicKey, solPool, true, TOKEN_2022_PROGRAM_ID)

    console.log({
      globalConfiguration: globalConfiguration,
      bondingCurve: bondingCurve,
      mintAddr: mintAddr.publicKey,
      userAta: userAta,
      solPool: solPool,
      tokenPool: tokenPool,
      feeAccount: feeAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID
    });

    // Add your test here.
    const tx = await program.methods
      .createPool(new BN(10 ** 7))     //   create Pool Fee 0.01 sol
      .accounts({
        globalConfiguration: globalConfiguration,
        bondingCurve: bondingCurve,
        mintAddr: mintAddr.publicKey,
        userAta: userAta,
        solPool: solPool,
        tokenPool: tokenPool,
        feeAccount: feeAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID
      })
      .signers([payer])
      .transaction();

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(sig);

  });

  it("add liquidity", async () => {
    const [globalConfiguration] = PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId)
    const [bondingCurve] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("bonding_curve")], program.programId)
    const [solPool] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("sol_pool")], program.programId)
    const tokenPool = await getAssociatedTokenAddress(mintAddr.publicKey, solPool, true, TOKEN_2022_PROGRAM_ID)

    // Add your test here.
    const tx = await program.methods
      .addLiquidity(new BN(5 * 10 ** 13), new BN(10 ** 13))     //   token deposit 
      .accounts({
        globalConfiguration: globalConfiguration,
        bondingCurve: bondingCurve,
        mintAddr: mintAddr.publicKey,
        userAta: userAta,
        solPool: solPool,
        tokenPool: tokenPool,
        tokenProgram: TOKEN_2022_PROGRAM_ID
      })
      .signers([payer])
      .transaction();

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(sig);

    console.log("Init Configure : ", await program.account.initializeConfiguration.fetch(globalConfiguration))
    console.log("Bonding Curve : ", await program.account.bondingCurve.fetch(bondingCurve))

  });

  it("buy", async () => {

    const [globalConfiguration] = PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId)
    const [bondingCurve] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("bonding_curve")], program.programId)
    const [solPool] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("sol_pool")], program.programId)
    const tokenPool = await getAssociatedTokenAddress(mintAddr.publicKey, solPool, true, TOKEN_2022_PROGRAM_ID)

    const bunding = await program.account.bondingCurve.fetch(bondingCurve)
    const price = bunding.virtualSolReserves.div(bunding.virtualTokenReserves)

    console.log(await program.account.initializeConfiguration.fetch(globalConfiguration))
    console.log("bunding == > ", bunding.virtualSolReserves, bunding.virtualTokenReserves);
    console.log("bunding == > ", price);
    // Add your test here.
    const tx = await program.methods
      .buy(new BN(2 * 10 ** 8))     //   buy 0.1 sol
      .accounts({
        globalConfiguration: globalConfiguration,
        bondingCurve: bondingCurve,
        mintAddr: mintAddr.publicKey,
        userAta: userAta,
        solPool: solPool,
        tokenPool: tokenPool,
        feeAccount: feeAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID
      })
      .signers([payer])
      .transaction();

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(sig);
  });

  it("sell", async () => {
    const [globalConfiguration] = PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId)
    const [bondingCurve] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("bonding_curve")], program.programId)
    const [solPool] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("sol_pool")], program.programId)
    const tokenPool = await getAssociatedTokenAddress(mintAddr.publicKey, solPool, true, TOKEN_2022_PROGRAM_ID)

    const bunding = await program.account.bondingCurve.fetch(bondingCurve)
    const price = bunding.virtualSolReserves.div(bunding.virtualTokenReserves)

    console.log("bunding == > ", bunding.virtualSolReserves, bunding.virtualTokenReserves);
    console.log("bunding == > ", price);
    // Add your test here.
    const tx = await program.methods
      .sell(new BN(10 ** 8))     //   sell 0.1 token
      .accounts({
        globalConfiguration: globalConfiguration,
        bondingCurve: bondingCurve,
        mintAddr: mintAddr.publicKey,
        userAta: userAta,
        solPool: solPool,
        tokenPool: tokenPool,
        feeAccount: feeAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([payer])
      .transaction();

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(sig);
  });

  
  it("buy", async () => {

    const [globalConfiguration] = PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId)
    const [bondingCurve] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("bonding_curve")], program.programId)
    const [solPool] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("sol_pool")], program.programId)
    const tokenPool = await getAssociatedTokenAddress(mintAddr.publicKey, solPool, true, TOKEN_2022_PROGRAM_ID)

    const bunding = await program.account.bondingCurve.fetch(bondingCurve)
    const price = bunding.virtualSolReserves.div(bunding.virtualTokenReserves)

    console.log(await program.account.initializeConfiguration.fetch(globalConfiguration))
    console.log("bunding == > ", bunding.virtualSolReserves, bunding.virtualTokenReserves);
    console.log("bunding == > ", price);
    // Add your test here.
    const tx = await program.methods
      .buy(new BN(8 * LAMPORTS_PER_SOL))     //   buy 0.1 sol
      .accounts({
        globalConfiguration: globalConfiguration,
        bondingCurve: bondingCurve,
        mintAddr: mintAddr.publicKey,
        userAta: userAta,
        solPool: solPool,
        tokenPool: tokenPool,
        feeAccount: feeAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID
      })
      .signers([payer])
      .transaction();

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(sig);
  });

  it("remove liquidity", async () => {

    const [globalConfiguration] = PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId)
    const [bondingCurve] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("bonding_curve")], program.programId)
    const [solPool] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("sol_pool")], program.programId)
    const tokenPool = await getAssociatedTokenAddress(mintAddr.publicKey, solPool, true, TOKEN_2022_PROGRAM_ID)

    console.log("solPool : ", solPool);
    console.log("tokenPool : ", tokenPool);


    //  coin mint address


    const tx = await program.methods
      .removeLiquidity()
      .accounts({
        globalConfiguration: globalConfiguration,
        bondingCurve: bondingCurve,
        ammCoinMint: mintAddr.publicKey,
        solPool: solPool,
        tokenPool: tokenPool,
        userTokenCoin: userAta,
        userTokenPc: userNativeAta,
        userWallet: payer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        splTokenProgram : TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID,
        sysvarRent: SYSVAR_RENT_PUBKEY
      })
      .preInstructions([
        ComputeBudgetProgram.setComputeUnitLimit({
          units: 1_000_000
        })
      ])
      .signers([payer])
      .transaction()

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx))
    const sig = await sendAndConfirmTransaction(connection, tx, [payer], { skipPreflight: true })
    console.log("Successfully Removed liquidity : ", sig)

  })
});
