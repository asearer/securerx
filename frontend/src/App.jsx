import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Home from './pages/Home';
import Transactions from './pages/Transactions';
import Nodes from './pages/Nodes';
import Header from './components/Header';

/**
 * App component – sets up routes for the frontend GUI
 */
function App() {
  return (
    <Router>
      <div className="min-h-screen bg-gray-100">
        <Header />
        <main className="p-6">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/transactions" element={<Transactions />} />
            <Route path="/nodes" element={<Nodes />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
}

export default App;
