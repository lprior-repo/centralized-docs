---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1024
summary: # Publishing modules to the Central Registry | CUE. **Source:** https://cuelang
---

# Publishing modules to the Central Registry | CUE

**Source:** https://cuelang.org/docs/tutorial/publishing-modules-to-the-central-registry/

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

 1. Tutorials [https://cuelang.org/docs/tutorial/]


 2. PUBLISHING MODULES TO THE CENTRAL REGISTRY

myitcv [https://github.com/myitcv.png]
Paul Jolly
myitcv [https://github.com/myitcv.png]
Paul Jolly

Github profile

[https://github.com/myitcv]

Search all content by this author

[/search/?q=author:myitcv]
 * cue command [/search?q=tag:%22cue%20command%22]
 * tooling [/search?q=tag:tooling]
 * modules [/search?q=tag:modules]

INTRODUCTION

In this tutorial you will publish a module to the Central Registry and then
create a second module that depends on the first.

PREREQUISITES

 * A GitHub [https://docs.github.com/en/get-started/start-your-journey/creating-an-account-on-github#signing-up-for-a-new-personal-account] account –
   this will let you authenticate to the Central Registry
 * A GitHub repository called frostyconfig –
   create it under your personal GitHub account (it doesn’t matter if it is public or private)
 * A Central Registry [https://registry.cue.works/] account
 * The cue binary –
   follow the installation instructions [/docs/introduction/installation/]
   if you don’t already use cue
 * A tool to edit text files –
   any text editor you have will be fine, such as
   VSCode [https://code.visualstudio.com/],
   Notepad [https://apps.microsoft.com/detail/9msmlrh6lzf3], or
   Vim [https://www.vim.org/download.php]
 * A command terminal –
   cue works on all platforms, so you can use any Linux or macOS terminal,
   or a Windows terminal such as PowerShell, cmd, or
   WSL [https://learn.microsoft.com/en-us/windows/wsl/install]
   to run commands.
 * Some awareness of CUE schemata –
   the language tour’s pages on
   Constraints [/docs/tour/basics/constraints/] and
   Definitions [/docs/tour/basics/definitions/] are a good refresher

This tutorial is written using the following version of cue:

TERMINAL

Copy code
Copied!

$ cue version
cue version v0.15.3
...

CREATE THE MODULE FOR THE SCHEMA CODE

In this tutorial we will focus on an imaginary application called FrostyApp,
which consumes its configuration in YAML format.
You will define the configuration in CUE and use a CUE schema to validate it.
We would like to be able to share the schema between several consumers,
so we will publish it to the Central Registry.

1

Create a directory to hold the schema code:

TERMINAL

Copy code
Copied!

$ mkdir frostyconfig
$ cd frostyconfig

You need to adapt the command shown in the next step.

Don’t simply paste the command into your terminal and run it.

Before running the command, replace the example username,
cueckoo,
with the lower-cased form of YOUR GitHub username.
For example:
if your GitHub username is _TomHanks
then you would replace cueckoo with _tomhanks.

You need to make this replacement everywhere you see
the username cueckoo in this tutorial.

2

Initialize the directory as a git repository and a CUE module:

TERMINAL

Copy code
Copied!

$ git init -q

# Replace "cueckoo" with *your* GitHub username, lower-cased.
$ cue mod init --source=git github.com/cueckoo/frostyconfig@v0

The --source=git flag tells cue to use the same file-inclusion rules as
git, when publishing this module.

The GitHub user cueckoo controls all the repositories under
github.com/cueckoo/, so they can publish modules to the Central
Registry inside that namespace. The same is true for your GitHub username.

3

Create the configuration schema:

Copied!
frostyconfig/config.cue

Copy code
Copied!

package frostyconfig
