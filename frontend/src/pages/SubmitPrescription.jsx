import React, { useState } from 'react';
import { submitPrescription } from '../services/api';

/**
 * Submit Prescription page: Form for submitting new prescriptions
 */
function SubmitPrescription() {
  const [doctorId, setDoctorId] = useState('');
  const [patientId, setPatientId] = useState('');
  const [drug, setDrug] = useState('');
  const [status, setStatus] = useState('');

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      await submitPrescription(doctorId, patientId, drug);
      setStatus(`Prescription submitted: Doctor ${doctorId}, Patient ${patientId}, Drug ${drug}`);
      // Reset form
      setDoctorId('');
      setPatientId('');
      setDrug('');
    } catch (err) {
      setStatus('Failed to submit prescription.');
    }
  };

  return (
    <div className="card">
      <h2>Submit Prescription</h2>
      <form id="prescriptionForm" onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="doctor">Doctor ID</label>
          <input 
            type="text" 
            id="doctor" 
            required 
            value={doctorId}
            onChange={(e) => setDoctorId(e.target.value)}
          />
        </div>
        <div className="form-group">
          <label htmlFor="patient">Patient ID</label>
          <input 
            type="text" 
            id="patient" 
            required 
            value={patientId}
            onChange={(e) => setPatientId(e.target.value)}
          />
        </div>
        <div className="form-group">
          <label htmlFor="drug">Drug</label>
          <input 
            type="text" 
            id="drug" 
            required 
            value={drug}
            onChange={(e) => setDrug(e.target.value)}
          />
        </div>
        <button type="submit" className="submit">Submit</button>
      </form>
      {status && (
        <p id="formStatus" style={{ color: status.includes('Failed') ? 'red' : 'green', marginTop: '1rem' }}>
          {status}
        </p>
      )}
    </div>
  );
}

export default SubmitPrescription;
