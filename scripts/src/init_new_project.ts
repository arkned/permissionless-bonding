/* eslint-disable @typescript-eslint/no-explicit-any */
import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";

import {
  tokenProgram,
  bondingProgramId,
  rent,
  systemProgram,
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
  const tokenAccount = "9AhcGWHk3z5ZtBuiAsxBiZJhCifZoGpQoRuUqQzCQd6H";
  const lpMint = new anchor.web3.PublicKey(
    "B4kdF3W8HWmQPB7tHoowNf9u9tX1vH84KgCLgtdGnYYT"
  );
  const lpTokenAccount = new anchor.web3.PublicKey(
    "42qqJi7BNRrTsnCWX1UnbMWSbbWqVkq4F6bChGP4sj3Q"
  );

  const [projectBonds] = await anchor.web3.PublicKey.findProgramAddress(
    [tokenMint.toBuffer(), Buffer.from("project-bonds")],
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
  const [projectInfo] = await anchor.web3.PublicKey.findProgramAddress(
    [
      tokenMint.toBuffer(),
      Buffer.from("project-info"),
      Buffer.from(projectBondId.toString()),
    ],
    bond.programId
  );

  const discountSettings = {
    minDiscount: new BN(100000000), // 10%
    maxDiscount: new BN(200000000),
    discountMode: new BN(0),
  };
  const vestingSchedule = {
    releaseInterval: new BN(60), // every minute
    releaseRate: new BN(1000), // 0.0001% every minute
    instantUnlock: new BN(100000000), // 10%
    initialUnlock: new BN(100000000), // 10%
    lockPeriod: new BN(600), // 10 mins
    vestingPeriod: new BN(86400), // 1 day
  };

  const amount = new BN(1000000000000000);
  const price = new BN(2000000000);

  await bond.rpc.initNewProject(
    amount,
    price,
    discountSettings,
    vestingSchedule,
    {
      accounts: {
        initializer: initializer.publicKey,
        tokenMint,
        tokenAccount,
        lpMint,
        lpTokenAccount,
        projectBonds,
        vaultAccount,
        projectInfo,
        rent,
        systemProgram,
        tokenProgram,
      },
      signers: [initializer],
    }
  );
}

main().then().catch(console.log);
