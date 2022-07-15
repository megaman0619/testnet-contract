import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DemoProject } from "../target/types/demo_project";

describe("demo_project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DemoProject as Program<DemoProject>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
