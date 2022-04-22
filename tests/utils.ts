import * as anchor from "@project-serum/anchor";
import {
  TOKEN_PROGRAM_ID,
  Token,
  MintLayout,
  AccountLayout,
} from "@solana/spl-token";

export const rent = anchor.web3.SYSVAR_RENT_PUBKEY;
export const clock = anchor.web3.SYSVAR_CLOCK_PUBKEY;
export const systemProgram = anchor.web3.SystemProgram.programId;
export const tokenProgram = TOKEN_PROGRAM_ID;

export const getTokenBalance = async (provider, pubkey) => {
  return parseInt(
    (await provider.connection.getTokenAccountBalance(pubkey)).value.amount
  );
};

export const createRandomMint = async (provider, decimals) => {
  const mint = await Token.createMint(
    provider.connection,
    provider.wallet.payer,
    provider.wallet.publicKey,
    null,
    decimals,
    TOKEN_PROGRAM_ID
  );
  return mint;
};

export const createTokenAccount = async (provider, mint, owner) => {
  const tokenAccount = anchor.web3.Keypair.generate();
  const tx = new anchor.web3.Transaction();
  const balanceNeeded = await Token.getMinBalanceRentForExemptAccount(
    provider.connection
  );
  tx.add(
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.payer.publicKey,
      newAccountPubkey: tokenAccount.publicKey,
      lamports: balanceNeeded,
      space: AccountLayout.span,
      programId: TOKEN_PROGRAM_ID,
    })
  );
  tx.add(
    Token.createInitAccountInstruction(
      TOKEN_PROGRAM_ID,
      mint,
      tokenAccount.publicKey,
      owner
    )
  );
  await provider.send(tx, [tokenAccount]);
  return tokenAccount.publicKey;
};

export const mintToAccount = async (provider, mint, destination, amount) => {
  const tx = new anchor.web3.Transaction();
  tx.add(
    Token.createMintToInstruction(
      TOKEN_PROGRAM_ID,
      mint,
      destination,
      provider.wallet.publicKey,
      [],
      amount
    )
  );
  await provider.send(tx);
};

export const sendLamports = async (provider, destination, amount) => {
  const tx = new anchor.web3.Transaction();
  tx.add(
    anchor.web3.SystemProgram.transfer({
      fromPubkey: provider.wallet.publicKey,
      lamports: amount,
      toPubkey: destination,
    })
  );
  await provider.send(tx);
};

export const createMint = async (
  mintAccount,
  provider,
  mintAuthority,
  freezeAuthority,
  decimals,
  programId
) => {
  const token = new Token(
    provider.connection,
    mintAccount.publicKey,
    programId,
    provider.wallet.payer
  );

  // Allocate memory for the account
  const balanceNeeded = await Token.getMinBalanceRentForExemptMint(
    provider.connection
  );

  const transaction = new anchor.web3.Transaction();
  transaction.add(
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.payer.publicKey,
      newAccountPubkey: mintAccount.publicKey,
      lamports: balanceNeeded,
      space: MintLayout.span,
      programId,
    })
  );

  transaction.add(
    Token.createInitMintInstruction(
      programId,
      mintAccount.publicKey,
      decimals,
      mintAuthority,
      freezeAuthority
    )
  );

  await provider.send(transaction, [mintAccount]);
  return token;
};
