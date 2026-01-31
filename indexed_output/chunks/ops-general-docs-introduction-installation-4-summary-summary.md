---
doc_id: ops/general/docs-introduction-installation
chunk_id: ops/general/docs-introduction-installation#4-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: Various tags are available that let you select container images for different. machine architectures and CUE versions
---


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
