// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Gamekbb {

    enum Gametype {NONE, KEO, BUA, BAO }

    address private owner;

    address[] public gamers; 

    mapping(address => Gametype) public points; 

    constructor() {
        owner = msg.sender;
    }

    function joinZoom () public {
        require( owner == msg.sender, "you are must player");
        require (gamers.length > 2, "Zoom closed"); 
        gamers.push(msg.sender);
    }

    function play(Gametype r) public {
        require( owner == msg.sender, "you are must player");
        for (uint p = 0; p < gamers.length; p++) {
            bool has = false;
             if (msg.sender == gamers[p]) {
                has = true;
             }    

             require (has, "you can't play the game");
             points[msg.sender] = r;
        }
    }

    function reset()public {
        if (gamers.length == 2) {
            gamers.pop();
            gamers.pop();
        } 
    }

    function winnerPlayer () public view 
                        returns (address wd) {
        //require (gamers.length < 2, "Please in waiting room"); 
        address p1 = gamers[0];
        address p2 = gamers[2];
        wd = p1;               

        Gametype t1 = points[p1];
        Gametype t2 = points[p2];

        if(t1 == Gametype.KEO) {
            if (t2 == Gametype.BUA) { wd = p2; }
            if (t2 == Gametype.BAO) wd = p1;
        }

         if(t1 == Gametype.BUA) {
            if (t2 == Gametype.BAO) { wd = p1; }
            if (t2 == Gametype.KEO) wd = p2;
        }

        if(t1 == Gametype.BAO) {
            if (t2 == Gametype.BUA) { wd = p1; }
            if (t2 == Gametype.KEO) wd = p2;
        }
    }
}
