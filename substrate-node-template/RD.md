# Wallet A:

## 1. Sr25519 key Aura
   Secret phrase: deliver weird dolphin bid typical enable agree minor ramp female debate prepare
   Network ID: substrate
   Secret seed: 0x25b733bd313f4519613e725fa509c420e520da6560b774354dcd1d0a281f5a1a
   Public key (hex): 0x360f1a7f559b1c576f47fca67e5bbcecf5505c561f883e205c2513807645dc18
   Account ID: 0x360f1a7f559b1c576f47fca67e5bbcecf5505c561f883e205c2513807645dc18
   Public key (SS58): 5DHavsUFk75WtG9PQqNBjMRx1rg5aZJ5m4t5TNgN7PDDXxnB
   SS58 Address: 5DHavsUFk75WtG9PQqNBjMRx1rg5aZJ5m4t5TNgN7PDDXxnB

2. Gen key Granpa:
   --> ./target/release/node-template key inspect --password-interactive --scheme Ed25519 "deliver weird dolphin bid typical enable agree minor ramp female debate prepare"

Secret phrase: deliver weird dolphin bid typical enable agree minor ramp female debate prepare
Network ID: substrate
Secret seed: 0x25b733bd313f4519613e725fa509c420e520da6560b774354dcd1d0a281f5a1a
Public key (hex): 0xcba1a88b74d762da5567fadb3c5d687dfd9d8655aada6d3950b80a65f5c17fdf
Account ID: 0xcba1a88b74d762da5567fadb3c5d687dfd9d8655aada6d3950b80a65f5c17fdf
Public key (SS58): 5GfhbmkAw8UG4Pb7faqmUbF9hvJznhxBkaisKHK7tSNAfEKS
SS58 Address: 5GfhbmkAw8UG4Pb7faqmUbF9hvJznhxBkaisKHK7tSNAfEKS

# Wallet B:

1. Sr25519 key Aura
   Secret phrase: solve hammer mercy beef solar strike output cream mixed joke venture right
   Network ID: substrate
   Secret seed: 0x44e54bdeabd3ae9af9308255455d56323ab5b59469e47bfee3dc0d85ad1d0a8a
   Public key (hex): 0xbc4699d193786fe0a77b60103580c421d3d6533cbc6ba9760a6dab195e341008
   Account ID: 0xbc4699d193786fe0a77b60103580c421d3d6533cbc6ba9760a6dab195e341008
   Public key (SS58): 5GKZqEB8sWCKgvmCyQPg5gK8Fxt449DE9ir7FeoYuVZkBtD8
   SS58 Address: 5GKZqEB8sWCKgvmCyQPg5gK8Fxt449DE9ir7FeoYuVZkBtD8

2. Gen key Granpa:
   --> ./target/release/node-template key inspect --password-interactive --scheme Ed25519 "solve hammer mercy beef solar strike output cream mixed joke venture right"

Secret phrase: solve hammer mercy beef solar strike output cream mixed joke venture right
Network ID: substrate
Secret seed: 0x44e54bdeabd3ae9af9308255455d56323ab5b59469e47bfee3dc0d85ad1d0a8a
Public key (hex): 0xc31bf4468df0852ac1eae5491545d042ba6a9b9c38ec9fb5f05ec2933329c0fd
Account ID: 0xc31bf4468df0852ac1eae5491545d042ba6a9b9c38ec9fb5f05ec2933329c0fd
Public key (SS58): 5GUXVtFWeg1vWvKsnNPv8pLy6XfQc2JSg24LnWhHT6wkufkg
SS58 Address: 5GUXVtFWeg1vWvKsnNPv8pLy6XfQc2JSg24LnWhHT6wkufkg

3. Generate `customSpec.json`:
```sh
   ./target/release/node-template build-spec --disable-default-bootnode --chain local > customSpec.json
```

--> Change key Aura & Granpa in `customSpec.json` file. 
4. Generate `customSpecRaw.json`:
```sh
./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
```
# Run Node A:
```sh
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
 --password-interactive
```

Node A Id: 12D3KooWP3aJDFGW9Nscro3QZD3fssRQ6mjtc5i8aQMYvibTyve5

Insert Key A to ENV
Aura:
```sh
./target/release/node-template key insert --base-path /tmp/node01 \
 --chain customSpecRaw.json \
 --scheme Sr25519 \
 --suri "deliver weird dolphin bid typical enable agree minor ramp female debate prepare" \
 --password-interactive \
 --key-type aura
 ```
Granpa:
```sh
./target/release/node-template key insert \
 --base-path /tmp/node01 \
 --chain customSpecRaw.json \
 --scheme Ed25519 \
 --suri "deliver weird dolphin bid typical enable agree minor ramp female debate prepare" \
 --password-interactive \
 --key-type gran
```

# Run Node B:
```sh
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
 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWP3aJDFGW9Nscro3QZD3fssRQ6mjtc5i8aQMYvibTyve5 \
 --password-interactive
```
Insert Key B to ENV
Aura:
```sh
./target/release/node-template key insert --base-path /tmp/node02 \
 --chain customSpecRaw.json \
 --scheme Sr25519 \
 --suri "solve hammer mercy beef solar strike output cream mixed joke venture right" \
 --password-interactive \
 --key-type aura
```
Granpa:
```sh
./target/release/node-template key insert \
 --base-path /tmp/node02 \
 --chain customSpecRaw.json \
 --scheme Ed25519 \
 --suri "solve hammer mercy beef solar strike output cream mixed joke venture right" \
 --password-interactive \
 --key-type gran
```