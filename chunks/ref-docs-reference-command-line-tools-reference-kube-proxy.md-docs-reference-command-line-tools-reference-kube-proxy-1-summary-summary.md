---
doc_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy
chunk_id: ref/docs-reference-command-line-tools-reference-kube-proxy.md/docs-reference-command-line-tools-reference-kube-proxy#1-summary
chunk_level: summary
chunk_type: prose
heading: Synopsis
token_count: 123
summary: ## Synopsis The Kubernetes network proxy runs on each node. This reflects services as defined in the Kubernetes API on each node and can do simple TCP, UDP, and SCTP stream forwarding or round robin...
---

## Synopsis
The Kubernetes network proxy runs on each node. This
reflects services as defined in the Kubernetes API on each node and can do simple
TCP, UDP, and SCTP stream forwarding or round robin TCP, UDP, and SCTP forwarding across a set of backends.
Service cluster IPs and ports are currently found through Docker-links-compatible
environment variables specifying ports opened by the service proxy. There is an optional
addon that provides cluster DNS for these cluster IPs. The user must create a service
with the apiserver API to configure the proxy.
```
`kube-proxy [flags]
`
```