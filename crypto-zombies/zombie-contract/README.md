# Crypto Zombies Project

A solidity smart contract for the Ethereum based zombie game.

- Main attraction is the random traits for every new Zombie created via random DNA generation. surprise element when Zombies feeded with cryptokitties.
- Ownership of zombie and quantity is stored in Contract as permanent storage in Blockchain. Ownership is mapped through the identity ethereum 16 Bit hexadecimal address.
- Public function in contract for the zombie creation and couples of private method defined.
- Contract safe from overflow situations.
- Zombies can attack, can be level up.
- Higher the level, higher will be the fee.
- Can be easily pluggable to any standard Crypto Platform as it the Contract works on the ERC721 Standards.

## Built with
- [Solidity is an object-oriented, high-level language for implementing smart contracts.](https://docs.soliditylang.org/en/v0.8.16/)
- [Hardhat is a development environment for Ethereum software](https://hardhat.org/)

### Compile
```shell
yarn complie
```
### Deploy
```shell
yarn deploy bsctest
```
### Verify
```shell
yarn verify bsctest
```
