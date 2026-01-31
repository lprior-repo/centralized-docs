---
doc_id: ops/general/docs-introduction-installation
chunk_id: ops/general/docs-introduction-installation#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1036
summary: # Installation | CUE. **Source:** https://cuelang
---

# Installation | CUE

**Source:** https://cuelang.org/docs/introduction/installation/

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

 1. Introduction [https://cuelang.org/docs/introduction/]


 2. INSTALLATION

TRY OUT CUE IN YOUR BROWSER!

You can use the latest version of CUE without installing anything – just visit
cuelang.org/play [https://cuelang.org/play/]!

INSTALLING THE CUE COMMAND

The cue command is available from various sources, listed on this page.
If you don’t know which source to choose, download and unpack the
latest version [https://github.com/cue-lang/cue/releases/latest]
that’s appropriate for your operating system – it contains the most recent
features and bug fixes.
The latest version of the cue command is:

TERMINAL

Copy code
Copied!

$ cue version
cue version v0.15.3
...


DOWNLOAD AN OFFICIAL CUE BINARY

On Linux, Microsoft Windows, and macOS, the cue command can be downloaded from the
official CUE releases [https://github.com/cue-lang/cue/releases/].

These releases include pre-releases, which are cutting-edge versions of the
cue command made available to help expose bugs and flush out unintended
behaviours. You should choose to install the
latest release [https://github.com/cue-lang/cue/releases/latest] if you don’t
have a specific reason to select a pre-release.


INSTALL USING HOMEBREW

On macOS and Linux, cue can be installed using Homebrew
(brew.sh [https://brew.sh]):

TERMINAL

Copy code
Copied!

$ brew install cue-lang/tap/cue


INSTALL FROM DOCKER HUB

The CUE project publishes official container images containing cue on Docker Hub
(hub.docker.com/r/cuelang/cue [https://hub.docker.com/r/cuelang/cue]).
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
