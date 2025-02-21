# Decentralized-Identity-DID-App-on-Solana
Decentralized Identity (DID) App on Solana

This is a basic Decentralized Identity (DID) that allows users to control their own identity without relying on centralized authorities, but using Solana blockchain to securely verify credentials for authentication, reputation systems, and access control really fast and avoiding long waits.


1) Elements of this DID system project:

* DID Registry (Rust Smart Contract) / this stores user identities on-chain.
* Public/Private Key Authentication / Uses wallets like Phantom to sign messages.
* Verifiable Credentials / these are the attestations proving user identity.

2) The Rust program on the decentralized_DID.rs smart contract is to register and verify a DID on Solana.

register_did(): Registers a user’s DID with a data field.

verify_did(): Ensures the user owns the DID before granting access.

DID Struct: Stores identity data on-chain.


3) Integrating DID contract with a Web3 Frontend:
   
* Install dependencies in a React project:   npm install @solana/web3.js @solana/wallet-adapter-react

* Modify DID_Dapp.tsx to register and verify a DID
