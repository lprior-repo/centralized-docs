---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#18-summary
chunk_level: summary
chunk_type: prose
heading: Example
token_count: 76
summary: ``` `kubectl exec hello-apparmor -- touch /tmp/test ` ``` ``` `touch: /tmp/test: Permission denied error: error executing remote command: command terminated with non-zero exit code: Error executing...
---

```
`kubectl exec hello-apparmor -- touch /tmp/test
`
```
```
`touch: /tmp/test: Permission denied
error: error executing remote command: command terminated with non-zero exit code: Error executing in Docker Container: 1
`
```
To wrap up, see what happens if you try to specify a profile that hasn't been loaded: