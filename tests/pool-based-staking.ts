import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PoolBasedStaking } from "../target/types/pool_based_staking";

describe("pool-based-staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PoolBasedStaking as Program<PoolBasedStaking>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
