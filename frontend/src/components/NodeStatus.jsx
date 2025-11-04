import React from 'react';

/**
 * Display node online/offline status
 */
function NodeStatus({ nodes }) {
  return (
    <div className="bg-white p-4 rounded shadow">
      <h2 className="text-lg font-bold">Node Status</h2>
      <ul>
        {nodes.map(node => (
          <li key={node.id} className={node.online ? 'text-green-600' : 'text-red-600'}>
            {node.id}: {node.online ? 'Online' : 'Offline'}
          </li>
        ))}
      </ul>
    </div>
  );
}

export default NodeStatus;
