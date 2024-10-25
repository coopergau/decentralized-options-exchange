import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { Options } from "../target/types/options";

describe("options", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Options as Program<Options>;

  const counter = anchor.web3.Keypair.generate();
  const option1 = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({ optionCounter: counter.publicKey })
      .signers([counter])
      .rpc();

    const account = await program.account.optionCounter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(0);
  });

  it("Option listed", async () => {
    const strikePrice = 100;
    const expiryDateAndTime = Math.floor(Date.now() / 1000) + 3600; // 1 hour from now
    const isCall = true;
    const optionPrice = 10;

    const tx = await program.methods
      .listOption(
        new anchor.BN(strikePrice),
        new anchor.BN(expiryDateAndTime),
        isCall,
        new anchor.BN(optionPrice))
      .accounts({ optionData: option1.publicKey, optionCounter: counter.publicKey, user: provider.wallet.publicKey })
      .signers([option1])
      .rpc();

    const account = await program.account.optionCounter.fetch(counter.publicKey);
    console.log("Transaction signature:", tx);
    const optionAccount = await program.account.optionData.fetch(option1.publicKey);
    console.log("Option Data:", {
      strikePrice: optionAccount.strikePrice.toNumber(),
      expiryDateAndTime: optionAccount.expiryDateAndTime.toNumber(),
      isCall: optionAccount.isCall,
      optionPrice: optionAccount.optionPrice.toNumber(),
    });
    expect(account.count.toNumber()).to.equal(1);
  });
});