import axios from 'axios';

/**
 * Axios instance for API calls
 * BASE_URL points to the SecureRx API container
 */
const BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

export const api = axios.create({
  baseURL: BASE_URL,
  headers: { 'Content-Type': 'application/json' }
});

/**
 * Submit a new prescription
 */
export const submitPrescription = async (doctorId, patientId, drug) => {
  const response = await api.post('/prescription', { doctor_id: doctorId, patient_id: patientId, drug });
  return response.data;
};

/**
 * Get all blocks from blockchain
 */
export const getBlocks = async () => {
  const response = await api.get('/blocks');
  return response.data;
};

/**
 * Get system health
 */
export const getHealth = async () => {
  const response = await api.get('/health');
  return response.data;
};
