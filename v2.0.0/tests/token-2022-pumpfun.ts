import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Token2022Pumpfun } from "../target/types/token_2022_pumpfun";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { getAssociatedTokenAddress, getAssociatedTokenAddressSync, NATIVE_MINT, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { AddressLookupTableProgram, ComputeBudgetProgram, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, SYSVAR_RENT_PUBKEY, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import { BN } from "bn.js";
import { getNftMetadataAddress, getOrcleAccountAddress, getPersonalPositionAddress, getPoolAddress, getPoolVaultAddress, getProtocolPositionAddress, getTickArrayAddress, getTickArrayBitmapAddress, i32ToBytes, initialize, openPosition, setupInitializeTest, waitFor } from "./utils";
import { DEVNET_PROGRAM_ID, SqrtPriceMath, TickUtils } from "@raydium-io/raydium-sdk-v2";
import { ClmmProgram, devConfigs } from "./config";
import { createAndSendV0Tx } from "./utils/transaction";

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());
// const connection = new Connection("http://localhost:8899")
export const connection = new Connection("https://devnet.helius-rpc.com/?api-key=", { commitment: "finalized" })

const payer = Keypair.fromSecretKey(bs58.decode(""))
const feeAccount = new PublicKey("3MQVpAwsccXHG7k6RvhwBVRCs3tfmHRW8VUYJUdyPBXd")
const program = anchor.workspace.Token2022Pumpfun as Program<Token2022Pumpfun>;
let mintAddr: Keypair;

describe("token-2022-pumpfun", () => {
  it("Is initialized!", async () => {

    const initializeArgu = {
      bondingCurveLimitation: new BN(85 * LAMPORTS_PER_SOL),
      initialVirtualSol: new BN(40 * LAMPORTS_PER_SOL),
      initialVirtualToken: new BN(1030000000).mul(new BN(LAMPORTS_PER_SOL)),
      createPoolFeeLamports: new BN(0.05 * LAMPORTS_PER_SOL),
      swapFee: 2.0,
    }

    const tx = await program.methods
      .initialize(initializeArgu)
      .accounts({ feeAccount: feeAccount })
      .signers([payer])
      .transaction();

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));
    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log(`https://solscan.io/tx/${sig}?cluster=devnet`);
    console.log(`https://solana.fm/tx/${sig}?cluster=devnet-solana`)
  });

  it("create", async () => {
    mintAddr = Keypair.generate()
    const [solPool] = PublicKey.findProgramAddressSync([mintAddr.publicKey.toBuffer(), Buffer.from("sol_pool")], program.programId)
    const tokenPool = await getAssociatedTokenAddress(mintAddr.publicKey, solPool, true, TOKEN_2022_PROGRAM_ID)

    console.log("mintAddr : ", mintAddr.publicKey.toBase58());

    const tx = await program.methods
      .create({
        name: "wizardev",
        symbol: "wizardevSym",
        uri: "wizardevUri",
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
    console.log(`https://solscan.io/tx/${sig}?cluster=devnet`)
    console.log(`https://solana.fm/tx/${sig}?cluster=devnet-solana`)
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
    console.log(`https://solscan.io/tx/${sig}?cluster=devnet`)
    console.log(`https://solana.fm/tx/${sig}?cluster=devnet-solana`)
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
    console.log(`https://solscan.io/tx/${sig}?cluster=devnet`)
    console.log(`https://solana.fm/tx/${sig}?cluster=devnet-solana`)
  });

  it("raydium integrate", async () => {
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