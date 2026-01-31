---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#13-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 522
summary: package import paths within the module.  For example, \"cuelang
---

package import paths within the module. For example, "cuelang.org/x/foo".

module root directory: The directory that contains the cue.mod/module.cue file that
defines a module.

package: A collection of source files, usually in the
same directory, that are evaluated together. See the Packages
section [https://cuelang.org/docs/references/spec/#modules-instances-and-packages]
in the CUE Language Specification.

package path: The path that uniquely identifies a package. A package path is
a module path [/docs/reference/modules/#glos-module-path] joined with a subdirectory within the module.
For example "cuelang.org/x/foo/html" is the package path for the package in the
module "cuelang.org/x/foo" in the "html" subdirectory. Synonym of
import path [/docs/reference/modules/#glos-import-path].

patch version: The third number in a semantic version (3 in v1.2.3). In
a release with no changes to the module’s public interface, the patch version
must be incremented.

pre-release version: A version with a dash followed by a series of
dot-separated identifiers immediately following the patch version, for example,
v1.2.3-beta4. Pre-release versions are considered unstable and are not
assumed to be compatible with other versions. A pre-release version sorts before
the corresponding release version: v1.2.3-pre comes before v1.2.3. See also
release version [/docs/reference/modules/#glos-release-version].

release version: A version without a pre-release suffix. For example,
v1.2.3, not v1.2.3-pre. See also pre-release
version [/docs/reference/modules/#glos-pre-release-version].

repository root path: The portion of a module path [/docs/reference/modules/#glos-module-path] that
corresponds to a version control repository’s root directory. See Module
paths [/docs/reference/modules/#module-path].

selected version: The version of a given module chosen by minimal version
selection [/docs/reference/modules/#minimal-version-selection]. The selected version is the highest
version for the module’s path found in the module graph [/docs/reference/modules/#glos-module-graph].
