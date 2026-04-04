---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#12-summary
chunk_level: summary
chunk_type: prose
heading: Example
token_count: 87
summary: ## Example *This example assumes you have already set up a cluster with AppArmor support.* First, load the profile you want to use onto your Nodes. This profile blocks all file write operations: ```...
---

## Example
*This example assumes you have already set up a cluster with AppArmor support.*
First, load the profile you want to use onto your Nodes. This profile blocks all file write operations:
```
`#include &lt;tunables/global&gt;
profile k8s-apparmor-example-deny-write flags=(attach\_disconnected) {
# Deny all file writes.
deny /\*\* w,
}
`
```