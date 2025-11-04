import React, { useEffect, useState } from 'react';
import { getHealth, getBlocks } from '../services/api';

/**
 * Dashboard component shows chain height, blocks, transactions
 */
function Dashboard() {
  const [blocks, setBlocks] = useState([]);
  const [health, setHealth] = useState({});

  useEffect(() => {
    const fetchData = async () => {
      const blockData = await getBlocks();
      const healthData = await getHealth();
      setBlocks(blockData);
      setHealth(healthData);
    };
    fetchData();
  }, []);

  return (
    <div className="space-y-4">
      <div className="bg-white p-4 rounded shadow">
        <h2 className="text-lg font-bold">System Health</h2>
        <p>API Status: {health.status || 'Loading...'}</p>
        <p>Chain Height: {blocks.length}</p>
      </div>
    </div>
  );
}

export default Dashboard;
