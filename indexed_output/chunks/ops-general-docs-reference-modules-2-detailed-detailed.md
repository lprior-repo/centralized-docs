---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1026
summary: a module is required at two different versions by transitive dependencies, the. higher version will be used
---

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
