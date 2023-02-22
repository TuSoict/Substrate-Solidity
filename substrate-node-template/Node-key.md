1. Cmd:
./target/release/subkey generate

Output:
Secret phrase:       village into fix awesome give lizard tobacco quantum capital second spare custom
  Network ID:        substrate
  Secret seed:       0x955a848ab638f47e38a76a2602cd0f96f75d8d2f73f3e27ef57ba8621dbe4e62
  Public key (hex):  0x7c3552be20945aa70d657164f43872ee2ce4dd37ef682547f007f7063c8f2b38
  Account ID:        0x7c3552be20945aa70d657164f43872ee2ce4dd37ef682547f007f7063c8f2b38
  Public key (SS58): 5EsZdnxLZ15oHD3WvUmDzWVnBAeVB4bxpXxQwXQ6Ew2bwaNJ
  SS58 Address:      5EsZdnxLZ15oHD3WvUmDzWVnBAeVB4bxpXxQwXQ6Ew2bwaNJ

2. Save Secret Seed to file `key-20230222`: without `0x`
Node Key(Secret seed): 955a848ab638f47e38a76a2602cd0f96f75d8d2f73f3e27ef57ba8621dbe4e62
3. Cmd:
./target/release/subkey inspect-node-key --file key-20230222
Output:
Node ID: 12D3KooWR3kDPrNx4f9rxLGuUqR7xjb1e4pG9GmUXQxtQDFLJ8Yh

4. Calculate OpaquePeerId
Using Decode base58 Node ID Output `Hex`: https://appdevtools.com/base58-encoder-decoder
Output(add `0x`):0x002408011220e24cc81461111f1f679819eff0a8bf05d3e9d9a8d0ca054559bce31a8f0d8842

5. Click Developer and select Sudo.
Select nodeAuthorization and select addWellKnownNode(node, owner).

6. Start Node:
./target/release/node-template \
--chain=local \
--base-path /tmp/validator5 \
--name duonghb  \
--node-key=955a848ab638f47e38a76a2602cd0f96f75d8d2f73f3e27ef57ba8621dbe4e62 \
--port 30337 \
--ws-port=9948 \
--offchain-worker always