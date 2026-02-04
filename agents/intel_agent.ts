import crypto from "crypto"

export function analyzeInsight(raw: string) {
  const hash = crypto.createHash("sha256").update(raw).digest()

  const score =
    Math.floor(
      (Math.random() * 0.6 + Math.random() * 0.4) * 100
    )

  return { hash, score }
}
