const BigNumber = require('./node_modules/bignumber.js')
const { keccak256 } = require("@ethersproject/keccak256")

function main() {
    const keccak256Data = keccak256(0x06);
    console.log(keccak256Data);
    var a = new BigNumber(keccak256Data);
    var bn = BigInt(keccak256Data);

    var d = bn.toString(10);
    console.log(d)
}

main();

// dbb8d0f4c497851a5043c6363657698cb1387682cac2f786c731f8936109d795
// 0xdbb8d0f4c497851a5043c6363657698cb1387682cac2f786c731f8936109d795