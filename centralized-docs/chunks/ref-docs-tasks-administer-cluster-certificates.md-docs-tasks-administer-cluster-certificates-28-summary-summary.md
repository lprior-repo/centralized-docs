---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#28-summary
chunk_level: summary
chunk_type: prose
heading: Certificates API
token_count: 120
summary: ``` `sudo cp ca.crt /usr/local/share/ca-certificates/kubernetes.crt sudo update-ca-certificates ` ``` ``` `Updating certificates in /etc/ssl/certs... 1 added, 0 removed; done. Running hooks in...
---

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
## Certificates API
You can use the `certificates.k8s.io` API to provision
x509 certificates to use for authentication as documented
in the [Managing TLS in a cluster](/docs/tasks/tls/managing-tls-in-a-cluster/)
task page.