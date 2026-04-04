---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#2-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 124
summary: ### easyrsa **easyrsa** can manually generate certificates for your cluster. 1. Download, unpack, and initialize the patched version of `easyrsa3`. ``` `curl -LO...
---

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