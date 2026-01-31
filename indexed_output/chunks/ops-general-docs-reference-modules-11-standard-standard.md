---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#11-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 529
summary: documented under cue help environment [/docs/reference/command/cue-help-environment/]. AUTHORIZATION
---

documented under cue help environment [/docs/reference/command/cue-help-environment/].

AUTHORIZATION

For custom OCI registries, CUE understands the usual conventions
for authorization: specifically the usual way to configure
registry authorization information for custom OCI registries
is by setting them up in the $HOME/.docker/config.json file.
You can
use docker login [https://docs.docker.com/engine/reference/commandline/login/]
to do this or
edit the file directly [https://www.flatcar.org/docs/latest/container-runtimes/registry-authentication/].

The CUE command knows how to read auth tokens from the $HOME/.docker/config.json,
including running helper commands to fetch them from secure storage.

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
determined from the main module’s [/docs/reference/modules/#glos-main-module] cue.mod/module.cue
file [/docs/reference/modules/#glos-cue-mod-file] and cue.mod/module.cue files in transitively required modules
using minimal version selection [/docs/reference/modules/#glos-minimal-version-selection]. The build
list contains versions for all modules in the module
graph [/docs/reference/modules/#glos-module-graph], not just those relevant to a specific command.

canonical version: A correctly formatted version [/docs/reference/modules/#glos-version] without
a build metadata suffix other than +incompatible. For example, v1.2.3
is a canonical version, but v1.2.3+meta is not.

current module: Synonym for main module [/docs/reference/modules/#glos-main-module].

cue.mod/module.cue file: The file that defines a module’s path, requirements, and
