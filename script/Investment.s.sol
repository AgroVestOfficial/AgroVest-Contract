// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Investment} from "../src/Investment.sol";

contract InvestmentScript is Script {
    Investment public investment;
    function setUp() public {}
    function run() public {
        vm.startBroadcast();
        investment = new Investment(0x994f29ce3A1d753983cd4Ebb0D84A1CE75ba5e1b);
        vm.stopBroadcast();
    }
}
