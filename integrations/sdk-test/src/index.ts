import { bcs } from '@dwallet-network/dwallet.js/bcs';
import { DWalletClient, SuiHTTPTransport } from '@dwallet-network/dwallet.js/client';
import { requestSuiFromFaucetV0 as requestDwltFromFaucetV0 } from '@dwallet-network/dwallet.js/faucet';
import { Ed25519Keypair } from '@dwallet-network/dwallet.js/keypairs/ed25519';
import {
	createActiveEncryptionKeysTable,
	createDWallet,
	createPartialUserSignedMessages,
	createVirginBoundDWallet,
	getOrCreateEncryptionKey,
	submitDWalletCreationProof,
	submitTxStateProof,
} from '@dwallet-network/dwallet.js/signature-mpc';
import { SuiClient } from '@mysten/sui.js/client';
import { TransactionBlock as TransactionBlockSUI } from '@mysten/sui.js/transactions';

function fromB64(base64String: string): Uint8Array {
	return Uint8Array.from(atob(base64String), (char) => char.charCodeAt(0));
}

async function main() {
	try {
		const serviceUrl = 'http://localhost:6920/gettxdata'; // For local development
		// const serviceUrl = 'http://sui-testnet-light-client.testnet.dwallet.cloud/gettxdata';

		const dWalletNodeUrl = 'http://127.0.0.1:9000';

		const suiTestnetURL = 'https://fullnode.testnet.sui.io:443';

		const dWalletCapPackageSUI =
			'0x8b527e2c7b0b29f2f6fe25a5b4505a4e0473f2d54a1c9dfaff125eed1eb327fd';

		const configObjectId = '0x3cae1bcb0ad02137a4f60509133d174c9abaa00fb42072c1bb5d9c3474aa871d';
		const registryObjectId = '0xdc85edbdee2f2630d05464fa8740a19cdaf9c4e20aabb519ad9f174a3f2b44b9';

		const sui_client = new SuiClient({ url: suiTestnetURL });
		const dwallet_client = new DWalletClient({
			transport: new SuiHTTPTransport({
				url: dWalletNodeUrl,
			}),
		});

		// const keyPair = Ed25519Keypair.deriveKeypairFromSeed(
		//   'witch collapse practice feed shame open despair creek road again ice least',
		// )
		// const keyPair = Ed25519Keypair.generate();
		const PRIVATE_KEY_SIZE = 32;
		const key = 'AEoBv2qpRIJ3N6JInfoQGj0tqYkbIGkho3mMQPYjm2Yt';
		const raw = fromB64(key);
		if (raw[0] !== 0 || raw.length !== PRIVATE_KEY_SIZE + 1) {
			throw new Error('invalid key');
		}
		const keyPair = Ed25519Keypair.fromSecretKey(raw.slice(1));

		const address = keyPair.getPublicKey().toSuiAddress();
		// const address2 = keyPair2.getPublicKey().toSuiAddress();

		console.log('address', address);
		// console.log('address2', address);

		console.log('SUI address', keyPair.toSuiAddress());
		// console.log('SUI address2', keyPair2.toSuiAddress());

		await requestDwltFromFaucetV0({
			host: 'http://127.0.0.1:9123/gas',
			recipient: keyPair.getPublicKey().toSuiAddress(),
		});

		// await requestDwltFromFaucetV0({
		// 	host: 'http://127.0.0.1:9123/gas',
		// 	recipient: keyPair2.getPublicKey().toSuiAddress(),
		// });

		// await requestSuiFromFaucetV0({
		// 	host: 'https://faucet.testnet.sui.io',
		// 	recipient: keyPair.getPublicKey().toSuiAddress(),
		// });

		// sleep for 5 seconds
		await new Promise((resolve) => setTimeout(resolve, 5000));

		console.log('creating dwallet');

		const encryptionKeysHolder = await createActiveEncryptionKeysTable(dwallet_client, keyPair);
		// const encryptionKeysHolder2 = await createActiveEncryptionKeysTable(dwallet_client, keyPair2);

		let activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
		let senderEncryptionKeyObj = await getOrCreateEncryptionKey(
			keyPair,
			dwallet_client,
			activeEncryptionKeysTableID,
		);

		// let activeEncryptionKeysTableID2 = encryptionKeysHolder2.objectId;
		// let senderEncryptionKeyObj2 = await getOrCreateEncryptionKey(
		// 	keyPair2,
		// 	dwallet_client,
		// 	activeEncryptionKeysTableID2,
		// );

		// todo(yuval): create bind to authority

		// todo(yuval): virgin bound dwallet here
		const createdDwallet1 = await createVirginBoundDWallet(
			keyPair,
			dwallet_client,
			senderEncryptionKeyObj.encryptionKey,
			senderEncryptionKeyObj.objectID,
		);
		const createdDwallet2 = await createVirginBoundDWallet(
			keyPair,
			dwallet_client,
			senderEncryptionKeyObj.encryptionKey,
			senderEncryptionKeyObj.objectID,
		);

		const createdDwallet3 = await createVirginBoundDWallet(
			keyPair,
			dwallet_client,
			senderEncryptionKeyObj.encryptionKey,
			senderEncryptionKeyObj.objectID,
		);

		if (createdDwallet1 == null || createdDwallet2 == null) {
			throw new Error('createDWallet returned null');
		}
		let dwalletCapId1 = createdDwallet1?.dwalletCapID;
		let dWalletId1 = createdDwallet1?.dwalletID;

		let dwalletCapId2 = createdDwallet2?.dwalletCapID;
		// let dWalletId2 = createdDwallet2?.dwalletID;

		let dwalletCapId3 = createdDwallet3?.dwalletCapID!;

		console.log('initialising dwallet cap with ID: ', dwalletCapId1);
		let txb = new TransactionBlockSUI();

		let dWalletCapArg1 = txb.pure(dwalletCapId1);
		let dWalletCapArg2 = txb.pure(dwalletCapId2);
		let dWalletCapArg3 = txb.pure(dwalletCapId3);

		let [cap1] = txb.moveCall({
			target: `${dWalletCapPackageSUI}::dwallet_cap::create_cap`,
			arguments: [dWalletCapArg1],
		});

		let [cap2] = txb.moveCall({
			target: `${dWalletCapPackageSUI}::dwallet_cap::create_cap`,
			arguments: [dWalletCapArg2],
		});
		let [cap3] = txb.moveCall({
			target: `${dWalletCapPackageSUI}::dwallet_cap::create_cap`,
			arguments: [dWalletCapArg3],
		});

		const messageSign: Uint8Array = new TextEncoder().encode('dWallets are coming... to Sui');

		let signMsgArg = txb.pure(bcs.vector(bcs.vector(bcs.u8())).serialize([messageSign]));

		txb.moveCall({
			target: `${dWalletCapPackageSUI}::dwallet_cap::approve_message`,
			arguments: [cap1, signMsgArg],
		});
		txb.moveCall({
			target: `${dWalletCapPackageSUI}::dwallet_cap::approve_message`,
			arguments: [cap2, signMsgArg],
		});
		txb.moveCall({
			target: `${dWalletCapPackageSUI}::dwallet_cap::approve_message`,
			arguments: [cap3, signMsgArg],
		});

		txb.transferObjects([cap1], keyPair.toSuiAddress());
		txb.transferObjects([cap2], keyPair.toSuiAddress());
		txb.transferObjects([cap3], keyPair.toSuiAddress());

		txb.setGasBudget(10000000);

		let res = await sui_client.signAndExecuteTransactionBlock({
			signer: keyPair,
			transactionBlock: txb,
			options: {
				showEffects: true,
			},
		});

		const createCapTxId = res.digest;
		const signTxId = res.digest;
		// const approveMsgTxId = res.digest;

		let first = res.effects?.created?.[0];
		let ref;
		if (first) {
			ref = first.reference.objectId;
			console.log('cap created', ref);
		} else {
			console.log('No objects were created');
		}

		// sleep for 10 seconds
		await new Promise((resolve) => setTimeout(resolve, 15000));

		console.log('address', keyPair.getPublicKey().toSuiAddress());
		// console.log('dWalletId1', dwalletCapId1)
		// console.log('dWalletId2', dwalletCapId2)
		// console.log('dWalletId3', dwalletCapId3)
		// todo(yuval): this should be replaced with create virgin dwallet
		let resultFinal1 = await submitDWalletCreationProof(
			dwallet_client,
			sui_client,
			configObjectId,
			registryObjectId,
			dwalletCapId1,
			createCapTxId,
			serviceUrl,
			keyPair,
		);
		await new Promise((resolve) => setTimeout(resolve, 15000));

		let resultFinal2 = await submitDWalletCreationProof(
			dwallet_client,
			sui_client,
			configObjectId,
			registryObjectId,
			dwalletCapId2,
			createCapTxId,
			serviceUrl,
			keyPair,
		);
		await new Promise((resolve) => setTimeout(resolve, 15000));

		let resultFinal3 = await submitDWalletCreationProof(
			dwallet_client,
			sui_client,
			configObjectId,
			registryObjectId,
			dwalletCapId3,
			createCapTxId,
			serviceUrl,
			keyPair,
		);

		console.log('creation done 1', resultFinal1);
		// console.log('creation done 2', resultFinal2)
		// console.log('creation done 3', resultFinal3)

		const bytes: Uint8Array = new TextEncoder().encode('dWallets are coming... to Sui');

		const signMessagesIdSHA256 = await createPartialUserSignedMessages(
			createdDwallet1?.dwalletID!,
			createdDwallet1?.decentralizedDKGOutput!,
			new Uint8Array(createdDwallet1?.secretKeyShare!),
			[bytes],
			'SHA256',
			keyPair,
			dwallet_client,
		);

		console.log('created signMessages');

		if (signMessagesIdSHA256 == null) {
			throw new Error('createSignMessages returned null');
		}

		if (
			resultFinal1.effects &&
			Array.isArray(resultFinal1.effects.created) &&
			typeof resultFinal1.effects.created[0] === 'object' &&
			'reference' in resultFinal1.effects.created[0]
		) {
			const capWrapperRef = resultFinal1.effects?.created?.[0].reference;

			console.log('A');

			let res = await submitTxStateProof(
				dwallet_client,
				sui_client,
				dWalletId1,
				configObjectId,
				registryObjectId,
				capWrapperRef,
				signMessagesIdSHA256,
				signTxId,
				serviceUrl,
				keyPair,
			);

			console.log('res', res);
			console.log('tx done');
		}
	} catch (error) {
		console.error('Failed to retrieve transaction data:', error);
	}
}

main();
