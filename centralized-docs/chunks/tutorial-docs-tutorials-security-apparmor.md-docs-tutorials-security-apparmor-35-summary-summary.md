---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#35-summary
chunk_level: summary
chunk_type: prose
heading: Authoring Profiles
token_count: 109
summary: `type` *(required)* - indicates which kind of AppArmor profile will be applied. Valid options are: `Localhost`a profile pre-loaded on the node (specified by `localhostProfile`).`RuntimeDefault`the...
---

`type` *(required)* - indicates which kind of AppArmor profile will be applied. Valid options are:
`Localhost`a profile pre-loaded on the node (specified by `localhostProfile`).`RuntimeDefault`the container runtime's default profile.`Unconfined`no AppArmor enforcement.
`localhostProfile` - The name of a profile loaded on the node that should be used.
The profile must be preconfigured on the node to work.
This option must be provided if and only if the `type` is `Localhost`.