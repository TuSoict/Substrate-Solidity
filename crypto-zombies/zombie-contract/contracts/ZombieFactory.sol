// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

// Import this file to use console.log
import "hardhat/console.sol";
import "./Ownable.sol";
import "./SafeMath.sol";
import "./ZombieBase.sol";

contract ZombieFactory is ZombieBase {
    using SafeMath for uint256;
    using SafeMath32 for uint32;
    using SafeMath16 for uint16;

    event NewZombie(
        address sender,
        uint zombieId,
        string name,
        uint dna,
        Sex sex,
        uint32 level
    );

    function _createZombie(string memory _name, uint _dna) internal {
        Sex sex = randomSex();
        uint id = zombies.length;
        zombies.push(
            Zombie(
                id,
                _name,
                _dna,
                1,
                uint32(block.timestamp + cooldownTime),
                0,
                0,
                0,
                sex,
                uint16(randomAttack()),
                ATTACK_COUNT_DEFAULT,
                0
            )
        );
        zombieToOwner[id] = msg.sender;
        ownerZombieCount[msg.sender] = ownerZombieCount[msg.sender].add(1);
        emit NewZombie(msg.sender, id, _name, _dna, sex, 1);
    }

    function _generateRandomDna(string memory _str) private returns (uint) {
        randNonce = randNonce.add(1);
        uint rand = uint(
            keccak256(
                abi.encodePacked(block.timestamp, msg.sender, randNonce, _str)
            )
        );
        return rand % dnaModulus;
    }

    function createRandomZombie(string memory _name) public {
        uint randDna = _generateRandomDna(_name);
        randDna = randDna - (randDna % 100);
        _createZombie(_name, randDna);
    }
}
