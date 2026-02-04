import { Program } from "@coral-xyz/anchor"

export async function submitInsight(
  program: Program,
  insightAccount: any,
  hash: Buffer
) {
  return program.methods
    .submitInsight([...hash])
    .accounts({
      insight: insightAccount.publicKey,
    })
    .rpc()
}
