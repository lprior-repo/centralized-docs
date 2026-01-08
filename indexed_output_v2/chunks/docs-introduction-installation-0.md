---
doc_id: docs-introduction-installation
chunk_id: docs-introduction-installation#0
chunk_type: prose
heading: Introduction
token_count: 2088
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

Last modified December 30, 2025 [https://github.com/cue-lang/cuelang.org/commit/72e9d5a34edd0ed8ba86fe0990b9ef3a945a37fa]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/introduction/installation/&text=%20Try%20out%20CUE%20in%20your%20browser!%20You%20can%20use%20the%20latest%20version%20of%20CUE%20without%20installing%20anything%20&ndash;%20just%20visit%20cuelang.org/play!%0aInstalling%20the%20cue%20command%20The%20cue%20command%20is%20available%20from%20various%20sources,%20listed%20on%20this%20page.%20If%20you%20don&rsquo;t%20know%20which%20source%20to%20choose,%20download%20and%20unpack%20the%20latest%20version%20that&rsquo;s%20appropriate%20for%20your%20operating%20system%20&ndash;%20it%20contains%20the%20most%20recent%20features%20and%20bug%20fixes.%20The%20latest%20version%20of%20the%20cue%20command%20is:%0aTERMINAL%20Copy%20code%20Copied!%20$%20cue%20version%20cue%20version%20v0.15.3%20...%20]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/introduction/installation/&summary=%20Try%20out%20CUE%20in%20your%20browser!%20You%20can%20use%20the%20latest%20version%20of%20CUE%20without%20installing%20anything%20&ndash;%20just%20visit%20cuelang.org/play!%0aInstalling%20the%20cue%20command%20The%20cue%20command%20is%20available%20from%20various%20sources,%20listed%20on%20this%20page.%20If%20you%20don&rsquo;t%20know%20which%20source%20to%20choose,%20download%20and%20unpack%20the%20latest%20version%20that&rsquo;s%20appropriate%20for%20your%20operating%20system%20&ndash;%20it%20contains%20the%20most%20recent%20features%20and%20bug%20fixes.%20The%20latest%20version%20of%20the%20cue%20command%20is:%0aTERMINAL%20Copy%20code%20Copied!%20$%20cue%20version%20cue%20version%20v0.15.3%20...%20]


Previous
What is CUE?
[/docs/introduction/what-is-cue/]
 * Introduction [/docs/introduction/]
   * Installation [/docs/introduction/installation/]
      1. Installing the cue command
      2. Using the Go API
      3. Related content
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
 * References [/docs/reference/]

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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Fintroduction%2Finstallation%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


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
