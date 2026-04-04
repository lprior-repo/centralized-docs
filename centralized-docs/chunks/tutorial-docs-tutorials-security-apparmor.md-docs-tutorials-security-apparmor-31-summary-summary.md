---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#31-summary
chunk_level: summary
chunk_type: prose
heading: Authoring Profiles
token_count: 113
summary: * [bane](https://github.com/jfrazelle/bane) is an AppArmor profile generator for Docker that uses a simplified profile language. To debug problems with AppArmor, you can check the system logs to see...
---

* [bane](https://github.com/jfrazelle/bane) is an AppArmor profile generator for Docker that uses a
simplified profile language.
To debug problems with AppArmor, you can check the system logs to see what, specifically, was
denied. AppArmor logs verbose messages to `dmesg`, and errors can usually be found in the system
logs or through `journalctl`. More information is provided in
[AppArmor failures](https://gitlab.com/apparmor/apparmor/wikis/AppArmor_Failures).