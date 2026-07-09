# Rialo Testnet Console — Vercel

Same functionality as the Docker/Railway version — wallet create, balance,
airdrop, transfer against `https://testnet.rialo.io/` — rebuilt for
Vercel's serverless model. Nothing about the actual wallet logic changed;
only how it's hosted and how the binary gets built.

```
rialo-vercel/
├── .github/workflows/build-binary.yml   Compiles the Rust CLI on every push
├── .rialo-tester/                       Rust CLI source
│   ├── Cargo.toml
│   └── src/main.rs
├── bin/rialo-tester                      Compiled binary (committed by CI)
├── api/                                   Serverless functions
│   ├── wallet/create.js
│   ├── balance.js
│   ├── airdrop.js
│   └── transfer.js
├── lib/runCli.js                          Shared: spawns the binary, parses JSON
├── index.html                              Frontend (served automatically)
├── package.json
├── vercel.json                              Bundles bin/ with each function
└── .gitignore
```

## Why the binary is built in GitHub Actions, not in Vercel

The first attempt tried compiling Rust directly inside Vercel's build step
(`curl ... sh.rustup.org | sh`). That failed with `Could not resolve host:
sh.rustup.org` — Vercel's build sandbox allows the network access it needs
for `npm`/git, but isn't a reliable place to install an arbitrary
third-party toolchain from scratch on every deploy.

The fix: a GitHub Actions workflow (`.github/workflows/build-binary.yml`)
compiles the Linux binary — GitHub's runners have full, unrestricted
network access — and commits it straight into `bin/rialo-tester` in the
repo. Vercel's build step then does nothing but `chmod +x` a file that's
already there. No network dependency at deploy time at all.

## 1. First-time setup

```bash
git add .
git commit -m "Switch to CI-built binary instead of building inside Vercel"
git push
```

This push triggers the GitHub Actions workflow automatically (check the
**Actions** tab on your GitHub repo — it should show a run in progress).
Wait for it to finish (a minute or two) — it'll push a second commit
containing the compiled `bin/rialo-tester`.

## 2. Deploy

Once that Actions run finishes and `bin/rialo-tester` exists in the repo,
either:
- Trigger a redeploy from the Vercel dashboard (Deployments → the failed
  one → "Redeploy"), or
- Just push any small change — Vercel redeploys automatically on push.

You should get a public `https://<project>.vercel.app` URL. Test **Create
Wallet** first.

## If the GitHub Actions build itself fails

Check the **Actions** tab on your repo for the specific error — this
environment is a completely standard Ubuntu runner with normal internet
access, so failures here would point to something in `main.rs`/`Cargo.toml`
rather than a networking/sandboxing issue like before.

## Known limitations

- **No rate limiting.** Anyone with the link can spam airdrop/transfer.
- **Private keys travel over the network per-request** for transfers —
  Vercel gives you HTTPS by default, so this is fine as deployed.
- **Testnet only**, as always.
- **The binary needs rebuilding (via a push to `.rialo-tester/**`) any
  time `main.rs` changes** — the GitHub Actions workflow only triggers on
  changes to that folder, by design, so it doesn't rebuild on every
  unrelated commit.
