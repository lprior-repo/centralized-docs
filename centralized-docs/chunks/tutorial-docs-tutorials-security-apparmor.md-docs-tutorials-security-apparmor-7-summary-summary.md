---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#7-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 113
summary: 3. Profile is loaded -- AppArmor is applied to a Pod by specifying an AppArmor profile that each container should be run with. If any of the specified profiles are not loaded in the kernel, the...
---

3. Profile is loaded -- AppArmor is applied to a Pod by specifying an AppArmor profile that each
container should be run with. If any of the specified profiles are not loaded in the
kernel, the kubelet will reject the Pod. You can view which profiles are loaded on a
node by checking the `/sys/kernel/security/apparmor/profiles` file. For example:
```
`ssh gke-test-default-pool-239f5d02-gyn2 "sudo cat /sys/kernel/security/apparmor/profiles | sort"
`
```