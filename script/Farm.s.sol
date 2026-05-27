// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Farm} from "../src/Farm.sol";

contract FarmScript is Script {
    Farm public farm;
    function setUp() public {}
    function run() public {
        vm.startBroadcast();
        farm = new Farm(
            address(0x994f29ce3A1d753983cd4Ebb0D84A1CE75ba5e1b),
            address(0x6Cfa015f4Eac27c328951396791A93d29E84C237)
        );
        vm.stopBroadcast();
    }
}
