---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 518
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
