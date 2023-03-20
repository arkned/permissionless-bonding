/* eslint-disable @typescript-eslint/no-explicit-any */
import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";

import {
  tokenProgram,
  bondingProgramId,
  systemProgram,
  clock,
} from "./constant";

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

  const tokenMint = new anchor.web3.PublicKey(
    "GCSHY7hLv3Y5qvnkbLKQLMpNYJQTYqXKNkxgWDMBtMzt"
  );
  const lpTokenAccount = new anchor.web3.PublicKey(
    "42qqJi7BNRrTsnCWX1UnbMWSbbWqVkq4F6bChGP4sj3Q"
  );

  const [projectInfo] = await anchor.web3.PublicKey.findProgramAddress(
    [
      tokenMint.toBuffer(),
      Buffer.from("project-info"),
      Buffer.from(projectBondId.toString()),
    ],
    bond.programId
  );
  const [bondsInfo] = await anchor.web3.PublicKey.findProgramAddress(
    [
      tokenMint.toBuffer(),
      initializer.publicKey.toBuffer(),
      Buffer.from("bonds-info"),
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
  const projectInfoData = await bond.account.projectInfo.fetch(projectInfo);

  const lpAmount = new BN(10000000000);

  const res = await bond.account.vestingInfo.fetch(vestingInfo);
  console.log(res.totalAmount.toString());

  await bond.rpc.bond(new BN(projectBondId), lpAmount, {
    accounts: {
      user: initializer.publicKey,
      lpMint: projectInfoData.lpToken,
      lpDepositAccount: lpTokenAccount,
      lpRecieveAccount: projectInfoData.lpTokenAccount,
      tokenMint,
      projectInfo,
      bondsInfo,
      vestingInfo,
      systemProgram,
      tokenProgram,
      clock,
    },
    signers: [initializer],
  });
}

main().then().catch(console.log);
