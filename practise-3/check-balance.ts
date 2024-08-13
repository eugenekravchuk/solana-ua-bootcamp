import "dotenv/config";
import {
    Connection,
    LAMPORTS_PER_SOL,
    PublicKey,
    clusterApiUrl
} from "@solana/web3.js";

const connection = new Connection(clusterApiUrl("devnet"));
console.log(`⚡️ Connected to devnet`);

const publicKey = new PublicKey("3s2Lffxm1cAMepju8uQAasX3fTMmoqKXuYaXAYDJwCeP")
const balanceInLamports = await connection.getBalance(publicKey);

const balanceInSol = balanceInLamports / LAMPORTS_PER_SOL;

console.log(`The balance of the wallet at address ${publicKey} is: ${balanceInSol}`)