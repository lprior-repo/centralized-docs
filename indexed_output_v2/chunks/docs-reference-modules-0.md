---
doc_id: docs-reference-modules
chunk_id: docs-reference-modules#0
chunk_type: table
heading: Introduction
token_count: 8688
summary: # CUE Modules | CUE. **Source:** https://cuelang
---

# CUE Modules | CUE

**Source:** https://cuelang.org/docs/reference/modules/

Skip to content

Homepage of CUE [/]
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]

Install
[/docs/introduction/installation/]

Search [/search]

What are you looking for?

Menu

 1. References [https://cuelang.org/docs/reference/]


 2. CUE MODULES

rogpeppe [https://github.com/rogpeppe.png]
Roger Peppe
rogpeppe [https://github.com/rogpeppe.png]
Roger Peppe

Github profile

[https://github.com/rogpeppe]

Search all content by this author

[/search/?q=author:rogpeppe]
 * modules [/search?q=tag:modules]

INTRODUCTION

Modules are how CUE manages dependencies.
This document is a detailed reference manual for CUE’s module system.
CUE’s modules support has a lot in common with Go’s modules
and this document has substantial parts that have been taken
directly from the Go modules reference [https://go.dev/ref/mod].
Thanks very much to Russ Cox and the Go team for their
amazing work there.

This document largely supercedes the
prior modules documentation [/docs/concept/modules-packages-instances/]
although, as a transitionary measure, the CUE tool still supports
the import of packages present in the cue.mod/pkg, cue.mod/usr and
cue.mod/gen directories. This only applies to the main module, and if
there is any ambiguity with respect to regular module dependencies an
“ambiguous import” error will be reported.

MODULES, PACKAGES, AND VERSIONS

A module is a collection of packages that are released,
versioned, and distributed together. Modules are downloaded from
OCI-compliant [https://github.com/opencontainers/distribution-spec/blob/main/spec.md]
artifact registries. This means that if you are deploying CUE to the cloud,
you can use the same distribution mechanism that you might be using for
Docker images to deploy your CUE configuration too.

A module is identified by a module path, which is declared in a
cue.mod/module.cue file, together with information about the module’s
dependencies. The module root directory is the directory that contains
the cue.mod directory. The main module is the module containing the
directory where the cue command is invoked.

Each package within a module is a collection of source files that are
unified together, usually all in the same directory. A package path
is the module path joined with the subdirectory containing the package,
relative to the module root.

MODULE PATHS

A module path is the canonical name for a module, declared with the
module field in the module’s cue.mod/module.cue file. A module’s
path is the prefix for package paths within the module.

A module path consists of a root path and a major version suffix,
for example in the module path myhost.example/foo@v0, the root path is
myhost.example/foo and the major version suffix is @v0.

Module paths are domain-name qualified: a module path always begins
with a host name, although that host is only a guide to the origin of
the module and is not used directly to fetch the module’s contents (see
here [/docs/reference/modules/#cue-registry-env] for details about that)). The expectation is
that any modules you create should have names that are inside domains
or namespaces that you have control of, enabling modules from different
creators to live together without conflicts in the same registry.

 * The root path is the
   portion of the module path that identifies the OCI repository within
   a registry. All versions of a module are located in that same OCI
   repository.
 * The major version suffix declares the major
   version of the module and is of the form @v1 where the version
   v1 here must match the major version of the full version it’s been
   published as.

There are also several lexical restrictions on characters allowed in
module paths. As modules are stored in OCI repositories, these correspond
to the restrictions
documented there [https://github.com/opencontainers/distribution-spec/blob/main/spec.md#pulling-manifests].
To summarize:

 * The path must consist of one or more path elements separated by slashes
   (/, U+002F). It must not begin or end with a slash.
 * No characters are allowed in the path except lower case ASCII letters,
   ASCII digits, and limited ASCII punctuation (-, _, .).
 * The first character of each path element is a letter or a digit.
 * No more than one period (.) is allowed in sequence.
 * No more than two underscores (_) are allowed in sequence.

In addition, the first path element must contain at least one period character (.).

No restriction is directly enforced on the length of module names, but as registries
can refuse module paths over 128 characters, and it’s possible to specify
an arbitrary storage prefix, long module paths may fail.

VERSIONS

A version identifies an immutable snapshot of a module, which may be
either a release or a
pre-release (with a pre-release suffix). Each version starts with the letter
v, followed by a semantic version. See
Semantic Versioning 2.0.0 [https://semver.org/spec/v2.0.0.html] for details on how versions are
formatted, interpreted, and compared.

To summarize, a semantic version consists of three non-negative integers (the
major, minor, and patch versions, from left to right) separated by dots. The
patch version may be followed by an optional pre-release string starting with a
hyphen.

Each part of a version indicates whether the version is stable and whether it is
compatible with previous versions.

 * The major version must be incremented and the minor
   and patch versions must be set to zero after a backwards incompatible change
   is made to the module’s public interface or documented functionality, for
   example, after a package is removed.
 * The minor version must be incremented and the patch
   version set to zero after a backwards compatible change, for example, after a
   new function is added.
 * The patch version must be incremented after a change
   that does not affect the module’s public interface, such as a bug fix or
   change to the documentation.
 * The pre-release suffix indicates a version is a pre-release. Pre-release versions sort before
   the corresponding release versions. For example, v1.2.3-pre comes before
   v1.2.3.

A version is considered unstable if its major version is 0 or it has a
pre-release suffix. Unstable versions are not subject to compatibility
requirements. For example, v0.2.0 may not be compatible with v0.1.0, and
v1.5.0-beta may not be compatible with v1.5.0.

MAJOR VERSION SUFFIXES

Module paths must have a major version
suffix like @v2 that matches the major version. For example, if a module
has the path foo.example/mod@v1 at v1.0.0, it must have the path
foo.example/mod@v2 at version v2.0.0.

Major version suffixes implement the import compatibility
rule [https://research.swtch.com/vgo-import]:

> If an old package and a new package have the same import path,
> the new package must be backwards compatible with the old package.

By definition, packages in a new major version of a module are not backwards
compatible with the corresponding packages in the previous major version.
Consequently each new major version of a package needs a new import path.
This is accomplished by adding a major version suffix to the module path.
The import path for a package also includes the major version suffix,
providing a distinct import path for each incompatible version.

Unlike in Go [https://go.dev/ref/mod#major-version-suffixes],
major version suffixes are always required in module paths . The burden
of changing import paths in packages is eased by allowing the
major version suffix to be omitted and inferred from the module.cue
file. See major version defaults [/docs/reference/modules/#major-version-defaults] for details.

Major version suffixes let multiple major versions of a module coexist in the
same build. This may be necessary due to a diamond dependency
problem [https://research.swtch.com/vgo-import#dependency_story]. Ordinarily, if
a module is required at two different versions by transitive dependencies, the
higher version will be used. However, if the two versions are incompatible,
neither version will satisfy all clients. Since incompatible versions must have
different major version numbers, they must also have different module paths due
to major version suffixes. This resolves the conflict: modules with distinct
suffixes are treated as separate modules, and their packages—even packages in
same subdirectory relative to their module roots—are distinct.

MAJOR VERSION DEFAULTS

When a package import path does not contain a major version,
the module.cue file is consulted to determine which major
version of the module to use. In a canonical module.cue file,
all imports without major versions will have an explicit default: true
present in the corresponding dependency entry, but cue mod tidy
will add those if not present and there is no ambiguity in the build list.

That is, given only a single major version of a module in the build list,
the major version need not be specified in any of the package imports.

RESOLVING A PACKAGE TO A MODULE

When CUE loads a package using a package
path [/docs/reference/modules/#glos-package-path], it needs to determine which module provides the
package.

It starts by searching the build list [/docs/reference/modules/#glos-build-list] for
modules with paths that are prefixes of the package path. For example, if the
package foo.example/a/b is imported, and the module foo.example/a is in the
build list, CUE will check whether foo.example/a contains the
package, in the directory b. At least one file with the .cue extension must
be present in a directory for it to be considered a package. Build
constraints [/docs/reference/modules/#glos-build-constraint] are not applied for this
purpose. If exactly one module in the build list provides the package, that
module is used. If no modules provide the package or if two or more modules
provide the package, CUE reports an error. The cue mod tidy command
will attempt to find new modules providing missing
packages and to update cue.mod/module.cue accordingly.

THE CUE_REGISTRY ENVIRONMENT VARIABLE

When CUE looks up a new module for a package path, it checks the
CUE_REGISTRY environment variable. This determines the registry
and repository within a registry that a module will be searched for.
It holds a complete list of any registries that are consulted for fetching modules.

Specifically it holds a comma-separated list specifying which registry to use for
downloading and publishing modules. A registry is specifed
as follows:


Copy code
Copied!

[modulePrefix=]hostname[:port][/repoPrefix][+insecure]

The optional modulePrefix specifes that all modules with a path that
has the given prefix will use the associated registry. If there are
multiple registries with a prefix, the longest matching prefix wins.
It’s an error for there to be multiple entries with the same prefix.

The hostname holds the OCI registry host (in square brackets if it’s
an IPv6 address), with an optional numeric TCP port.

Each module is stored inside its own repository in the registry which
is named after the module path. The repoPrefix holds a prefix to be
added to the repository name. That is, all repositories in the registry
will be of the form repoPrefix/modulePath.

If there’s a +insecure suffix it specifies that an insecure HTTP
connection should be used to this registry. The default is to use a
secure HTTPS connection except for localhost addresses. For symmetry,
it’s also possible to use +secure to force an HTTPS connection even
on localhost connections.

For example, given:


Copy code
Copied!

CUE_REGISTRY=public-registry.example,github.com/acmecorp=registry.acme.example:6000/modules

all modules, such as github.com/foo/bar will be fetched from
public-registry.example with the exception of modules with the
prefix github.com/acmecorp/, such as github.com/acmecorp/somemodule
which will be fetched from the modules/github.com/acmecorp/somemodule repository
in the host registry.acme.example at port 6000.

CUE.MOD/MODULE.CUE FILES

A module is defined by a cue.mod directory in its root containing
a module.cue CUE file.


Copy code
Copied!

// module indicates the module's path.
module!: #Module

// version indicates the language version used by the code in this module
// - the minimum version of CUE required to evaluate the code in this
// module. When a later version of CUE is evaluating code in this module,
// this will be used to choose version-specific behavior. If an earlier
// version of CUE is used, an error will be given.
language?: version?: #Semver

// source holds information about the source of the files within the
// module. This field is mandatory at publish time.
source?: #Source

// description describes the purpose of this module.
description?: string

// deps holds dependency information for modules, keyed by module path.
deps?: [#Module]: #Dep

// custom holds arbitrary data intended for use by third-party tools.
// Each field at the top level represents a tooling namespace,
// conventionally a module or domain name. Data migrated from legacy
// module.cue files is placed in the "legacy" namespace.
custom?: [#Module | "legacy"]: [_]: _

#Dep: {
	// v indicates the minimum required version of the module.
	v!: #Semver

	// default indicates this module is used as a default in case more
	// than one major version is specified for the same module path.
	// Imports must specify the exact major version for a module path if
	// there is more than one major version for that path and default is
	// not set for exactly one of them.
	default?: bool
}

// #Module constrains a module path. The major version indicator is
// optional, but should always be present in a normalized module.cue
// file.
#Module: =~#"^[^@]+(@v(0|[1-9]\d*))$"#

// #Semver constrains a semantic version. This regular expression is taken
// from https://semver.org/spec/v2.0.0.html
#Semver: =~#"^v(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"#

// #Source describes a source of truth for a module's content.
#Source: {
	// kind specifies the kind of source.
	//
	// The special value "self" signifies a module is stand-alone, associated
	// with no particular source. The module's file list is determined from
	// the contents of the directory (and its subdirectories) that contains
	// the cue.mod directory.
	//
	// See https://cuelang.org/docs/reference/modules/#determining-zip-file-contents
	// for details on all the possible values for kind, and how they relate
	// to determining the list of files in a module.
	kind!: "self" | "git"
}

For example:


Copy code
Copied!

language: version: "v0.4.3"

module: "foo.example/my/thing@v1"

deps: {
	"foo.example/other/thing@v1": v: "v1.0.2"
	"foo.example/new/thing@v2": v:   "v2.3.4"
}

The module.cue file is designed to be human readable and machine writable. The
cue command will provide several subcommands that manipulate cue.mod/module.cue files.
For now, the only one is cue mod tidy which will fetch dependencies
and canonicalize the module.cue file to reflect all the most recent versions.

A cue.mod/module.cue file is required for all modules.

MINIMAL VERSION SELECTION (MVS)

CUE uses an algorithm called Minimal version selection (MVS) to select
a set of module versions to use when building packages. MVS is described in
detail in Minimal Version Selection [https://research.swtch.com/vgo-mvs] by
Russ Cox.

Conceptually, MVS operates on a directed graph of modules, specified with
module.cue files [/docs/reference/modules/#glos-cue-mod-file]. Each vertex in the graph represents a
module version. Each edge represents a minimum required version of a dependency,
specified with an entry in the deps field.

MVS produces the build list [/docs/reference/modules/#glos-build-list] as output, the list of module
versions used for an evaluation.

MVS starts at the main modules (special vertices in the graph that have no
version) and traverses the graph, tracking the highest required version of each
module. At the end of the traversal, the highest required versions comprise the
build list: they are the minimum versions that satisfy all requirements.

Unlike other dependency management systems, the build list is
not saved in a “lock” file. MVS is deterministic, and the build list doesn’t
change when new versions of dependencies are released, so MVS is used to compute
it at the beginning of every module-aware command.

Module version graph with visited versions highlighted [buildlist.svg]
buildlist.svgModule version graph with visited versions highlightedConsider the example in this diagram. The main module requires module A
at version 1.2 or higher and module B at version 1.2 or higher. A 1.2 and B 1.2
require C 1.3 and C 1.4, respectively. C 1.3 and C 1.4 both require D 1.2.

MVS visits and loads the cue.mod/module.cue file for each of the module versions
highlighted in blue. At the end of the graph traversal, MVS returns a build list
containing the bolded versions: A 1.2, B 1.2, C 1.4, and D 1.2. Note that higher
versions of B and D are available but MVS does not select them, since nothing
requires them.

MODULE STORAGE FORMAT

Modules are stored in a registry using a standard manifest + blob
format. There is rarely any need to
interact directly with these artifacts, since the cue command creates, downloads,
and extracts them automatically from registries. However, it’s still useful to know about these
files to understand cross-platform compatibility constraints.

A module is stored in a registry with a top level manifest with media type
application/vnd.oci.image.manifest.v1+json and artifact type
application/vnd.cue.module.v1+json, that points to two blobs.
The first blob (also known as a “layer 0” although there’s actually
no layering going on here) has media type application/zip and holds the full contents
of the module. The second blob, layer 1, has media type application/vnd.cue.modulefile.v1
and stores an exact copy of the contents of the cue.mod/module.cue file
from the zip file. The latter enables fast access to the dependency information
without the need to download the entire module archive.

DETERMINING ZIP FILE CONTENTS

The source field in module.cue is used by cue mod publish to determine
which files to include in a module zip. It is required when publishing a module.
The source.kind field specifies the kind of source. The supported kinds are
listed below.

source: kind: "self" determines the module file list from the module root
directory tree on disk.

source: kind: "git" requires that the module root directory be under the
control of a Git VCS [https://git-scm.com/] repository. The git ls-files
command is then used to determine the module file list within the module root
directory. When publishing a module that is not in the repository root
directory, if the module does not have a file named LICENSE in its root
directory, cue mod publish will include the file named LICENSE from the
repository root directory at the module root. Every entry in the module file
list must be “clean” with respect to the current commit.

The initial list of files determined by the source is then filtered according
to file path and size constraints [/docs/reference/modules/#zip-path-size-constraints].

FILE PATH AND SIZE CONSTRAINTS

There are a number of restrictions on the content of module zip files. These
constraints ensure that zip files can be extracted safely and consistently on
a wide range of platforms.

 * A module zip file may be at most 500 MiB in size. The total uncompressed size
   of its files is also limited to 500 MiB. module.cue files are limited to 16 MiB.
   LICENSE files are also limited to 16 MiB. These limits exist to mitigate
   denial of service attacks on users, proxies, and other parts of the module
   ecosystem. Repositories that contain more than 500 MiB of files in a module
   directory tree should tag module versions at commits that only include files
   needed to build the module’s packages; videos, models, and other large assets
   are usually not needed for builds.
 * File modes, timestamps, and other metadata are ignored.
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

selected version: The version of a given module chosen by minimal version
selection [/docs/reference/modules/#minimal-version-selection]. The selected version is the highest
version for the module’s path found in the module graph [/docs/reference/modules/#glos-module-graph].

version: An identifier for an immutable snapshot of a module, written as the
letter v followed by a semantic version. See the section on
Versions [/docs/reference/modules/#versions].

RELATED CONTENT

 * Tutorial: Working with a custom module registry [/docs/tutorial/working-with-a-custom-module-registry/]

Last modified May 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/2bb11f637a7a1cfe18b3bed5c0717cf07f5ea21d]

 * modules [/search?q=tag:modules]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/reference/modules/&text=%20Introduction%20Modules%20are%20how%20CUE%20manages%20dependencies.%20This%20document%20is%20a%20detailed%20reference%20manual%20for%20CUE&rsquo;s%20module%20system.%20CUE&rsquo;s%20modules%20support%20has%20a%20lot%20in%20common%20with%20Go&rsquo;s%20modules%20and%20this%20document%20has%20substantial%20parts%20that%20have%20been%20taken%20directly%20from%20the%20Go%20modules%20reference.%20Thanks%20very%20much%20to%20Russ%20Cox%20and%20the%20Go%20team%20for%20their%20amazing%20work%20there.%0aThis%20document%20largely%20supercedes%20the%20prior%20modules%20documentation%20although,%20as%20a%20transitionary%20measure,%20the%20CUE%20tool%20still%20supports%20the%20import%20of%20packages%20present%20in%20the%20cue.mod/pkg,%20cue.mod/usr%20and%20cue.mod/gen%20directories.%20This%20only%20applies%20to%20the%20main%20module,%20and%20if%20there%20is%20any%20ambiguity%20with%20respect%20to%20regular%20module%20dependencies%20an%20&ldquo;ambiguous%20import&rdquo;%20error%20will%20be%20reported.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/reference/modules/&summary=%20Introduction%20Modules%20are%20how%20CUE%20manages%20dependencies.%20This%20document%20is%20a%20detailed%20reference%20manual%20for%20CUE&rsquo;s%20module%20system.%20CUE&rsquo;s%20modules%20support%20has%20a%20lot%20in%20common%20with%20Go&rsquo;s%20modules%20and%20this%20document%20has%20substantial%20parts%20that%20have%20been%20taken%20directly%20from%20the%20Go%20modules%20reference.%20Thanks%20very%20much%20to%20Russ%20Cox%20and%20the%20Go%20team%20for%20their%20amazing%20work%20there.%0aThis%20document%20largely%20supercedes%20the%20prior%20modules%20documentation%20although,%20as%20a%20transitionary%20measure,%20the%20CUE%20tool%20still%20supports%20the%20import%20of%20packages%20present%20in%20the%20cue.mod/pkg,%20cue.mod/usr%20and%20cue.mod/gen%20directories.%20This%20only%20applies%20to%20the%20main%20module,%20and%20if%20there%20is%20any%20ambiguity%20with%20respect%20to%20regular%20module%20dependencies%20an%20&ldquo;ambiguous%20import&rdquo;%20error%20will%20be%20reported.%0a]


Code of Conduct
[/docs/reference/code-of-conduct/]Next

 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
 * References [/docs/reference/]
   * The CUE Language Specification [/docs/reference/spec/]
   * Glossary of terms [/docs/reference/glossary/]
   * The cue command [/docs/reference/command/]
   * Code of Conduct [/docs/reference/code-of-conduct/]
   * CUE Modules [/docs/reference/modules/]
      1.  Introduction
      2.  Modules, packages, and versions
      3.  cue.mod/module.cue files
      4.  Minimal version selection (MVS)
      5.  Module storage format
      6.  Determining zip file contents
      7.  File path and size constraints
      8.  Module caching
      9.  Authorization
      10. Glossary
      11. Related content

Hide side navigation


Show side navigation

Get Started

 * Documentation [/docs/]
 * Language Tour [/docs/tour/]
 * Playground [/play/]
 * Install CUE [/docs/introduction/installation/]

Community

 * The CUE Community [/community]
 * Contributing [https://github.com/cue-lang/cue/blob/master/CONTRIBUTING.md#contribution-guide]
 * Code of Conduct [/docs/reference/code-of-conduct/]
 * Slack Workspace [/s/slack]
 * Discord Server [/s/discord]

Connect

 * GitHub [https://github.com/cue-lang/cue]
 * X (Twitter) [https://twitter.com/cue_lang]
 * Bluesky [https://bsky.app/profile/cuelang.org]
 * YouTube [https://www.youtube.com/@cuelang/videos]

 * © 2025 CUE
 * Privacy policy [/privacy-policy/]
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Freference%2Fmodules%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


Homepage of CUE [/]
CUE v0.15 is now available – learn more about its new features and improvements [https://github.com/cue-lang/cue/releases/tag/v0.15.0]
Install CUE

[/docs/introduction/installation/]

Close

Homepage of CUE [/]


Hide menu
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]
 * Install [/docs/introduction/installation/]
 * 

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]
