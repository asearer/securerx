import React from 'react';
import Dashboard from '../components/Dashboard';
import PrescriptionForm from '../components/PrescriptionForm';

/**
 * Home page: Shows dashboard and prescription form
 */
function Home() {
  return (
    <div className="space-y-6">
      <Dashboard />
      <PrescriptionForm />
    </div>
  );
}

export default Home;
