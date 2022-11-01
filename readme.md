# agnesi

Blockchain prototyping.

## Development

### Dependencies 
- Rust 1.64.0-x86_64-apple-darwin.
- [ganache](https://trufflesuite.com/ganache/). 
- [solidity](https://docs.soliditylang.org/en/v0.8.17/installing-solidity.html)
- [truffle](https://trufflesuite.com/docs/truffle/)

Run ganache (personal Ethereum blockchain).

### Interact with deployed smart contracts

Would be nice to do it through our rust app :)

```bash
cargo run --release
```

### Deploying smart contracts

To compile:
```bash
truffle compile
```

To test:
```bash
truffle test
```

To deploy:
```bash
truffle migrate
```

