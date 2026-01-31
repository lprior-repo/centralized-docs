---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#43-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary: For organizations that don’t allow the use of docker, podman. login [https://docs
---


For organizations that don’t allow the use of docker, podman
login [https://docs.podman.io/en/latest/markdown/podman-login.1.html] allows
using the --compat-auth-file $HOME/.docker/config.json flag to generate a
docker compatible json file.

GLOSSARY

build constraint: A condition that determines whether a CUE source file is
used when compiling a package. Build constraints are expressed with file-level @if(name)
annotations.

build list: The list of module versions that will be used for a CUE
command such as cue export, or cue vet. The build list is
