# Security Requirements

All API endpoints must be served over HTTPS (port 443) with a certificate
signed by a recognized certificate authority. In particular, self-signed
certificates are not permitted. Further, while an API _may_ be available over
HTTP (port 80), HTTPS must always be available. The authors highly recommend
servers redirect HTTP to HTTPS rather than serve your API on two separate
ports.
