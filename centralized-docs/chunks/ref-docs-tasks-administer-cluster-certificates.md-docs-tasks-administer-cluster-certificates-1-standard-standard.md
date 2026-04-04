---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 511
summary: # Generate Certificates Manually When using client certificate authentication, you can generate certificates manually through [`easyrsa`](https://github.com/OpenVPN/easy-rsa),...
---

# Generate Certificates Manually
When using client certificate authentication, you can generate certificates
manually through [`easyrsa`](https://github.com/OpenVPN/easy-rsa), [`openssl`](https://github.com/openssl/openssl) or [`cfssl`](https://github.com/cloudflare/cfssl).
### easyrsa
**easyrsa** can manually generate certificates for your cluster.
1. Download, unpack, and initialize the patched version of `easyrsa3`.
```
`curl -LO https://dl.k8s.io/easy-rsa/easy-rsa.tar.gz
tar xzf easy-rsa.tar.gz
cd easy-rsa-master/easyrsa3
./easyrsa init-pki
`
```
2. Generate a new certificate authority (CA). `--batch` sets automatic mode;
`--req-cn` specifies the Common Name (CN) for the CA's new root certificate.
```
`./easyrsa --batch "--req-cn=${MASTER\_IP}@`date +%s`" build-ca nopass
`
```
3. Generate server certificate and key.
The argument `--subject-alt-name` sets the possible IPs and DNS names the API server will
be accessed with. The `MASTER\_CLUSTER\_IP` is usually the first IP from the service CIDR
that is specified as the `--service-cluster-ip-range` argument for both the API server and
the controller manager component. The argument `--days` is used to set the number of days
after which the certificate expires.
The sample below also assumes that you are using `cluster.local` as the default
DNS domain name.
```
`./easyrsa --subject-alt-name="IP:${MASTER\_IP},"\\
"IP:${MASTER\_CLUSTER\_IP},"\\
"DNS:kubernetes,"\\
"DNS:kubernetes.default,"\\
"DNS:kubernetes.default.svc,"\\
"DNS:kubernetes.default.svc.cluster,"\\
"DNS:kubernetes.default.svc.cluster.local" \\
--days=10000 \\
build-server-full server nopass
`
```
4. Copy `pki/ca.crt`, `pki/issued/server.crt`, and `pki/private/server.key` to your directory.
5. Fill in and add the following parameters into the API server start parameters:
```
`--client-ca-file=/yourdirectory/ca.crt
--tls-cert-file=/yourdirectory/server.crt
--tls-private-key-file=/yourdirectory/server.key
`
```