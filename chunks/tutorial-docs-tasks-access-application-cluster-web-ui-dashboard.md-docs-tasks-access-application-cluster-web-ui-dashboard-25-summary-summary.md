---
doc_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard
chunk_id: tutorial/docs-tasks-access-application-cluster-web-ui-dashboard.md/docs-tasks-access-application-cluster-web-ui-dashboard#25-summary
chunk_level: summary
chunk_type: prose
heading: Deploying containerized applications
token_count: 116
summary: * **Image Pull Secret**: In case the specified Docker container image is private, it may require [pull secret](/docs/concepts/configuration/secret/) credentials. Dashboard offers all available...
---

* **Image Pull Secret**:
In case the specified Docker container image is private, it may require
[pull secret](/docs/concepts/configuration/secret/) credentials.
Dashboard offers all available secrets in a dropdown list, and allows you to create a new secret.
The secret name must follow the DNS domain name syntax, for example `new.image-pull.secret`.
The content of a secret must be base64-encoded and specified in a
[`.dockercfg`](/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod) file.