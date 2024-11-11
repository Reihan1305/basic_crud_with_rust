FROM grafana/k6:latest

WORKDIR /test

COPY load_test.js /test/
COPY api /test/api/

RUN export K6_WEB_DASHBOARD=true
# Explicitly set the entrypoint and command
ENTRYPOINT ["/usr/bin/k6"]
CMD ["run", "--out", "experimental-prometheus-rw=http://localhost:9090/api/v1/write", "load_test.js"]
