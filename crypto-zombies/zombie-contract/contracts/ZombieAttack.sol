// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "hardhat/console.sol";
import "./ZombieHelper.sol";
import "./SafeMath.sol";

contract ZombieAttack is ZombieHelper {
    using SafeMath for uint256;
    using SafeMath32 for uint32;
    using SafeMath16 for uint16;

    event FindBattle(uint _zombieId);
    event Battle(uint _zombieId);

    function findBattle(uint _zombieId) public view returns (uint) {
        // Kiểm tra Zombie còn lượt tấn công hay không?
        require(_isCanAttack(_zombieId));

        // Kiểm tra nếu chỉ có 1 ví sở hữu Zombie thì sẽ trả về lỗi
        require(_isNotOnlyOwner());

        // Tìm kiếm Zombie
        uint _targetId = randomZombie(_zombieId);
        require(_targetId < zombies.length);
        return _targetId;
    }

    function attack(uint _zombieId, uint _targetId)
        external
        onlyOwnerOf(_zombieId)
    {
        Zombie storage myZombie = zombies[_zombieId];
        Zombie storage enemyZombie = zombies[_targetId];
        // Kiểm tra Zombie còn lượt tấn công hay không?
        require(_isCanAttack(_zombieId));
        require(_isCanAttack(_targetId));

        // Kiểm tra xem Zombie nào chiến thắng
        uint16 myZombieBattleTimes = ATTACK_COUNT_DEFAULT -
            myZombie.attack_count;
        uint16 enemyZombieBattleTimes = ATTACK_COUNT_DEFAULT -
            enemyZombie.attack_count;
        uint32 myZombieAtkCur = myZombie.attack.sub(
            myZombie.attack.mul(5).div(100).mul(myZombieBattleTimes)
        );
        uint32 enemyZombieAtkSur = enemyZombie.attack.sub(
            enemyZombie.attack.mul(5).div(100).mul(enemyZombieBattleTimes)
        );
        uint winnerZombieId = _targetId;
        if (myZombieAtkCur > enemyZombieAtkSur) {
            winnerZombieId = _zombieId;
        } else if (myZombieAtkCur == enemyZombieAtkSur) {
            if (myZombieBattleTimes >= enemyZombieBattleTimes) {
                winnerZombieId = _zombieId;
            }
        }

        // Tính toán số exp nhận được
        uint winnerLevel = 1;
        if (winnerZombieId == _targetId) {
            winnerLevel = enemyZombie.level;
        } else {
            winnerLevel = myZombie.level;
        }
        uint exp = calculateExp(winnerLevel);

        // Tăng điểm kinh nghiệm cho Zombie
        // Exp Zombie thua sẽ = 30% Zombie thắng
        if (winnerZombieId == _targetId) {
            updateZombie(enemyZombie, myZombie, exp);
        } else {
            updateZombie(myZombie, enemyZombie, exp);
        }

        // Kiểm tra nếu Zombie đủ exp sẽ UpLevel + Attack
        internalLevelUp(_zombieId);
        internalLevelUp(_targetId);

        console.log("WinnerId %s, myZombie ", winnerZombieId);
        
        emit Battle(winnerZombieId);
    }

    function calculateExp(uint level) private pure returns (uint) {
        uint expCal = BASE_EXP;
        for (uint i = 2; i <= level; i++) {
            expCal = expCal.mul(105).div(100);
        }
        return expCal;
    }

    // Update các thông tin của Zombie: exp. a
    function updateZombie(
        Zombie storage winZombie,
        Zombie storage lossZombie,
        uint exp
    ) internal {
        winZombie.exp = winZombie.exp.add(exp);
        lossZombie.exp = lossZombie.exp.add(exp.mul(30).div(100));
        winZombie.winCount = winZombie.winCount.add(1);
        lossZombie.lossCount = lossZombie.lossCount.add(1);
        winZombie.attack_count = winZombie.attack_count.sub(1);
        lossZombie.attack_count = lossZombie.attack_count.sub(1);
    }
}
