---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#11-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 125
summary: `[ req ] default\_bits = 2048 prompt = no default\_md = sha256 req\_extensions = req\_ext distinguished\_name = dn [ dn ] C = &lt;country&gt; ST = &lt;state&gt; L = &lt;city&gt; O =...
---

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