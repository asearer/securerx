import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
    vus: 50,
    duration: '1m',
};

export default function () {
    let payload = JSON.stringify({
        doctor_id: `doctor${Math.floor(Math.random() * 100)}`,
        patient_id: `patient${Math.floor(Math.random() * 100)}`,
        drug: "Drug A"
    });

    let params = { headers: { 'Content-Type': 'application/json' } };

    let res = http.post('http://api:8080/prescription', payload, params);

    check(res, { 'status 201': (r) => r.status === 201 });
    sleep(0.1);
}
