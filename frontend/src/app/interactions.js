'use client'
import {
    StellarWalletsKit,
    WalletNetwork,
    allowAllModules,
    XBULL_ID,
    ISupportedWallet
} from '@creit.tech/stellar-wallets-kit';
import { ProxyPOST } from './forwarder';
var StellarSdk = require('@stellar/stellar-sdk');

const kitConfig = {
    network: WalletNetwork.TESTNET,
    selectedWalletId: XBULL_ID,
    modules: allowAllModules(),
};

async function createTransaction(mode, from, sequence, params) {
    const res = await ProxyPOST(JSON.stringify({ mode: { "Function": { fname: "simulate", arguments: JSON.stringify({ [mode]: { from, sequence: sequence + 1, ...params } }) } } }));
    return res;
}

async function handleTransaction(kit, from, sequence, mode, params, elementId) {
    document.getElementById(elementId).innerText = "pending";

    const res = await createTransaction(mode, from, sequence, params);
    const { result } = await kit.signTx({ xdr: res.tx, publicKeys: [from], network: WalletNetwork.TESTNET });
    const signedTx = StellarSdk.xdr.TransactionEnvelope.fromXDR(result, "base64");
    const tx = new StellarSdk.Transaction(signedTx, WalletNetwork.TESTNET);

    const server = new StellarSdk.Horizon.Server('https://horizon-testnet.stellar.org');
    const sendResponse = await server.submitTransaction(tx);

    if (sendResponse.successful) {
        document.getElementById(elementId).innerText = "Successful";
        document.location.reload(true);
    } else {
        document.getElementById(elementId).innerText = sendResponse.errorResultXdr;
    }
}

async function vote(hash, upvote) {
    const kit = new StellarWalletsKit(kitConfig);
    await kit.openModal({
        onWalletSelected: async (option) => {
            kit.setWallet(option.id);
            const publicKey = await kit.getPublicKey();
            const server = new StellarSdk.Horizon.Server('https://horizon-testnet.stellar.org');
            const sequence = parseInt((await server.loadAccount(publicKey)).sequenceNumber());

            await handleTransaction(kit, publicKey, sequence, 'Vote', { hash, upvote }, hash);
        }
    });
}

async function feedback(message) {
    const kit = new StellarWalletsKit(kitConfig);
    await kit.openModal({
        onWalletSelected: async (option) => {
            kit.setWallet(option.id);
            const publicKey = await kit.getPublicKey();
            const server = new StellarSdk.Horizon.Server('https://horizon-testnet.stellar.org');
            const sequence = parseInt((await server.loadAccount(publicKey)).sequenceNumber());

            await handleTransaction(kit, publicKey, sequence, 'Send', { message }, 'send-status');
        }
    });
}

export async function Upvote({ hash }) {
    await vote(hash, true);
}

export async function Downvote({ hash }) {
    await vote(hash, false);
}

export const FeedbackTable = ({ feedback }) => {
    const handleVote = async (hash, upvote) => {
        upvote ? await Upvote({ hash }) : await Downvote({ hash });
    };

    return (
        <div className="flex flex-col w-full text-sm text-gray-900 dark:text-gray-800">
            {feedback.map((item, index) => (
                <div key={index} className="bg-white border-b dark:bg-gray-200 dark:border-gray-400 mb-4 p-4">
                    <div className="flex flex-col mb-2">
                        <span className="text-xs text-gray-700 uppercase">Message</span>
                        <textarea readOnly className="w-full h-[80px] scrollbar-hide bg-gray-200 mt-1" value={item.text}></textarea>
                    </div>
                    <div className="flex flex-col mb-2">
                        <span className="text-xs text-gray-700 uppercase">From</span>
                        <span className="mt-1">{item.from}</span>
                    </div>
                    <div className="flex flex-col mb-2">
                        <span className="text-xs text-gray-700 uppercase">Hash</span>
                        <span className="mt-1">{item.hash}</span>
                    </div>
                    <div className="flex flex-col">
                        <span className="text-xs text-gray-700 uppercase">Votes</span>
                        <span className="mt-1">{item.votes}</span>
                    </div>
                    <button
                        className="mt-4 mr-2 bg-blue-500 text-white py-2 px-4 rounded"
                        onClick={() => handleVote(item.hash, true)}
                    >
                        Upvote
                    </button>
                    <button
                        className="mt-4 bg-blue-500 text-white py-2 px-4 rounded"
                        onClick={() => handleVote(item.hash, false)}
                    >
                        Downvote
                    </button>
                    <p className='mt-2' id={item.hash}></p>
                </div>
            ))}
        </div>
    );
};

export const FeedbackForm = () => {
    const handleSend = async () => {
        const text = document.getElementById("send-text").value;
        await feedback(text);
    };

    return (
        <div className="flex flex-col w-full text-sm text-gray-900 dark:text-gray-800">
            <div className="bg-white dark:bg-gray-200 dark:border-gray-400 mb-8 p-4">
                <div className="flex flex-col mb-2">
                    <span className="text-xs text-gray-700 uppercase">Send your feedback</span>
                </div>
                <div className="flex flex-col mb-2">
                    <textarea id="send-text" className="mt-1"></textarea>
                </div>
                <button
                    className="mt-4 mr-2 bg-blue-500 text-white py-2 px-4 rounded"
                    onClick={handleSend}
                >
                    Publish On-Chain
                </button>
                <p className='mt-2' id="send-status"></p>
            </div>
        </div>
    );
};
