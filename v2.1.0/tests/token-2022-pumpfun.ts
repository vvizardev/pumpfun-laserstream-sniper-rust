import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Token2022Pumpfun } from "../target/types/token_2022_pumpfun";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { getAssociatedTokenAddress, getAssociatedTokenAddressSync, NATIVE_MINT, burn, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID, createBurnInstruction, getMint } from "@solana/spl-token";
import { ComputeBudgetProgram, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { BN } from "bn.js";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { getPoolAddress, getPoolLpMintAddress, getPoolVaultAddress } from "./utils";
import { configAddress, cpSwapProgram, createPoolFeeReceive } from "./config";


// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());
// const connection = new Connection("http://localhost:8899")
const connection = new Connection("https://devnet.helius-rpc.com/?api-key=", { commitment: "finalized" })

const payer = Keypair.fromSecretKey(bs58.decode(""))
const feeAccount = new PublicKey("3MQVpAwsccXHG7k6RvhwBVRCs3tfmHRW8VUYJUdyPBXd")
const program = anchor.workspace.Token2022Pumpfun as Program<Token2022Pumpfun>;
let mintAddr: Keypair, userAta, userNativeAta;

describe("token-2022-pumpfun", () => {

  console.log("payer.", payer.publicKey.toBase58());

  it("Is initialized!", async () => {

    const [globalConfiguration] = PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId)

    const initializeArgu = {
      bondingCurveLimitation: new BN(85 * LAMPORTS_PER_SOL),
      initialVirtualSol: new BN(40 * LAMPORTS_PER_SOL),
      initialVirtualToken: new BN(1030000000).mul(new BN(LAMPORTS_PER_SOL)),
      createPoolFeeLamports: new BN(0.05 * LAMPORTS_PER_SOL),
      swapFee: 2.0,
    }

    // Add your test here.
    const tx = await program.methods
      .initialize(initializeArgu)
      .accounts({
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

  it("create", async () => {
    mintAddr = Keypair.generate()

    const [solPool] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("sol_pool")], program.programId)
    const tokenPool = await getAssociatedTokenAddress(mintAddr.publicKey, solPool, true, TOKEN_2022_PROGRAM_ID)

    console.log(mintAddr.publicKey.toBase58());

    // Add your test here.
    const tx = await program.methods
      .create({
        name: "wiz05.06",
        symbol: "wizSym",
        uri: "wizUri",
        transferFeeBasisPoints: 50, ///   0.005 %
        maximumFee: new BN(5000)
      })     //   create Pool Fee 0.01 sol
      .accounts({
        mintAddr: mintAddr.publicKey,
        tokenPool: tokenPool,
        feeAccount: feeAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID
      })
      .signers([payer, mintAddr])
      .transaction();

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer, mintAddr]);
    console.log(sig);

  });

  it("buy", async () => {

    const buyQuote = await getBuyQuote(3 * 10 ** 9, 2)

    const tx = await program.methods
      .buy(new BN(3 * 10 ** 9), buyQuote)     //   buy 0.1 sol
      .accounts({
        mintAddr: mintAddr.publicKey,
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
    const sellQuote = await getSellQuote(10 ** 13, 2)

    // Add your test here.
    const tx = await program.methods
      .sell(new BN(10 ** 13), sellQuote)    //   buy amount / expected amount / slippage
      .accounts({
        mintAddr: mintAddr.publicKey,
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


  // it("buy", async () => {

  //   const buyQuote = await getBuyQuote(10 ** 9)

  //   const tx = await program.methods
  //     .buy(new BN(2 * 10 ** 9), buyQuote, 2.0)     //   buy 0.1 sol
  //     .accounts({
  //       mintAddr: mintAddr.publicKey,
  //       feeAccount: feeAccount,
  //       tokenProgram: TOKEN_2022_PROGRAM_ID
  //     })
  //     .signers([payer])
  //     .transaction();

  //   tx.feePayer = payer.publicKey
  //   tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

  //   console.log(await connection.simulateTransaction(tx));

  //   const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
  //   console.log(sig);
  // });

  // it("buy", async () => {

  //   const [globalConfiguration] = PublicKey.findProgramAddressSync([Buffer.from("global_config")], program.programId)
  //   const [bondingCurve] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("bonding_curve")], program.programId)
  //   const [solPool] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("sol_pool")], program.programId)
  //   const tokenPool = await getAssociatedTokenAddress(mintAddr.publicKey, solPool, true, TOKEN_2022_PROGRAM_ID)

  //   const bunding = await program.account.bondingCurve.fetch(bondingCurve)
  //   const price = bunding.virtualSolReserves.div(bunding.virtualTokenReserves)

  //   console.log(await program.account.initializeConfiguration.fetch(globalConfiguration))
  //   console.log("bunding == > ", bunding.virtualSolReserves, bunding.virtualTokenReserves);
  //   console.log("bunding == > ", price);
  //   // Add your test here.
  //   const tx = await program.methods
  //     .buy(new BN(8 * LAMPORTS_PER_SOL))     //   buy 0.1 sol
  //     .accounts({
  //       globalConfiguration: globalConfiguration,
  //       bondingCurve: bondingCurve,
  //       mintAddr: mintAddr.publicKey,
  //       userAta: userAta,
  //       solPool: solPool,
  //       tokenPool: tokenPool,
  //       feeAccount: feeAccount,
  //       tokenProgram: TOKEN_2022_PROGRAM_ID
  //     })
  //     .signers([payer])
  //     .transaction();

  //   tx.feePayer = payer.publicKey
  //   tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

  //   console.log(await connection.simulateTransaction(tx));

  //   const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
  //   console.log(sig);
  // });

  const opAddress = Keypair.generate()

  it("remove liquidity", async () => {
    // Add your test here.
    const tx = await program.methods
      .removeLiquidity()    //   buy amount / expected amount / slippage
      .accounts({
        creator: payer.publicKey,
        nativeMint: NATIVE_MINT,
        opAddress: opAddress.publicKey,
        mintAddr: mintAddr.publicKey,
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

  it("initialize proxy", async () => {









    //  contact to https://t.me/wizardev








});

const getBuyQuote = async (lamport, slippage) => {
  const [bondingCurve] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("bonding_curve")], program.programId)
  const { initVirtualSol, solReserves, initVirtualToken, kParam } = await program.account.bondingCurve.fetch(bondingCurve)

  const initVirtualSolNew = Number(initVirtualSol)
  const solReservesNew = Number(solReserves)
  const initVirtualTokenNew = Number(initVirtualToken)
  const tokenNew = initVirtualTokenNew - (Number(kParam) / (initVirtualSolNew + solReservesNew))

  let price = (initVirtualSolNew + solReservesNew) / (initVirtualTokenNew - tokenNew)

  return new BN(`${((1 - 0.01 * slippage) * lamport) / price}`)
}

const getSellQuote = async (token_amount, slippage) => {
  const [bondingCurve] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("bonding_curve")], program.programId)
  const { initVirtualSol, solReserves, initVirtualToken, kParam } = await program.account.bondingCurve.fetch(bondingCurve)

  const initVirtualSolNew = Number(initVirtualSol)
  const solReservesNew = Number(solReserves)
  const initVirtualTokenNew = Number(initVirtualToken)
  const tokenNew = initVirtualTokenNew - (Number(kParam) / (initVirtualSolNew + solReservesNew))

  let price = (initVirtualSolNew + solReservesNew) / (initVirtualTokenNew - tokenNew)

  return new BN(`${Math.floor((1 - 0.01 * slippage) * token_amount * price)}`)
}