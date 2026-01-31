---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1024
summary:  * Empty directories (entries with paths ending with a slash) may be included.    in module zip files but are not extracted
---

 * Empty directories (entries with paths ending with a slash) may be included
   in module zip files but are not extracted. The cue command does not include
   empty directories in zip files it creates.
 * Symbolic links and other irregular files are ignored when creating zip files,
   since they aren’t portable across operating systems and file systems, and
   there’s no portable way to represent them in the zip file format.
 * Files within directories containing cue.mod directories, other than the module
   root directory and the cue.mod directory itself, are ignored when creating zip files,
   since they are not part
   of the module. CUE ignores subdirectories containing cue.mod
   directories when extracting zip files.
 * No two files within a zip file may have paths equal under Unicode case-folding
   (see strings.EqualFold [https://pkg.go.dev/strings?tab=doc#EqualFold]).
   This ensures that zip files can be extracted on case-insensitive file systems
   without collisions.
 * A cue.mod/module.cue file must appear in the top-level directory.
   If present, it must have the name cue.mod/module.cue (all
   lowercase). Directories named cue.mod are not allowed in any other directory.
 * File and directory names within a module may consist of Unicode letters, ASCII
   digits, the ASCII space character (U+0020), and the ASCII punctuation
   characters !#$%&()+,-.=@[]^_{}~. Note that package paths may not contain all
   these characters. See
   module.CheckFilePath [https://pkg.go.dev/cuelang.org/go/internal/mod/module?tab=doc#CheckFilePath]
   and
   module.CheckImportPath [https://pkg.go.dev/golang.org/x/mod/module?tab=doc#CheckImportPath]
   for the differences.
 * A file or directory name up to the first dot must not be a
   reserved file name on Windows [https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file#naming-conventions],
   regardless of case (CON, com1, NuL, and so on).

MODULE CACHING

By default, the cue command caches downloaded modules in the local
filesystem. It uses the local user configuration directory by default, but
that can be changed by setting $CUE_CACHE_DIR, which is
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
