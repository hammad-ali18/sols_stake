import * as anchor from "@project-serum/anchor";
import { join } from "path";
import { Program } from "@coral-xyz/anchor";
import { Day6 } from "../target/types/day6";
import { expect } from "chai";
import {Token,TOKEN_PROGRAM_ID} from "@solana/spl-token";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, clusterApiUrl } from "@solana/web3.js";
import { readFileSync } from "fs";

describe("day6", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()

  anchor.setProvider(provider)


  const program = anchor.workspace.Day6 as Program<Day6>;
const counter =  anchor.web3.Keypair.generate();
const signer1= anchor.web3.Keypair.generate();
   // Add your test here.
   
  //  it("Initialize",async()=>{
  //   const tx = await program.methods.initialize()
  //   .accounts({
  //     counter: counter.publicKey
  //   })
  //   .signers([counter])
  //   .rpc();
 
  //   const account  =  await program.account.counter.fetch(counter.publicKey)
  //   expect(account.count.toNumber()).to.equal(0);
  //   console.log("Your transaction signature", tx);
  //  })
  // it("Incremenet!", async () => {
 
  //   const tx = await program.methods.increment()
  //   .accounts({
  //     counter:counter.publicKey,
  //     user:provider.wallet.publicKey
  //   })
  //   .rpc();
  //   console.log("Your Incrment Transaction Signature:",tx);

  //   const account = await program.account.counter.fetch(counter.publicKey);
  //   expect(account.count.toNumber()).to.equal(1);
  // });
  // it("Decrement", async()=>{
  //   const tx = await program.methods.decrement()
  //   .accounts({
  //     counter:counter.publicKey,
  //     user:provider.wallet.publicKey
  //   })
  //   .rpc();
  //   console.log("Your Incrment Transaction Signature:",tx);
  //   const account = await program.account.counter.fetch(counter.publicKey);
  //   expect(account.count.toNumber()).to.equal(0);
  // })
  // const WALLET_PATH = join(process.env["HOME"]!, ".config/solana/id.json");
  // const WALLET_PATH = join(process.env["HOME"]!, ".config/solana/id.json");
  // const admin = anchor.web3.Keypair.fromSecretKey(
  //   // Buffer.from(JSON.parse(readFileSync(WALLET_PATH, { encoding: "utf-8" })))
  //   Uint8Array.from([48,70,85,200,158,55,53,169,210,165,251,162,95,216,112,43,57,24,51,65,8,190,29,7,99,149,223,254,105,196,160,57,202,243,115,97,23,228,176,194,202,120,229,31,196,187,150,90,226,102,173,155,253,216,81,151,134,238,90,138,142,29,112,206]),
  // );
  const admin = anchor.web3.Keypair.generate();
  const user = anchor.web3.Keypair.generate();
  // const userPubkey  ='8znyXSStvFpyYFAKXDsL9o11kjhAu4aqKvbcuaGZJEjK';

  // console.log(admin.publicKey,user.publicKey);
  const poolInfo = anchor.web3.Keypair.generate();
  const userInfo = anchor.web3.Keypair.generate();
  // let stakingToken:anchor.web3.PublicKey;
  let token: Token;
  let adminStakingWallet:anchor.web3.PublicKey;
  let userStakingWallet:anchor.web3.PublicKey;
// const connection = new Connection(clusterApiUrl('testnet'),'confirmed');
const connection = new Connection('http://127.0.0.1:8899','confirmed');
// it("Localhost airdrop",async()=>{
//   const myAddress = new PublicKey("By2ALVLq6avcheRivbRVqQeqJdAraK4cvikY1rcAcZ3Q");
//   const signature = await connection.requestAirdrop(myAddress, LAMPORTS_PER_SOL);
//   const latestBlockHash = await connection.getLatestBlockhash();
//       await connection.confirmTransaction({
//         blockhash: latestBlockHash.blockhash,
//         lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
//         signature: signature,
//       });

//       const userSignature = await connection.requestAirdrop(user.publicKey,10*LAMPORTS_PER_SOL);
//       const userLatestBlockHash = await connection.getLatestBlockhash();
//       await connection.confirmTransaction({
//         blockhash: userLatestBlockHash.blockhash,
//         lastValidBlockHeight: userLatestBlockHash.lastValidBlockHeight,
//         signature: userSignature
//       });
//       console.log("Account info:", await connection.getAccountInfo(user.publicKey))
// })
// it("account info", async()=>{
//   console.log("Account info: ",await connection.getAccountInfo(admin.publicKey))
// })

it("Balance: ", async()=>{
  // console.log("Balances: ", await connection.getBalance(admin.publicKey),await connection.getBalance(user.publicKey));
  console.log(await connection.getBalance(anchor.AnchorProvider.local().wallet.publicKey));//signer
  console.log(await connection.getBalance(new PublicKey('By2ALVLq6avcheRivbRVqQeqJdAraK4cvikY1rcAcZ3Q')));
})
//register a callback to listen the wallet(ws subscription)
// connection.onAccountChange(
//   user.publicKey,
//   (updatedAccountInfo, context)=>
//     console.log("Updatrd account info: ",updatedAccountInfo),
//   'confirmed'
// );
// console.log("hello");
// (async()=>{
//   console.log("block hash")
// const airdropSignature = await connection.requestAirdrop(
// user.publicKey,
// LAMPORTS_PER_SOL
// );
// const latestBlockHash = await connection.getLatestBlockhash();
//       await connection.confirmTransaction({
//         blockhash: latestBlockHash.blockhash,
//         lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
//         signature: airdropSignature,
//       });
// })

  it("Staking initialize",async()=>{
    //Airdropping
    // const myAddress = new PublicKey("By2ALVLq6avcheRivbRVqQeqJdAraK4cvikY1rcAcZ3Q");
    const signature = await connection.requestAirdrop(admin.publicKey,10* LAMPORTS_PER_SOL);
    const latestBlockHash = await connection.getLatestBlockhash();
        await connection.confirmTransaction({
          blockhash: latestBlockHash.blockhash,
          lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
          signature: signature,
        });
  console.log("Admin Account info",await connection.getAccountInfo(admin.publicKey));
        const userSignature = await connection.requestAirdrop(user.publicKey,10*LAMPORTS_PER_SOL);
        const userLatestBlockHash = await connection.getLatestBlockhash();
        await connection.confirmTransaction({
          blockhash: userLatestBlockHash.blockhash,
          lastValidBlockHeight: userLatestBlockHash.lastValidBlockHeight,
          signature: userSignature
        });
        console.log("User Account info:", await connection.getAccountInfo(user.publicKey))
  })

  })

