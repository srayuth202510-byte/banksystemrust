# Production Secrets

Store production-only TLS and Redis secret material in this directory.

Required files for `docker-compose.prod.yml`:

- `gateway.crt`
- `gateway.key`
- `ca.crt`
- `redis_password.txt`

Recommended format:

- `gateway.crt`: PEM-encoded gateway certificate
- `gateway.key`: PEM-encoded private key
- `ca.crt`: PEM-encoded CA certificate used by internal clients
- `redis_password.txt`: single-line Redis password, no trailing spaces

Do not commit the secret values. This directory is ignored except for this template.
