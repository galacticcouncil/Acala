import { expect } from "chai";

import Block from "../build/Block.json"
import { describeWithAcala, nextBlock } from "./util";
import { deployContract } from "ethereum-waffle";
import { Signer } from "@acala-network/bodhi";
import { Contract } from "ethers";

describeWithAcala("Acala RPC (Contract Methods)", (context) => {
	let alice: Signer;
	let contract: Contract;

	before("create the contract", async function () {
		this.timeout(15000);
		[ alice ] = await context.provider.getWallets();
		contract = await deployContract(alice as any, Block);
	});

	it("should return contract method result", async function () {
		expect((await contract.multiply(3)).toString()).to.equal("21");
	});

	it("should get correct environmental block number", async function () {
		// Solidity `block.number` is expected to return the same height at which the runtime call was made.
		let height = await contract.currentBlock();
		let current_block_number = await context.provider.api.query.system.number();

		expect(await height.toString()).to.eq(current_block_number.toString());
		expect((await context.provider.getBlockNumber()).toString()).to.equal(current_block_number.toString());
	});

	it("should get correct environmental block hash", async function () {
		this.timeout(15000);
		// Solidity `blockhash` is expected to return the ethereum block hash at a given height.
		let number = await context.provider.getBlockNumber();
		let last = number + 900;

		// TODO
		expect(await contract.blockHash(number)).to.eq(
			"0x0000000000000000000000000000000000000000000000000000000000000000"
		);
		expect(await contract.blockHash(number + 1)).to.eq(
			"0x0000000000000000000000000000000000000000000000000000000000000000"
		);
		console.log(number);
		console.log((await contract.blockHash(number)).toString());
		console.log((await context.provider.api.query.system.blockHash(number)).toString());
		console.log((await context.provider.api.rpc.chain.getBlockHash(number)).toString());

		//for(let i = number; i <= last; i++) {
		//	let hash = await context.provider.api.query.system.blockHash(i);
		//	expect(await contract.blockHash(i)).to.eq(hash.toString());
		//	await nextBlock(context.provider);
		//}
		//// should not store more than 900 hashes
		//expect(await contract.blockHash(number)).to.eq(
		//	"0x0000000000000000000000000000000000000000000000000000000000000000"
		//);
	});

	it("should get correct environmental chainId", async function () {
		expect((await contract.chainId()).toString()).to.eq('595');
	});

	it("should get correct environmental coinbase", async function () {
		expect((await contract.coinbase()).toString()).to.eq('0x0000000000000000000000000000000000000000');
	});

	it("should get correct environmental block timestamp", async function () {
		expect((await contract.timestamp()).toString()).to.eq((parseInt(await context.provider.api.query.timestamp.now() / 1000)).toString());
	});

	it("should get correct environmental block gaslimit", async function () {
		expect((await contract.gasLimit()).toString()).to.eq('0');
	});

	// Requires error handling
	it("should fail for missing parameters", async function () {
		const mock = new Contract(contract.address, [
			{
				...Block.abi.filter(function (entry) { return entry.name === "multiply"; })[0],
				inputs: [],
			}
		], alice);

		await mock
			.multiply()
			.catch((err) =>
				expect(err.message).to.equal(`-32603: execution revert: 0x`)
			);
	});

	// Requires error handling
	it("should fail for too many parameters", async function () {
		const mock = new Contract(contract.address, [
			{
				...Block.abi.filter(function (entry) { return entry.name === "multiply"; })[0],
				inputs: [
					{ internalType: "uint256", name: "a", type: "uint256" },
					{ internalType: "uint256", name: "b", type: "uint256" },
				],
			}
		], alice);

		await mock
			.multiply(3, 4)
			.catch((err) =>
				expect(err.message).to.equal(`-32603: execution revert: 0x`)
			);
	});

	// Requires error handling
	it("should fail for invalid parameters", async function () {
		const mock = new Contract(contract.address, [
			{
				...Block.abi.filter(function (entry) { return entry.name === "multiply"; })[0],
				inputs: [
					{ internalType: "address", name: "a", type: "address" },
				],
			}
		], alice);

		await mock
			.multiply("0x0123456789012345678901234567890123456789")
			.catch((err) =>
				expect(err.message).to.equal(`-32603: execution revert: 0x`)
			);
	});
});
