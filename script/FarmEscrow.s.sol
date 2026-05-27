// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {FarmEscrow} from "../src/FarmEscrow.sol";

contract FarmEscrowScript is Script {
    FarmEscrow public farmEscrow;
    function setUp() public {}
    function run() public {
        vm.startBroadcast();
        farmEscrow = new FarmEscrow(0x994f29ce3A1d753983cd4Ebb0D84A1CE75ba5e1b);
        vm.stopBroadcast();
    }
}
