---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#25-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 127
summary: `Name: my-service Namespace: default Labels: app.kubernetes.io/name=MyApp Annotations: &lt;none&gt; Selector: app.kubernetes.io/name=MyApp Type: ClusterIP IP Family Policy: PreferDualStack IP...
---

`Name: my-service
Namespace: default
Labels: app.kubernetes.io/name=MyApp
Annotations: &lt;none&gt;
Selector: app.kubernetes.io/name=MyApp
Type: ClusterIP
IP Family Policy: PreferDualStack
IP Families: IPv4,IPv6
IP: 10.0.216.242
IPs: 10.0.216.242,2001:db8:fd00::af55
Port: &lt;unset&gt; 80/TCP
TargetPort: 9376/TCP
Endpoints: &lt;none&gt;