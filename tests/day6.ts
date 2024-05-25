import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day6 } from "../target/types/day6";
import { expect } from "chai";

describe("day6", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)


  const program = anchor.workspace.Day6 as Program<Day6>;
const counter =  anchor.web3.Keypair.generate();
const signer1= anchor.web3.Keypair.generate();
   // Add your test here.
   
   it("Initialize",async()=>{
    const tx = await program.methods.initialize()
    .accounts({
      counter: counter.publicKey
    })
    .signers([counter])
    .rpc();
 
    const account  =  await program.account.counter.fetch(counter.publicKey)
    expect(account.count.toNumber()).to.equal(0);
    console.log("Your transaction signature", tx);
   })
  it("Incremenet!", async () => {
 
    const tx = await program.methods.increment()
    .accounts({
      counter:counter.publicKey,
      user:provider.wallet.publicKey
    })
    .rpc();
    console.log("Your Incrment Transaction Signature:",tx);

    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(1);
  });
  it("Decrement", async()=>{
    const tx = await program.methods.decrement()
    .accounts({
      counter:counter.publicKey,
      user:provider.wallet.publicKey
    })
    .rpc();
    console.log("Your Incrment Transaction Signature:",tx);
    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(0);
  })

});
