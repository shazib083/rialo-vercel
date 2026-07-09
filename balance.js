const { runCli } = require("../lib/runCli");

module.exports = async (req, res) => {
  if (req.method !== "POST") return res.status(405).json({ error: "POST only" });
  const { pubkey } = req.body || {};
  if (!pubkey) return res.status(400).json({ error: "pubkey required" });
  try {
    const result = await runCli(["balance", pubkey]);
    res.status(200).json(result);
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
};
