---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#24-summary
chunk_level: summary
chunk_type: prose
heading: Example
token_count: 111
summary: Normal Pulling 7s (x2 over 9s) kubelet Pulling image \"busybox:1.28\" Warning Failed 7s (x2 over 8s) kubelet Error: failed to get container spec opts: failed to generate apparmor spec opts: apparmor...
---

Normal Pulling 7s (x2 over 9s) kubelet Pulling image "busybox:1.28"
Warning Failed 7s (x2 over 8s) kubelet Error: failed to get container spec opts: failed to generate apparmor spec opts: apparmor profile not found k8s-apparmor-example-allow-write
Normal Pulled 7s kubelet Successfully pulled image "busybox:1.28" in 90.980331ms (91.005869ms including waiting)
`
```