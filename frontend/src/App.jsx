import React, { useState } from 'react';
import Header from './components/Header';
import Overview from './pages/Overview';
import Transactions from './pages/Transactions';
import Nodes from './pages/Nodes';
import SubmitPrescription from './pages/SubmitPrescription';

/**
 * App component – main application with tab-based navigation
 */
function App() {
  const [activeTab, setActiveTab] = useState('overview');

  return (
    <div>
      <Header activeTab={activeTab} setActiveTab={setActiveTab} />
      <main>
        <section id="overview" className={activeTab === 'overview' ? 'active' : ''}>
          <Overview />
        </section>
        <section id="transactions" className={activeTab === 'transactions' ? 'active' : ''}>
          <Transactions />
        </section>
        <section id="nodes" className={activeTab === 'nodes' ? 'active' : ''}>
          <Nodes />
        </section>
        <section id="submit" className={activeTab === 'submit' ? 'active' : ''}>
          <SubmitPrescription />
        </section>
      </main>
      <footer>&copy; 2025 SecureRx</footer>
    </div>
  );
}

export default App;
