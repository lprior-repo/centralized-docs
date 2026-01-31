---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#6-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1024
summary: current module: Synonym for main module [/docs/reference/modules/#glos-main-module]. cue file: The file that defines a module’s path, requirements, and
---


current module: Synonym for main module [/docs/reference/modules/#glos-main-module].

cue.mod/module.cue file: The file that defines a module’s path, requirements, and
other metadata. Appears in the module’s root
directory [/docs/reference/modules/#glos-module-root-directory]. See the section on cue.mod/module.cue
files [/docs/reference/modules/#cue-mod-file].

import path: A string used to import a package in a CUE source file.
Synonymous with package path [/docs/reference/modules/#glos-package-path].

main module: The module in which the cue command is invoked. The main
module is defined by a cue.mod/module.cue file [/docs/reference/modules/#glos-cue-mod-file] in the current
directory or a parent directory. See Modules, packages, and
versions [/docs/reference/modules/#modules-overview].

major version: The first number in a semantic version (1 in v1.2.3). In
a release with incompatible changes, the major version must be incremented, and
the minor and patch versions must be set to 0. Semantic versions with major
version 0 are considered unstable.

major version suffix: A module path suffix that matches the major version
number. For example, @v2 in foo.example/mod@v2. See
the section on Major version suffixes [/docs/reference/modules/#major-version-suffixes].

minimal version selection (MVS): The algorithm used to determine the
versions of all modules that will be used in a build. See the section on
Minimal version selection [/docs/reference/modules/#minimal-version-selection] for details.

minor version: The second number in a semantic version (2 in v1.2.3). In
a release with new, backwards compatible functionality, the minor version must
be incremented, and the patch version must be set to 0.

module: A collection of packages that are released, versioned, and
distributed together.

module cache: A local directory storing downloaded modules, located in
$CUE_CACHE_DIR. See Module cache [/docs/reference/modules/#module-cache].

module graph: The directed graph of module requirements, rooted at the main
module [/docs/reference/modules/#glos-main-module]. Each vertex in the graph is a module; each edge is a
version from an entry in the deps field in a cue.mod/module.cue file.

module path: A path that identifies a module and acts as a prefix for
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
