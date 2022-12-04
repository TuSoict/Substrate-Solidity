// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;
import "./Ownable.sol";
import "./SafeMath.sol";

uint8 constant MAX_BREEDING_POINTS = 8;
uint8 constant LVL_CAN_BREED = 10;
uint8 constant LVL_MAX = 20;
uint16 constant ATTACK_COUNT_DEFAULT = 10;
uint constant BASE_EXP = 100;
uint constant AMOUNT_REWARD = 10;

contract ZombieBase is Ownable {
    using SafeMath for uint256;
    using SafeMath32 for uint32;
    using SafeMath16 for uint16;

    uint dnaDigits = 16;
    uint dnaModulus = 10**dnaDigits;
    uint cooldownTime = 1 days;

    struct Zombie {
        uint id;
        string name;
        uint dna;
        uint32 level;
        uint32 readyTime;
        uint16 winCount;
        uint16 lossCount;
        uint16 breeds_points;
        Sex sex;
        uint32 attack;
        uint16 attack_count;
        uint exp;
    }
    uint randNonce = 0;
    uint16[] public EXP_UP_LEVEL = [
        100,
        205,
        315,
        536,
        878,
        1237,
        1835,
        2463,
        3122,
        4156,
        5601,
        7118,
        8711,
        10982,
        13366,
        15869,
        19125,
        22544,
        26793,
        32289
    ];

    Zombie[] public zombies;

    mapping(uint => address) public zombieToOwner;
    mapping(address => uint) ownerZombieCount;

    enum Sex {
        Male,
        Female
    }

    function randMod(uint _modulus) internal returns (uint) {
        randNonce = randNonce.add(1);
        return
            uint(
                keccak256(
                    abi.encodePacked(block.timestamp, msg.sender, randNonce)
                )
            ) % _modulus;
    }

    function randomSex() internal view returns (Sex) {
        uint rand = uint(keccak256(abi.encodePacked(block.timestamp)));
        return Sex(rand % 2);
    }

    function randomAttack() internal returns (uint) {
        uint rand = randMod(200);
        rand = rand.add(1000);
        return rand;
    }

    function randomZombie(uint _zombieId) internal view returns (uint) {
        uint counter = 0;
        uint[] memory result = new uint[](zombies.length);
        address _owner = zombieToOwner[_zombieId];
        for (uint i = 0; i < zombies.length; i++) {
            if (_owner != zombieToOwner[i] && _isCanAttack(i)) {
                result[counter] = i;
                counter++;
            }
        }

        uint rand = 0;
        if (counter > 0) {
            rand = uint(keccak256(abi.encodePacked(block.timestamp))) % counter;
            return result[rand];
        }

        return zombies.length;
    }

    function _isCanAttack(uint _zombieId) internal view returns (bool) {
        Zombie memory _zombie = zombies[_zombieId];
        return (_zombie.attack_count > 0);
    }

    function internalLevelUp(uint _zombieId) internal {
        while (
            zombies[_zombieId].level < LVL_MAX &&
            zombies[_zombieId].exp >= EXP_UP_LEVEL[zombies[_zombieId].level - 1]
        ) {
            zombies[_zombieId].level = zombies[_zombieId].level.add(1);
            zombies[_zombieId].attack = zombies[_zombieId].attack.mul(105).div(
                100
            );
        }
    }

    function resetAttackCount() external onlyOwner {
        for (uint i = 0; i < zombies.length; i++) {
            zombies[i].attack_count = ATTACK_COUNT_DEFAULT;
        }
    }
}
