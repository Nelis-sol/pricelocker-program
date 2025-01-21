import { useState } from 'react';
import * as anchor from '@coral-xyz/anchor';
import { Connection, PublicKey } from '@solana/web3.js';
import './App.css';
import idl from '../price_lock.json';
import { useAnchorWallet } from '@solana/wallet-adapter-react';
import { Program } from '@coral-xyz/anchor';
import {
  TOKEN_PROGRAM_ID,
  getMint,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  getAccount,
  getAssociatedTokenAddressSync
} from "@solana/spl-token";

function App() {
  const [status, setStatus] = useState("");

  // Set up the provider and program
  const wallet = useAnchorWallet();
  const connection = new Connection('https://api.devnet.solana.com');

  if (!wallet) {
    return <div>Wallet not connected</div>;
  }

  const provider = new anchor.AnchorProvider(connection, wallet, {});
  const programId = new PublicKey("BPVWMMj1eEALBvsR3TQZC2Zb3vt8jQkvzBYeJsoaKum7"); // Replace with your actual program ID
  const program = new Program(idl as unknown as anchor.Idl, programId, provider);

  const mint = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

  const payer = wallet;
  const authority = payer.publicKey;

  const priceUpdate = "7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE";

  const lockerName = "MyNewLocker112";
  const lockerPda = PublicKey.findProgramAddressSync(
    [
      Buffer.from('locker'), 
      payer.publicKey.toBuffer(), 
      Buffer.from(lockerName)
    ], 
      program.programId
    )[0];

  

  const handleCreateNewLocker = async () => {

    // Derive the associated token account
    const lockerTokenAccount = getAssociatedTokenAddressSync(
      mint,
      lockerPda,
      true
    );

    console.log("lockerPda", lockerPda.toBase58());
    console.log("lockerTokenAccount", lockerTokenAccount.toBase58());
    console.log("mint", mint.toBase58());
    console.log("payer", payer.publicKey.toBase58());


    try {
      await program.methods
        .createNewLocker(lockerName)
        .accounts({
          authority: payer.publicKey,
          lockerPda,
          lockerTokenAccount,
          mint,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc({ skipPreflight: true });

      setStatus(`New locker '${lockerName}' created successfully`);
    } catch (error) {
      setStatus(`Error: ${error}`);
    }
  };

  const handleDepositFunds = async () => {

    const authorityTokenAccount = getAssociatedTokenAddressSync(
      mint,
      authority,
      false
    );

    // Derive the associated token account
    const lockerTokenAccount = getAssociatedTokenAddressSync(
      mint,
      lockerPda,
      true
    );

    try {
      const amount = new anchor.BN(10000);

      console.log("authority: ", payer.publicKey.toBase58());

      await program.methods
        .depositNewFunds(lockerName, amount)
        .accounts({
          authority: payer.publicKey,
          authorityTokenAccount,
          lockerPda,
          lockerTokenAccount,
          mint,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc({ skipPreflight: true });

      setStatus(`Deposited ${amount} lamports into locker '${lockerName}' successfully`);
    } catch (error) {
      setStatus(`Error: ${error}`);
    }
  };

  const handleTimeLockFunds = async () => {

    const mint = (await getMint(connection, new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"))).address;

    // Derive the associated token account
    const lockerTokenAccount = await getAssociatedTokenAddress(
      mint,
      payer.publicKey
    );


    try {
      const strikeTime = new anchor.BN(Math.floor(Date.now() / 1000) + 60); // Lock for 1 hour
      const amount = new anchor.BN(100); // Example amount

      await program.methods
        .timeLockFunds(strikeTime, amount, null, null)
        .accounts({
          authority: payer.publicKey,
          lockerPda,
          lockerTokenAccount,
          mint,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc({ skipPreflight: true });

      setStatus(`Time lock added to locker '${lockerName}' for amount ${amount}`);
    } catch (error) {
      setStatus(`Error: ${error}`);
    }
  };

  const handleTimeUnlockFunds = async () => {

    const mint = (await getMint(connection, new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"))).address;

    // Derive the associated token account
    const lockerTokenAccount = await getAssociatedTokenAddress(
      mint,
      payer.publicKey
    );

    try {
      const lockIndex = 0;

      await program.methods
        .timeUnlockFunds(lockIndex)
        .accounts({
          authority: payer.publicKey,
          lockerPda,
          lockerTokenAccount,
          mint,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc({ skipPreflight: true });

      setStatus(`Unlocked time lock at index ${lockIndex} in locker '${lockerName}'`);
    } catch (error) {
      setStatus(`Error: ${error}`);
    }
  };

  const handlePriceLockFunds = async () => {

    const mint = (await getMint(connection, new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"))).address;

    // Derive the associated token account
    const lockerTokenAccount = await getAssociatedTokenAddress(
      mint,
      payer.publicKey
    );


    try {
      const strikePrice = 10;
      const amount = new anchor.BN(100);

      await program.methods
        .priceLockFunds(strikePrice, amount, null, null)
        .accounts({
          authority: payer.publicKey,
          lockerPda,
          lockerTokenAccount,
          mint,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc({ skipPreflight: true });

      setStatus(`Time lock added to locker '${lockerName}' for amount ${amount}`);
    } catch (error) {
      setStatus(`Error: ${error}`);
    }
  };

  const handlePriceUnlockFunds = async () => {

    const mint = (await getMint(connection, new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"))).address;

    // Derive the associated token account
    const lockerTokenAccount = await getAssociatedTokenAddress(
      mint,
      payer.publicKey
    );


    try {
      const lockIndex = 0;

      await program.methods
        .priceUnlockFunds(lockIndex)
        .accounts({
          authority: payer.publicKey,
          lockerPda,
          lockerTokenAccount,
          mint,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc({ skipPreflight: true });

      setStatus(`Unlocked time lock at index ${lockIndex} in locker '${lockerName}'`);
    } catch (error) {
      setStatus(`Error: ${error}`);
    }
  };

  const handleWithdrawUnlockedFunds = async () => {

    const mint = (await getMint(connection, new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"))).address;

    // Derive the associated token account
    const lockerTokenAccount = await getAssociatedTokenAddress(
      mint,
      payer.publicKey
    );


    try {
      const amount = new anchor.BN(200); // Example amount

      await program.methods
        .withdrawUnlockedFunds(amount)
        .accountsStrict({
          authority: payer.publicKey,
          lockerPda,
          lockerTokenAccount,
          mint,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc({ skipPreflight: true });

      setStatus(`Withdrawn ${amount} lamports from locker '${lockerName}'`);
    } catch (error) {
      setStatus(`Error: ${error}`);
    }
  };

  return (
    <>
      <div>
        <h1>Price Locker</h1>
        <div className="card">
          <button onClick={handleCreateNewLocker}>Create New Locker</button>
          <button onClick={handleDepositFunds}>Deposit Funds</button>
          <button onClick={handleTimeLockFunds}>Time Lock Funds</button>
          <button onClick={handleTimeUnlockFunds}>Time Unlock Funds</button>
          <button onClick={handlePriceLockFunds}>Price Lock Funds</button>
          <button onClick={handlePriceUnlockFunds}>Price Unlock Funds</button>
          <button onClick={handleWithdrawUnlockedFunds}>Withdraw Unlocked Funds</button>
        </div>
        <p>Status: {status}</p>
      </div>
    </>
  );
}

export default App;
