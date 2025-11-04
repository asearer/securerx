import React, { useEffect, useState } from 'react';
import { getBlocks } from '../services/api';
import BlockList from '../components/BlockList';

/**
 * Transactions page: List all blockchain blocks
 */
function Transactions() {
  const [blocks, setBlocks] = useState([]);

  useEffect(() => {
    const fetchBlocks = async () => {
      const data = await getBlocks();
      setBlocks(data);
    };
    fetchBlocks();
  }, []);

  return <BlockList blocks={blocks} />;
}

export default Transactions;
