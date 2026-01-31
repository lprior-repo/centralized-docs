---
doc_id: ops/general/docs-reference-modules
chunk_id: ops/general/docs-reference-modules#5-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 512
summary: packages and to update cue. cue accordingly
---

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
