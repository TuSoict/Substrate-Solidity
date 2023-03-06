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
--node-key=e4ba688372f0ee35db6c5688bde853b73d0d6fee96fd8e43b7aeba4fb98259fb \
--port 30337 \
--ws-port=9948 \
--offchain-worker always

OpaquePeerId:
0x002408011220792021f26e1752e14d36107f6c34d5bd43f85a8da99767c2d92a49d500079fc1
Node key: e4ba688372f0ee35db6c5688bde853b73d0d6fee96fd8e43b7aeba4fb98259fb
12D3KooWHyBvE1rWBMmMemYA9fMfixbRYrndj8wxNfpyphAYFZ12


Secret phrase:       famous depth dinosaur flight lady bright rail frown input update clarify napkin
  Network ID:        substrate
  Secret seed:       0xe4ba688372f0ee35db6c5688bde853b73d0d6fee96fd8e43b7aeba4fb98259fb
  Public key (hex):  0x04a5a1b62333204e65ac026fc8325281dc64163371648650ac6a0941c291c741
  Account ID:        0x04a5a1b62333204e65ac026fc8325281dc64163371648650ac6a0941c291c741
  Public key (SS58): 5CAoFYbLEBWmEvaAHEq5RywdEJH7xcz5STJ2QKSRmm3TUvws
  SS58 Address:      5CAoFYbLEBWmEvaAHEq5RywdEJH7xcz5STJ2QKSRmm3TUvws