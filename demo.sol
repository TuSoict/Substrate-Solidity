pragma solidity ^0.8.4;

contract RockPaperScissor {
    enum Choice {NONE, ROCK, PAPER, SCISSOR}

    struct PlayerChoice {
        address player;
        Choice choice;
        bytes32 commitHash;
    }

    PlayerChoice[2] public players;

    error AllPlayersRegistered(address player1, address player2);
    function registerPlayer() public {
        if (players[0].player == address(0)) {
            players[0].player = msg.sender;
        } else if (players[1].player == address(0)) {
            players[1].player = msg.sender;
        } else {
            revert AllPlayersRegistered({player1: players[0].player, player2: players[1].player});
        }
    }

    function play(bytes32 _commitHash) public {
        uint index = validateAndFindPlayerIndex();
        players[index].commitHash = _commitHash;
    }

    function reveal(Choice _choice, bytes32 _salt) public{
        require(players[0].commitHash != 0x0 && players[1].commitHash != 0x0);

        uint index = validateAndFindPlayerIndex();

        require(players[index].commitHash == getSaltedHash(_choice, _salt));
        
        players[index].choice = _choice;
    }

    function getAddress() public view returns (address) {
        return address(this);
    }

    function getSaltedHash(Choice _answer, bytes32 _salt) internal view returns (bytes32) {
        return keccak256(abi.encodePacked(address(this), _answer, _salt));
    }

    function checkWinner() public view returns (address) {
        if (players[0].choice == players[1].choice) return address(0);
        if (players[0].choice == Choice.ROCK && players[1].choice == Choice.SCISSOR) return players[0].player;
        if (players[0].choice == Choice.PAPER && players[1].choice == Choice.ROCK) return players[0].player;
        if (players[0].choice == Choice.SCISSOR && players[1].choice == Choice.PAPER) return players[0].player;

        return players[1].player;
    }

    function validateAndFindPlayerIndex() internal view returns (uint) {
        if (players[0].player == msg.sender && players[0].choice == Choice.NONE) return 0;
        if (players[1].player == msg.sender && players[1].choice == Choice.NONE) return 1;
        
        revert("Invalid call");
    }
}