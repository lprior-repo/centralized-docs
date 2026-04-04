---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#5-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 96
summary: 1. AppArmor kernel module is enabled -- For the Linux kernel to enforce an AppArmor profile, the AppArmor kernel module must be installed and enabled. Several distributions enable the module by...
---

1. AppArmor kernel module is enabled -- For the Linux kernel to enforce an AppArmor profile, the
AppArmor kernel module must be installed and enabled. Several distributions enable the module by
default, such as Ubuntu and SUSE, and many others provide optional support. To check whether the
module is enabled, check the `/sys/module/apparmor/parameters/enabled` file:
```
`cat /sys/module/apparmor/parameters/enabled
Y
`
```