---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#34-summary
chunk_level: summary
chunk_type: prose
heading: Authoring Profiles
token_count: 123
summary: You can specify the `appArmorProfile` on either a container's `securityContext` or on a Pod's `securityContext`. If the profile is set at the pod level, it will be used as the default profile for all...
---

You can specify the `appArmorProfile` on either a container's `securityContext` or on a Pod's
`securityContext`. If the profile is set at the pod level, it will be used as the default profile
for all containers in the pod (including init, sidecar, and ephemeral containers). If both a pod &amp; container
AppArmor profile are set, the container's profile will be used.
An AppArmor profile has 2 fields:
`type` *(required)* - indicates which kind of AppArmor profile will be applied. Valid options are:
`Localhost`