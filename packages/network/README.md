To run the `bin/tls_server.rs`, you need to generate a self-signed certificate and key pair. You can do this with the following command:

```bash
mkdir self_signed_certs
cd self_signed_certs
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout server.key -out server.crt -subj '/CN=localhost/'
```
