---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#10-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 116
summary: Where `&lt;profile\_type&gt;` is one of: * `RuntimeDefault` to use the runtime's default profile * `Localhost` to use a profile loaded on the host (see below) * `Unconfined` to run without AppArmor...
---

Where `&lt;profile\_type&gt;` is one of:
* `RuntimeDefault` to use the runtime's default profile
* `Localhost` to use a profile loaded on the host (see below)
* `Unconfined` to run without AppArmor
See [Specifying AppArmor Confinement](#specifying-apparmor-confinement) for full details on the AppArmor profile API.
To verify that the profile was applied, you can check that the container's root process is
running with the correct profile by examining its proc attr: