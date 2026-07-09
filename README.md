# Rialo Testnet Console — Vercel

Same functionality as the Docker/Railway version — wallet create, balance,
airdrop, transfer against `https://testnet.rialo.io/` — rebuilt for
Vercel's serverless model. Nothing about the actual wallet logic changed;
only how it's hosted.

```
rialo-vercel/
├── .rialo-tester/         Rust CLI (compiled by Vercel at build time)
│   ├── Cargo.toml
│   └── src/main.rs
├── api/                    Serverless functions (routes match the URL path)
│   ├── wallet/create.js
│   ├── balance.js
│   ├── airdrop.js
│   └── transfer.js
├── lib/runCli.js           Shared: spawns the compiled binary, parses JSON
├── index.html               Frontend (served automatically, static root)
├── build-rust.sh            Compiles the Rust CLI for Linux during deploy
├── package.json
├── vercel.json               Bundles the compiled binary with each function
└── .gitignore
```

## Why this version is simpler than my first Vercel attempt

The wallet logic itself changed since then: there's no more named/password
wallets saved anywhere. Every call is now self-contained — create returns
a pubkey + private key + mnemonic in one shot, balance/airdrop only need a
pubkey, and transfer takes a private key directly in the request. That
means there's nothing to persist between requests, so this version needs
**no database, no Redis, no volume** — just the compiled binary sitting
next to each function.

## 1. Push to git

```bash
git init
git add .
git status   # confirm bin/, node_modules/, .rialo-tester/target/ are NOT staged
git commit -m "Rialo testnet console (Vercel)"
git remote add origin https://github.com/<you>/<repo>.git
git branch -M main
git push -u origin main
```

## 2. Deploy

1. https://vercel.com/new → Import this GitHub repo.
2. Vercel automatically runs the `vercel-build` script from `package.json`,
   which runs `build-rust.sh` — this installs Rust and compiles the CLI
   for Linux before the functions are bundled. No manual cross-compilation
   needed on your end.
3. Deploy. You get a public `https://<project>.vercel.app` URL with HTTPS
   already handled.

## The one thing I still can't fully guarantee

The binary is compiled on Vercel's Linux build machines and then run on
Vercel's Linux Lambda runtime. These are normally compatible, but if a
dependency links against a system library version that differs between
build and runtime images, the function can fail at invocation even though
the build succeeded — this is exactly the class of error you hit last
time (`ENOENT` was a different problem — the binary wasn't there at all,
because the previous project had no build step for it. This version's
`vercel-build` script fixes that specific issue). If you see a *different*
runtime error this time (not `ENOENT`), check the function logs in the
Vercel dashboard and paste me the exact message.

## Known limitations

- **No rate limiting.** Anyone with the link can spam airdrop/transfer.
- **Private keys travel over the network per-request** for transfers —
  make sure the deployed URL is HTTPS (Vercel gives you this by default).
- **Testnet only**, as always.
