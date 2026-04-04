---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#18-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 106
summary: `openssl req -subj \"/CN=system:konnectivity-server\" -new -newkey rsa:2048 -noenc -out konnectivity.csr -keyout konnectivity.key openssl x509 -req -in konnectivity.csr -CA /etc/kubernetes/pki/ca.crt...
---

`openssl req -subj "/CN=system:konnectivity-server" -new -newkey rsa:2048 -noenc -out konnectivity.csr -keyout konnectivity.key
openssl x509 -req -in konnectivity.csr -CA /etc/kubernetes/pki/ca.crt -CAkey /etc/kubernetes/pki/ca.key -CAcreateserial -out konnectivity.crt -days 375 -sha256
SERVER=$(kubectl config view -o jsonpath='{.clusters..server}')