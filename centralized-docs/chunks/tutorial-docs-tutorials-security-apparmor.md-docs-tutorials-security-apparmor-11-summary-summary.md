---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#11-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 49
summary: ``` `kubectl exec &lt;pod\_name&gt; -- cat /proc/1/attr/current ` ``` The output should look something like this: ``` `cri-containerd.apparmor.d (enforce) ` ```
---

```
`kubectl exec &lt;pod\_name&gt; -- cat /proc/1/attr/current
`
```
The output should look something like this:
```
`cri-containerd.apparmor.d (enforce)
`
```