/* eslint-disable @typescript-eslint/no-explicit-any */
import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";

import { tokenProgram, bondingProgramId, clock } from "./constant";

import { Bond } from "./types/bond";
import BondIdl from "./idl/bond.json";
import deployerKey from "./keys/deployer.json";

async function main() {
  const connection = new anchor.web3.Connection(
    // "https://api.mainnet-beta.solana.com",
    "https://api.devnet.solana.com",
    {
      confirmTransactionInitialTimeout: 120000,
    }
  );
  const initializer = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from(deployerKey)
  );
  const wallet = new anchor.Wallet(initializer);
  const provider = new anchor.Provider(connection, wallet, {
    preflightCommitment: "recent",
    commitment: "recent",
  });
  anchor.setProvider(provider);

  const bond = new anchor.Program(
    BondIdl as any,
    bondingProgramId
  ) as Program<Bond>;

  const projectBondId = 1;
  const bondId = 0;

  const tokenMint = new anchor.web3.PublicKey(
    "GCSHY7hLv3Y5qvnkbLKQLMpNYJQTYqXKNkxgWDMBtMzt"
  );
  const tokenAccount = "9AhcGWHk3z5ZtBuiAsxBiZJhCifZoGpQoRuUqQzCQd6H";

  const [projectInfo] = await anchor.web3.PublicKey.findProgramAddress(
    [
      tokenMint.toBuffer(),
      Buffer.from("project-info"),
      Buffer.from(projectBondId.toString()),
    ],
    bond.programId
  );
  const totalBonds = 0;
  const [vestingInfo] = await anchor.web3.PublicKey.findProgramAddress(
    [
      tokenMint.toBuffer(),
      initializer.publicKey.toBuffer(),
      Buffer.from("vesting-info"),
      Buffer.from(totalBonds.toString()),
    ],
    bond.programId
  );
  const [vaultAccount] = await anchor.web3.PublicKey.findProgramAddress(
    [
      tokenMint.toBuffer(),
      Buffer.from("token-vault"),
      Buffer.from(projectBondId.toString()),
    ],
    bond.programId
  );

  const accounts = {
    taker: initializer.publicKey,
    takerReceiveTokenAccount: tokenAccount,
    vaultAccount,
    projectInfo,
    vestingInfo,
    tokenProgram,
    clock,
  };
  for (const [key, value] of Object.entries(accounts)) {
    console.log(`${key}: ${value}`);
  }
  await bond.rpc.withdrawVesting(new BN(projectBondId), new BN(bondId), {
    accounts: {
      taker: initializer.publicKey,
      takerReceiveTokenAccount: tokenAccount,
      vaultAccount,
      projectInfo,
      vestingInfo,
      tokenProgram,
      clock,
    },
    signers: [initializer],
  });
}

main().then().catch(console.log);
