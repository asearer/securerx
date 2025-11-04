import React, { useState } from 'react';
import { submitPrescription } from '../services/api';

/**
 * Form for submitting new prescriptions
 */
function PrescriptionForm() {
  const [doctorId, setDoctorId] = useState('');
  const [patientId, setPatientId] = useState('');
  const [drug, setDrug] = useState('');
  const [message, setMessage] = useState('');

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      await submitPrescription(doctorId, patientId, drug);
      setMessage('Prescription submitted successfully.');
    } catch (err) {
      setMessage('Failed to submit prescription.');
    }
  };

  return (
    <form onSubmit={handleSubmit} className="bg-white p-4 rounded shadow space-y-2">
      <h2 className="text-lg font-bold">New Prescription</h2>
      <input className="border p-2 w-full" placeholder="Doctor ID" value={doctorId} onChange={e => setDoctorId(e.target.value)} />
      <input className="border p-2 w-full" placeholder="Patient ID" value={patientId} onChange={e => setPatientId(e.target.value)} />
      <input className="border p-2 w-full" placeholder="Drug" value={drug} onChange={e => setDrug(e.target.value)} />
      <button className="bg-blue-600 text-white px-4 py-2 rounded" type="submit">Submit</button>
      {message && <p>{message}</p>}
    </form>
  );
}

export default PrescriptionForm;
