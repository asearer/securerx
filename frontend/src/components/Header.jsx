import React from 'react';

/**
 * Header component with navigation buttons
 */
function Header({ activeTab, setActiveTab }) {
  return (
    <>
      <header>SecureRx Dashboard</header>
      <nav>
        <button 
          className={activeTab === 'overview' ? 'active' : ''}
          onClick={() => setActiveTab('overview')}
          data-target="overview"
        >
          Overview
        </button>
        <button 
          className={activeTab === 'transactions' ? 'active' : ''}
          onClick={() => setActiveTab('transactions')}
          data-target="transactions"
        >
          Transactions
        </button>
        <button 
          className={activeTab === 'nodes' ? 'active' : ''}
          onClick={() => setActiveTab('nodes')}
          data-target="nodes"
        >
          Nodes
        </button>
        <button 
          className={activeTab === 'submit' ? 'active' : ''}
          onClick={() => setActiveTab('submit')}
          data-target="submit"
        >
          Submit Prescription
        </button>
      </nav>
    </>
  );
}

export default Header;
