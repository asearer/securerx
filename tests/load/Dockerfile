FROM grafana/k6
WORKDIR /app
COPY load_test.js /app/load_test.js
ENTRYPOINT ["k6", "run", "/app/load_test.js"]
