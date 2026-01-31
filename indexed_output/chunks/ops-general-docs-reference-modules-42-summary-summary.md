---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#42-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: for authorization: specifically the usual way to configure. registry authorization information for custom OCI registries
---

for authorization: specifically the usual way to configure
registry authorization information for custom OCI registries
is by setting them up in the $HOME/.docker/config.json file.
You can
use docker login [https://docs.docker.com/engine/reference/commandline/login/]
to do this or
edit the file directly [https://www.flatcar.org/docs/latest/container-runtimes/registry-authentication/].

The CUE command knows how to read auth tokens from the $HOME/.docker/config.json,
including running helper commands to fetch them from secure storage.
