./target/release/node-template key generate --scheme Sr25519
# Aura
Secret phrase:       airport sadness tumble universe attract essence sight outside rich rack mass naive
  Network ID:        substrate
  Secret seed:       0x6683666a733ae655cb18b7a07645bd4b34d67945edb64f7e1aa3bc5f569cf653
  Public key (hex):  0x0461cf1741a4c9ed2e84c73505220d2e331b37253ba7892c1d528d205a3e7145
  Account ID:        0x0461cf1741a4c9ed2e84c73505220d2e331b37253ba7892c1d528d205a3e7145
  Public key (SS58): 5CAT6zCNL62xrPgdcTmk6enfLkBni56aEiSuPCaGJhhvouZh
  SS58 Address:      5CAT6zCNL62xrPgdcTmk6enfLkBni56aEiSuPCaGJhhvouZh

./target/release/node-template key inspect --scheme Ed25519 "airport sadness tumble universe attract essence sight outside rich rack mass naive”
./target/release/node-template key inspect --password-interactive --scheme Ed25519 "airport sadness tumble universe attract essence sight outside rich rack mass naive"
# Ganpa
Secret phrase:       airport sadness tumble universe attract essence sight outside rich rack mass naive
  Network ID:        substrate
  Secret seed:       0x6683666a733ae655cb18b7a07645bd4b34d67945edb64f7e1aa3bc5f569cf653
  Public key (hex):  0x12bd96f0f5b8a804a4e28af30fcc111beeb6428dbac55a67ea82da62789d9f29
  Account ID:        0x12bd96f0f5b8a804a4e28af30fcc111beeb6428dbac55a67ea82da62789d9f29
  Public key (SS58): 5CVH3DL6JTHQqsNrnjRmznF86AxNF3WxwdNMp9rMQV3CPNQG
  SS58 Address:      5CVH3DL6JTHQqsNrnjRmznF86AxNF3WxwdNMp9rMQV3CPNQG
  


################################################################################

# Node 2

# Aura
./target/release/node-template key generate --scheme Sr25519
Secret phrase:       forest injury struggle drama mango couch festival cheese unveil half stone gas
  Network ID:        substrate
  Secret seed:       0xf851acecf63afb95a75b88461b2f97a1a6b3e20386b941dbea58b7e8f2da35db
  Public key (hex):  0x783fb1ddb700c264d67f14aaa886c2982ba49862bbd7d860bdbcac078d2cf740
  Account ID:        0x783fb1ddb700c264d67f14aaa886c2982ba49862bbd7d860bdbcac078d2cf740
  Public key (SS58): 5EnNXP7BxejDr9S1HCLpo89kmbNgXEPNqZ8tXgtBYY2U16FB
  SS58 Address:      5EnNXP7BxejDr9S1HCLpo89kmbNgXEPNqZ8tXgtBYY2U16FB


./target/release/node-template key inspect --password-interactive --scheme Ed25519 "forest injury struggle drama mango couch festival cheese unveil half stone gas"
Secret phrase:       forest injury struggle drama mango couch festival cheese unveil half stone gas
  Network ID:        substrate
  Secret seed:       0xf851acecf63afb95a75b88461b2f97a1a6b3e20386b941dbea58b7e8f2da35db
  Public key (hex):  0x6ab0ae0d1e20387ca7a5f37a243297c6b470b4750df0607a28f368dc4af13477
  Account ID:        0x6ab0ae0d1e20387ca7a5f37a243297c6b470b4750df0607a28f368dc4af13477
  Public key (SS58): 5EUbRBq34e4AqZEkwgFxK6KUGyYLdQ3XhBcYzgbgRc6kdugM
  SS58 Address:      5EUbRBq34e4AqZEkwgFxK6KUGyYLdQ3XhBcYzgbgRc6kdugM


################################################################################
./target/release/node-template build-spec --disable-default-bootnode --chain local > customSpec.json

./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json



################################################################################

./target/release/node-template key insert --base-path /tmp/node01 \
  --chain customSpecRaw.json \
  --scheme Sr25519 \
  --suri <your-secret-seed> \
  --key-type aura

./target/release/node-template key insert --base-path /tmp/node2 \
  --chain customSpecRaw.json \
  --scheme Sr25519 \
  --suri "forest injury struggle drama mango couch festival cheese unveil half stone gas" \
  --key-type aura
==================================================
  ./target/release/node-template key insert \
  --base-path /tmp/node01 \
  --chain customSpecRaw.json \
  --scheme Ed25519 \
  --suri <your-secret-key> \
  --key-type gran


  ./target/release/node-template key insert \
  --base-path /tmp/node02 \
  --chain customSpecRaw.json \
  --scheme Ed25519 \
  --suri 0xf851acecf63afb95a75b88461b2f97a1a6b3e20386b941dbea58b7e8f2da35db \
  --key-type gran
==================================================



  ./target/release/node-template \
  --base-path /tmp/node01 \
  --chain ./customSpecRaw.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode01 \


  ./target/release/node-template \
  --base-path /tmp/node02 \
  --chain ./customSpecRaw.json \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode02 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWAjJmhnvAyLjxhNb3mybiv5ysoageh1CKibmHwTjsWGUv \
