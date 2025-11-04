import React, { useEffect, useState } from 'react';
import { getBlocks } from '../services/api';

/**
 * Transactions page: List all prescription transactions
 */
function Transactions() {
  const [transactions, setTransactions] = useState([]);

  useEffect(() => {
    const fetchTransactions = async () => {
      try {
        const blocks = await getBlocks();
        // Extract transactions from blocks
        const allTransactions = [];
        blocks.forEach((block, blockIndex) => {
          block.transactions.forEach((tx, txIndex) => {
            allTransactions.push({
              id: `${blockIndex}-${txIndex}`,
              prescriptionId: tx.id || `${blockIndex}-${txIndex}`,
              doctor: tx.doctor_id || 'N/A',
              patient: tx.patient_id || 'N/A',
              drug: tx.drug || 'N/A',
              timestamp: tx.timestamp || new Date().toISOString().slice(0, 16).replace('T', ' ')
            });
          });
        });
        setTransactions(allTransactions);
      } catch (err) {
        console.error('Failed to fetch transactions:', err);
      }
    };
    fetchTransactions();
  }, []);

  return (
    <div className="card">
      <h2>Recent Transactions</h2>
      <table id="txTable">
        <thead>
          <tr>
            <th>Prescription ID</th>
            <th>Doctor</th>
            <th>Patient</th>
            <th>Drug</th>
            <th>Timestamp</th>
          </tr>
        </thead>
        <tbody>
          {transactions.length > 0 ? (
            transactions.map((tx) => (
              <tr key={tx.id}>
                <td>{tx.prescriptionId}</td>
                <td>{tx.doctor}</td>
                <td>{tx.patient}</td>
                <td>{tx.drug}</td>
                <td>{tx.timestamp}</td>
              </tr>
            ))
          ) : (
            <tr>
              <td colSpan="5" style={{ textAlign: 'center', color: '#777' }}>
                No transactions found
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
}

export default Transactions;
