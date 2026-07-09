const { execFile } = require("child_process");
const fs = require("fs");
const path = require("path");

const BIN = path.join(process.cwd(), "bin", "rialo-tester");

function runCli(args) {
  return new Promise((resolve, reject) => {
    try {
      fs.chmodSync(BIN, 0o755);
    } catch (_) {
      /* already executable, or missing — execFile below surfaces the real error */
    }
    execFile(BIN, ["--json", ...args], { timeout: 25_000 }, (err, stdout, stderr) => {
      if (stdout && stdout.trim().length > 0) {
        try {
          const parsed = JSON.parse(stdout.trim().split("\n").pop());
          if (parsed.ok === false) return reject(new Error(parsed.error || "Unknown error"));
          return resolve(parsed);
        } catch (_) {
          return reject(new Error(`Could not parse CLI output: ${stdout}`));
        }
      }
      reject(new Error(stderr || err?.message || "Unknown CLI failure"));
    });
  });
}

module.exports = { runCli };
