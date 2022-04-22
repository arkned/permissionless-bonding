import * as anchor from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

export const rent = anchor.web3.SYSVAR_RENT_PUBKEY;
export const clock = anchor.web3.SYSVAR_CLOCK_PUBKEY;
export const systemProgram = anchor.web3.SystemProgram.programId;
export const tokenProgram = TOKEN_PROGRAM_ID;

export const bondingProgramId = "B3HQ6SpGbgBD9ANG9dsFLxa3HWWkzdvYeFyy9TKbqrpF";
