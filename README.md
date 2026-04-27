# Coincli

Minimal price search CLI for cryptocurrencies, built with Rust.

`coincli` prompts for a cryptocurrency name, fetches live price data from CoinGecko, and prints the result in:

- `USD`
- `COP`

## Run

```bash
cargo run
```

Or with Nix:

```bash
nix run
```

## Nix

```bash
nix develop
nix run
nix build
nix flake check
```

- `nix develop`: opens the Rust dev shell
- `nix run`: runs `coincli`
- `nix build`: builds the package
- `nix flake check`: runs the flake checks

If you use `direnv`, this repo includes `.envrc` with `use flake`, so you only need:

```bash
direnv allow
```
