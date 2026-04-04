---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#3-detailed
chunk_level: detailed
chunk_type: code
heading: Distributing Self-Signed CA Certificate
token_count: 971
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
```
`../cfssl gencert -initca ca-csr.json | ../cfssljson -bare ca
`
```
6. Create a JSON config file for generating keys and certificates for the API
server, for example, `server-csr.json`. Be sure to replace the values in angle brackets with
real values you want to use. The `&lt;MASTER\_CLUSTER\_IP&gt;` is the service cluster
IP for the API server as described in previous subsection.
The sample below also assumes that you are using `cluster.local` as the default
DNS domain name.
```
`{
"CN": "kubernetes",
"hosts": [
"127.0.0.1",
"&lt;MASTER\_IP&gt;",
"&lt;MASTER\_CLUSTER\_IP&gt;",
"kubernetes",
"kubernetes.default",
"kubernetes.default.svc",
"kubernetes.default.svc.cluster",
"kubernetes.default.svc.cluster.local"
],
"key": {
"algo": "rsa",
"size": 2048
},
"names": [{
"C": "&lt;country&gt;",
"ST": "&lt;state&gt;",
"L": "&lt;city&gt;",
"O": "&lt;organization&gt;",
"OU": "&lt;organization unit&gt;"
}]
}
`
```
7. Generate the key and certificate for the API server, which are by default
saved into file `server-key.pem` and `server.pem` respectively:
```
`../cfssl gencert -ca=ca.pem -ca-key=ca-key.pem \\
--config=ca-config.json -profile=kubernetes \\
server-csr.json | ../cfssljson -bare server
`
```
## Distributing Self-Signed CA Certificate
A client node may refuse to recognize a self-signed CA certificate as valid.
For a non-production deployment, or for a deployment that runs behind a company
firewall, you can distribute a self-signed CA certificate to all clients and
refresh the local list for valid certificates.
On each client, perform the following operations:
```
`sudo cp ca.crt /usr/local/share/ca-certificates/kubernetes.crt
sudo update-ca-certificates
`
```
```
`Updating certificates in /etc/ssl/certs...
1 added, 0 removed; done.
Running hooks in /etc/ca-certificates/update.d....
done.
`
```