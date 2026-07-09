const { runCli } = require("../lib/runCli");

module.exports = async (req, res) => {
  if (req.method !== "POST") return res.status(405).json({ error: "POST only" });
  const { secret, toPubkey, amount } = req.body || {};
  if (!secret || !toPubkey || !amount)
    return res.status(400).json({ error: "secret, toPubkey, and amount required" });
  try {
    const result = await runCli(["transfer", secret, toPubkey, String(amount)]);
    res.status(200).json(result);
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
};
