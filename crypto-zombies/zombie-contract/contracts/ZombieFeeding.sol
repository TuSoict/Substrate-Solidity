// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "./ZombieFactory.sol";

abstract contract KittyInterface {
    function getKitty(uint256 _id)
        external
        view
        virtual
        returns (
            bool isGestating,
            bool isReady,
            uint256 cooldownIndex,
            uint256 nextActionAt,
            uint256 siringWithId,
            uint256 birthTime,
            uint256 matronId,
            uint256 sireId,
            uint256 generation,
            uint256 genes
        );
}

contract ZombieFeeding is ZombieFactory {
    KittyInterface kittyContract;

    using SafeMath for uint256;
    using SafeMath32 for uint32;
    using SafeMath16 for uint16;

    modifier onlyOwnerOf(uint _zombieId) {
        require(msg.sender == zombieToOwner[_zombieId]);
        _;
    }

    function setKittyContractAddress(address _address) external onlyOwner {
        kittyContract = KittyInterface(_address);
    }

    function _triggerCooldown(Zombie storage _zombie) internal {
        _zombie.readyTime = uint32(block.timestamp + cooldownTime);
    }

    function _isReady(Zombie storage _zombie) internal view returns (bool) {
        return (_zombie.readyTime <= block.timestamp);
    }

    function _isCanBreed(Zombie storage _zombie) internal view returns (bool) {
        return (_zombie.level >= LVL_CAN_BREED &&
            _zombie.breeds_points < MAX_BREEDING_POINTS);
    }

    function _stringNotEmptyOrNull(string memory input)
        internal
        pure
        returns (bool)
    {
        return bytes(input).length > 0;
    }

    function feedAndMultiply(
        uint _zombieId,
        uint _targetDna,
        string memory _species
    ) internal onlyOwnerOf(_zombieId) {
        Zombie storage myZombie = zombies[_zombieId];
        require(_isReady(myZombie));
        _targetDna = _targetDna % dnaModulus;
        uint newDna = (myZombie.dna + _targetDna) / 2;
        if (
            keccak256(abi.encodePacked(_species)) ==
            keccak256(abi.encodePacked("kitty"))
        ) {
            newDna = newDna - (newDna % 100) + 99;
        }
        _createZombie("NoName", newDna);
        _triggerCooldown(myZombie);
    }

    function feedOnKitty(uint _zombieId, uint _kittyId) public {
        uint kittyDna;
        (, , , , , , , , , kittyDna) = kittyContract.getKitty(_kittyId);
        feedAndMultiply(_zombieId, kittyDna, "kitty");
    }

    function _generateDna(
        uint dna1,
        uint dna2,
        string memory _name
    ) private returns (uint) {
        randNonce = randNonce.add(1);
        uint rand = uint(
            keccak256(abi.encodePacked(block.timestamp, dna1, dna2, _name))
        );
        return rand % dnaModulus;
    }

    function breedZombie(
        uint _fatherId,
        uint _motherId,
        string memory _name
    ) public onlyOwnerOf(_fatherId) onlyOwnerOf(_motherId) {
        Zombie storage father = zombies[_fatherId];
        Zombie storage mother = zombies[_motherId];

        // Kiểm tra điều kiện
        require(_isCanBreed(father));
        require(_isCanBreed(mother));
        require(_stringNotEmptyOrNull(_name));
        require(father.sex != mother.sex);

        // Add số lượt sinh sản
        father.breeds_points = father.breeds_points.add(1);
        mother.breeds_points = mother.breeds_points.add(1);
        // Tinh toán DNA Zombie con từ DNA của bố mẹ
        uint newKittyDna = _generateDna(father.dna, mother.dna, _name);
        _createZombie(_name, newKittyDna);
    }

    /// For testing
    /// -------------------------------------------------
    function setLevelZombie(uint _zombieId, uint32 level)
        public
        onlyOwnerOf(_zombieId)
    {
        Zombie storage zombie = zombies[_zombieId];
        if (level >= LVL_MAX) level = LVL_MAX;
        zombie.level = level;
    }

    // For testing
    function setSexZombie(uint _zombieId, Sex sex)
        public
        onlyOwnerOf(_zombieId)
    {
        Zombie storage zombie = zombies[_zombieId];
        zombie.sex = sex;
    }
    /// -------------------------------------------------
}
