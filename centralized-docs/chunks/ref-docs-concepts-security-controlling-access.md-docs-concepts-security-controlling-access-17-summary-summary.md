---
doc_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access
chunk_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access#17-summary
chunk_level: summary
chunk_type: prose
heading: Admission control
token_count: 110
summary: **3** in the diagram. Unlike Authentication and Authorization modules, if any admission controller module rejects, then the request is immediately rejected. In addition to rejecting objects,...
---

**3** in the diagram.
Unlike Authentication and Authorization modules, if any admission controller module
rejects, then the request is immediately rejected.
In addition to rejecting objects, admission controllers can also set complex defaults for
fields.
The available Admission Control modules are described in [Admission Controllers](/docs/reference/access-authn-authz/admission-controllers/).
Once a request passes all admission controllers, it is validated using the validation routines
for the corresponding API object, and then written to the object store (shown as step **4**).