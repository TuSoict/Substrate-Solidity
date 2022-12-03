// SPDX-License-Identifier: None
pragma solidity 0.8.3;

contract baobuakeo {
    event resultEvent(string name, uint amount);
    address constant player1 = 0x5B38Da6a701c568545dCfcB03FcB875f56beddC4;
    address constant player2 = 0xAb8483F64d9C6d1EcF9b849Ae677dD3315835cb2;

    enum Choice {
        Empty,
        Bao,
        Bua,
        Keo
    }

    Choice public player1Choice = Choice.Empty;
    Choice public player2Choice = Choice.Empty;

    bool gameEnded = false;

    mapping(address => uint) public balances;

    // commit the choice (Bao / Bua / Keo)
    function choice(Choice item) public payable {
        require(
            msg.sender == player1 || msg.sender == player2,
            "not player1 or player2"
        );

        if (msg.sender == player1) {
            player1Choice = item;
        } else {
            player2Choice = item;
        }
    }

    // check the result
    function findResult() public {
        require(!gameEnded, "can only compute result once");
        require(
            player1Choice != Choice.Empty && player2Choice != Choice.Empty,
            "someone did not reveal their choice"
        );

        // draw
        if (player1Choice == player2Choice) {
            balances[player1] += 1 ether;
            balances[player2] += 1 ether;
        } else if (player1Choice == Choice.Bao) {
            if (player2Choice == Choice.Bua) {
                balances[player2] += 2 ether;
            } else {
                balances[player1] += 2 ether;
            }
        } else if (player1Choice == Choice.Bua) {
            if (player2Choice == Choice.Keo) {
                balances[player2] += 2 ether;
            } else {
                balances[player1] += 2 ether;
            }
        } else if (player1Choice == Choice.Keo) {
            if (player2Choice == Choice.Bao) {
                balances[player2] += 2 ether;
            } else {
                balances[player1] += 2 ether;
            }
        }

        if (player1Choice == player2Choice) {
            emit resultEvent("player1, player2", balances[player1]);
        } else if (player1Choice < player2Choice) {
            emit resultEvent("player1 win", balances[player1]);
        } else {
            emit resultEvent("player2 win", balances[player2]);
        }

        gameEnded = true;
    }

    function resetAll() public {
        require(gameEnded);
        player1Choice = Choice.Empty;
        player2Choice = Choice.Empty;
        gameEnded = false;
    }

    function claimMoney() public {
        require(
            msg.sender == player1 || msg.sender == player2,
            "not player1 or player2"
        );
        require(balances[msg.sender] > 0);

        uint amount = balances[msg.sender];
        balances[msg.sender] = 0;
        bool transferred = payable(msg.sender).send(amount);
        if (transferred != true) {
            balances[msg.sender] = amount;
        }
    }
}
