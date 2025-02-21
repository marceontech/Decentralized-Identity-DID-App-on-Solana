import { useWallet } from "@solana/wallet-adapter-react";
import { Program, Provider, web3 } from "@project-serum/anchor";
import idl from "./idl.json";

const programId = new web3.PublicKey("FILL_WITH_YOUR_PROGRAM_ID");

const App = () => {
  const { connection } = useConnection();
  const { publicKey, sendTransaction } = useWallet();

  const registerDID = async () => {
    if (!publicKey) return;

    const provider = new Provider(connection, window.solana, "processed");
    const program = new Program(idl, programId, provider);

    await program.rpc.registerDid("My Decentralized ID");
    console.log("DID Registered!");
  };

  return (
    <div>
      <button onClick={registerDID}>Register DID</button>
    </div>
  );
};

export default App;
