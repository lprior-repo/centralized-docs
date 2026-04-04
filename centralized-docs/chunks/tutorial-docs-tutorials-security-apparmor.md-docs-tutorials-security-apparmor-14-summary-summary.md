---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#14-summary
chunk_level: summary
chunk_type: prose
heading: Example
token_count: 102
summary: ``` `# This example assumes that node names match host names, and are reachable via SSH. NODES=($( kubectl get node -o jsonpath='{.items[\*].status.addresses[?(.type == \"Hostname\")].address}' )) for...
---

```
`# This example assumes that node names match host names, and are reachable via SSH.
NODES=($( kubectl get node -o jsonpath='{.items[\*].status.addresses[?(.type == "Hostname")].address}' ))
for NODE in ${NODES[\*]}; do ssh $NODE 'sudo apparmor\_parser -q &lt;&lt;EOF
# Deny all file writes.
deny /\*\* w,
}
EOF'
done
`
```