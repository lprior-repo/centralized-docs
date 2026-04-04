---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#4-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 487
summary: ### cfssl **cfssl** is another tool for certificate generation. 1. Download, unpack and prepare the command line tools as shown below. Note that you may need to adapt the sample commands based on the...
---

### cfssl
**cfssl** is another tool for certificate generation.
1. Download, unpack and prepare the command line tools as shown below.
Note that you may need to adapt the sample commands based on the hardware
architecture and cfssl version you are using.
```
`curl -L https://github.com/cloudflare/cfssl/releases/download/v1.5.0/cfssl\_1.5.0\_linux\_amd64 -o cfssl
chmod +x cfssl
curl -L https://github.com/cloudflare/cfssl/releases/download/v1.5.0/cfssljson\_1.5.0\_linux\_amd64 -o cfssljson
chmod +x cfssljson
curl -L https://github.com/cloudflare/cfssl/releases/download/v1.5.0/cfssl-certinfo\_1.5.0\_linux\_amd64 -o cfssl-certinfo
chmod +x cfssl-certinfo
`
```
2. Create a directory to hold the artifacts and initialize cfssl:
```
`mkdir cert
cd cert
../cfssl print-defaults config &gt; config.json
../cfssl print-defaults csr &gt; csr.json
`
```
3. Create a JSON config file for generating the CA file, for example, `ca-config.json`:
```
`{
"signing": {
"default": {
"expiry": "8760h"
},
"profiles": {
"kubernetes": {
"usages": [
"signing",
"key encipherment",
"server auth",
"client auth"
],
"expiry": "8760h"
}
}
}
}
`
```
4. Create a JSON config file for CA certificate signing request (CSR), for example,
`ca-csr.json`. Be sure to replace the values marked with angle brackets with
real values you want to use.
```
`{
"CN": "kubernetes",
"key": {
"algo": "rsa",
"size": 2048
},
"names":[{
"C": "&lt;country&gt;",
"ST": "&lt;state&gt;",
"L": "&lt;city&gt;",
"O": "&lt;organization&gt;",
"OU": "&lt;organization unit&gt;"
}]
}
`
```
5. Generate CA key (`ca-key.pem`) and certificate (`ca.pem`):