import React, { useEffect, useState } from 'react';
import NodeStatus from '../components/NodeStatus';
import { getHealth } from '../services/api';

/**
 * Nodes page: Shows health status of all nodes
 */
function Nodes() {
  const [nodes, setNodes] = useState([]);

  useEffect(() => {
    const fetchNodes = async () => {
      const health = await getHealth();
      if (health.nodes) setNodes(health.nodes);
    };
    fetchNodes();
  }, []);

  return <NodeStatus nodes={nodes} />;
}

export default Nodes;
