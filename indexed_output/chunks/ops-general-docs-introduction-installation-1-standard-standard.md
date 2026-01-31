---
doc_id: ops/general/docs-introduction-installation
chunk_id: ops/general/docs-introduction-installation#1-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 513
summary: They can be used on. platforms supported by Docker [https://docs
---

They can be used on
platforms supported by Docker [https://docs.docker.com/engine/install/].

Various tags are available that let you select container images for different
machine architectures and CUE versions. To use the latest version that’s
appropriate for your machine, use the latest tag:

TERMINAL

Copy code
Copied!

$ docker pull cuelang/cue:latest


INSTALL FROM SOURCE

On
platforms supported by Go [https://go.dev/dl/#stable],
cue can be installed from source using any of its
release, pre-release, or as-yet-unreleased versions.
Installing from source requires that you already have
Go [https://go.dev]
installed and available.

For example, to fetch the latest version:

TERMINAL

Copy code
Copied!

$ go install cuelang.org/go/cmd/cue@latest
...

This page [https://pkg.go.dev/cuelang.org/go?tab=versions]
lists the installable releases and pre-releases that you can specify instead of
latest.


DEVELOPMENT VERSION

You can install the development version of cue from source by specifying master:

TERMINAL

Copy code
Copied!

$ go install cuelang.org/go/cmd/cue@master
...

The capabilities of the development version change frequently because it contains the
most recent, unreleased code.


INSTALL ON ARCH LINUX

On Arch Linux
(archlinux.org [https://archlinux.org]),
cue can be installed from a package in the official Arch extra repository:

TERMINAL

Copy code
Copied!

$ pacman -S extra/cue

This package is not published by the CUE project.

USING THE GO API

CUE’s Go APIs are defined in the
cuelang.org/go [https://pkg.go.dev/cuelang.org/go] module, which can be added
as a versioned dependency using Go’s
dependency management [https://go.dev/doc/modules/managing-dependencies]
workflows. All available versions are listed at
pkg.go.dev/cuelang.org/go [https://pkg.go.dev/cuelang.org/go?tab=versions].

For example, to add a dependency on the latest version:

TERMINAL

Copy code
Copied!

$ go get cuelang.org/go@latest
...

RELATED CONTENT

 * Concept Guide: How CUE works with Go [/docs/concept/how-cue-works-with-go/]
