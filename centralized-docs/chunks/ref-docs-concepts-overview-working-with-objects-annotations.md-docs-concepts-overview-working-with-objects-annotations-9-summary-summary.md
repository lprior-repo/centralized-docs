---
doc_id: ref/docs-concepts-overview-working-with-objects-annotations.md/docs-concepts-overview-working-with-objects-annotations
chunk_id: ref/docs-concepts-overview-working-with-objects-annotations.md/docs-concepts-overview-working-with-objects-annotations#9-summary
chunk_level: summary
chunk_type: prose
heading: Syntax and character set
token_count: 126
summary: *Annotations* are key/value pairs. Valid annotation keys have two segments: an optional prefix and name, separated by a slash (`/`). The name segment is required and must be 63 characters or less,...
---

*Annotations* are key/value pairs. Valid annotation keys have two segments: an optional prefix and name, separated by a slash (`/`). The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character (`[a-z0-9A-Z]`) with dashes (`-`), underscores (`\_`), dots (`.`), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (`.`), not longer than 253 characters in total, followed by a slash (`/`).