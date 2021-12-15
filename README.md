# goki-cli

![Banner](images/banner.jpeg)

CLI for the [Goki](https://goki.so) Smart Wallet system.

## Installation

First, make sure you have Solana installed. [Follow the instructions here.](https://docs.solana.com/cli/install-solana-cli-tools)

Next, install Goki via Cargo like so:

```
cargo install --git https://github.com/GokiProtocol/goki-cli --locked
```

## Usage

### Setup

Go to any directory and run the following command:

```
goki init
```

This will create a `.goki` directory, which you should add to your `.gitignore`.

The `.goki` directory contains keypairs that will contain the SOL you use for program deployment. You may want to back up this folder via an encrypted filestore such as [Keybase](https://keybase.io/). You should not be storing any sensitive funds in this wallet-- **only use this for program deploys.**

### Upgrading a Program

To upgrade any existing program on Solana, run `goki upload-program-buffer`.

```
Uploads a Solana program buffer.

USAGE:
    goki upload-program-buffer [OPTIONS] --location <LOCATION> --program-id <PROGRAM_ID>

OPTIONS:
    -c, --cluster <CLUSTER>          Cluster to deploy to. Defaults to devnet. [default: devnet]
    -h, --help                       Print help information
    -l, --location <LOCATION>        The path to the Solana program buffer.
    -p, --program-id <PROGRAM_ID>    The program being upgraded. The buffer authority will be the
```

For example, let's say you wanted to upgrade the [Goki Token Signer program](https://crates.io/crates/token-signer) on mainnet. You would run the following command:

```
goki upload-program-buffer --cluster mainnet --location gh:token_signer:GokiProtocol/goki@0.5.2 --program-id NFTUJzSHuUCsMMqMRJpB7PmbsaU7Wm51acdPk2FXMLn
```

If the command is successful, you should now have a buffer of the Goki Token Signer program at release v0.5.2 deployed somewhere on mainnet, owned by the current upgrade authority of the Goki program. The upgrade authority (ideally a Goki Smart Wallet) would then be able to upgrade their program's bytecode to the contents of that uploaded buffer.

If you don't have enough SOL in your wallet, the command will fail and tell you what key you should be sending SOL to.

#### Location

There are three formats of `location` that you may specify:

- a `.so` artifact of a GitHub release, for example `gh:smart_wallet:GokiProtocol/goki@0.5.2`
- a URL, for example `https://github.com/GokiProtocol/goki/releases/download/v0.5.2/smart_wallet.so`
- a file path, for example `./target/deploy/smart_wallet.so`.

## License

AGPL-3.0
