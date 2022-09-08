# Findora Scanner

A scanner for findora. This tool will generate all data and cache into db.
## Dependencies
```
# ubuntu
sudo apt install -y postgresql redis-server
```
## Usage of Scanner.

Before use scanner, you should set environment variable `DATABASE_URL` correctly. The format of it is `postgres://<Owner>:<Password>@<Host>/<DatabaseName>`.

Set environment variable `RUST_LOG=scanner=info` to show logs.

### Scan a single block.

```
scanner-cli load -s https://prod-mainnet.prod.findora.org:26657/ --height <int> 
```

### Scan blocks in a range.

```
scanner-cli scan -s https://prod-mainnet.prod.findora.org:26657/ --start <int> --end <int>
```

### Periodically scan a block

```
scanner-cli subscribe -s https://prod-mainnet.prod.findora.org:26657/
```

start height is loaded from database, or specified by `--start <int>`

## Explorer Service

## Wallet Service

