import React, { useState, useEffect } from 'react';
import { useGetAccountInfo } from '@multiversx/sdk-dapp/hooks';
import { sendTransactions } from '@multiversx/sdk-dapp/services';
import { contractAddress, pingAmount } from '../../config/config.devnet';

export const Dashboard = () => {
    const { address } = useGetAccountInfo();
    const [canPong, setCanPong] = useState(false);

    const ping = async () => {
        const pingTransaction = {
            value: pingAmount,
            data: 'ping',
            receiver: contractAddress,
            gasLimit: 60000000
        };
        await sendTransactions({
            transactions: pingTransaction
        });
    };

    const pong = async () => {
        const pongTransaction = {
            value: '0',
            data: 'pong',
            receiver: contractAddress,
            gasLimit: 60000000
        };
        await sendTransactions({
            transactions: pongTransaction
        });
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen">
            <h1 className="text-3xl font-bold mb-8">Ping Pong dApp</h1>
            
            <div className="space-x-4">
                <button 
                    onClick={ping}
                    className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                >
                    Ping (Send 1 EGLD)
                </button>
                
                <button 
                    onClick={pong}
                    className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
                >
                    Pong (Claim EGLD)
                </button>
            </div>
        </div>
    );
}; 