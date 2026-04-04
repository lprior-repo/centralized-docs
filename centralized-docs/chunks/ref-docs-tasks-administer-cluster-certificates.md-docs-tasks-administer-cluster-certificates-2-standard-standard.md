---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#2-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 497
summary: ### openssl **openssl** can manually generate certificates for your cluster. 1. Generate a ca.key with 2048bit: ``` `openssl genrsa -out ca.key 2048 ` ``` 2. According to the ca.key generate a ca.crt...
---

### openssl
**openssl** can manually generate certificates for your cluster.
1. Generate a ca.key with 2048bit:
```
`openssl genrsa -out ca.key 2048
`
```
2. According to the ca.key generate a ca.crt (use `-days` to set the certificate effective time):
```
`openssl req -x509 -new -noenc -key ca.key -subj "/CN=${MASTER\_IP}" -days 10000 -out ca.crt
`
```
3. Generate a server.key with 2048bit:
```
`openssl genrsa -out server.key 2048
`
```
4. Create a config file for generating a Certificate Signing Request (CSR).
Be sure to substitute the values marked with angle brackets (e.g. `&lt;MASTER\_IP&gt;`)
with real values before saving this to a file (e.g. `csr.conf`).
Note that the value for `MASTER\_CLUSTER\_IP` is the service cluster IP for the
API server as described in previous subsection.
The sample below also assumes that you are using `cluster.local` as the default
DNS domain name.
```
`[ req ]
default\_bits = 2048
prompt = no
default\_md = sha256
req\_extensions = req\_ext
distinguished\_name = dn
[ dn ]
C = &lt;country&gt;
ST = &lt;state&gt;
L = &lt;city&gt;
O = &lt;organization&gt;
OU = &lt;organization unit&gt;
CN = &lt;MASTER\_IP&gt;
[ req\_ext ]
subjectAltName = @alt\_names
[ alt\_names ]
DNS.1 = kubernetes
DNS.2 = kubernetes.default
DNS.3 = kubernetes.default.svc
DNS.4 = kubernetes.default.svc.cluster
DNS.5 = kubernetes.default.svc.cluster.local
IP.1 = &lt;MASTER\_IP&gt;
IP.2 = &lt;MASTER\_CLUSTER\_IP&gt;
[ v3\_ext ]
authorityKeyIdentifier=keyid,issuer:always
basicConstraints=CA:FALSE
keyUsage=keyEncipherment,dataEncipherment
extendedKeyUsage=serverAuth,clientAuth
subjectAltName=@alt\_names
`
```
5. Generate the certificate signing request based on the config file: