---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#19-summary
chunk_level: summary
chunk_type: prose
heading: Uses for Secrets
token_count: 98
summary: ### Use case: dotfiles in a secret volume You can make your data \"hidden\" by defining a key that begins with a dot. This key represents a dotfile or \"hidden\" file. For example, when the following...
---

### Use case: dotfiles in a secret volume
You can make your data "hidden" by defining a key that begins with a dot.
This key represents a dotfile or "hidden" file. For example, when the following Secret
is mounted into a volume, `secret-volume`, the volume will contain a single file,
called `.secret-file`, and the `dotfile-test-container` will have this file
present at the path `/etc/secret-volume/.secret-file`.