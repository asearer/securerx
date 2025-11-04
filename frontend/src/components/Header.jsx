import React from 'react';
import { Link } from 'react-router-dom';

/**
 * Header component with navigation
 */
function Header() {
  return (
    <header className="bg-blue-600 text-white p-4 flex justify-between items-center">
      <h1 className="text-xl font-bold">SecureRx Dashboard</h1>
      <nav className="space-x-4">
        <Link className="hover:underline" to="/">Home</Link>
        <Link className="hover:underline" to="/transactions">Transactions</Link>
        <Link className="hover:underline" to="/nodes">Nodes</Link>
      </nav>
    </header>
  );
}

export default Header;
