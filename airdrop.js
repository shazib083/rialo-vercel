const { runCli } = require("../lib/runCli");

module.exports = async (req, res) => {
  if (req.method !== "POST") return res.status(405).json({ error: "POST only" });
  const { pubkey, amount } = req.body || {};
  if (!pubkey || !amount) return res.status(400).json({ error: "pubkey and amount required" });
  try {
    const result = await runCli(["airdrop", pubkey, String(amount)]);
    res.status(200).json(result);
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
};
