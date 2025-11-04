import React from 'react';

/**
 * List all blocks in blockchain
 */
function BlockList({ blocks }) {
  return (
    <div className="bg-white p-4 rounded shadow">
      <h2 className="text-lg font-bold">Blocks</h2>
      <ul>
        {blocks.map(block => (
          <li key={block.index} className="border-b py-1">
            Index: {block.index}, Transactions: {block.transactions.length}, Hash: {block.hash.slice(0, 10)}...
          </li>
        ))}
      </ul>
    </div>
  );
}

export default BlockList;
