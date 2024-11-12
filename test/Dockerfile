FROM grafana/k6:latest

# Set the working directory inside the container
WORKDIR /test

# Copy your test script and API folder into the container
COPY load_test.js /test/
COPY api /test/api/
ENV K6_PROMETHEUS_RW_SERVER_URL=http://prometheus:9090/api/v1/write
# Set environment variables  persistently for the container
ENV K6_WEB_DASHBOARD=true

# Explicitly set the entrypoint and command to run the load test
ENTRYPOINT ["k6", "run", "--out", "experimental-prometheus-rw", "load_test.js"]