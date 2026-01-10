---
id: tutorial/general/docs-tutorial-publishing-modules-to-the-
title: Docs Tutorial Publishing Modules To The Central Registry
category: tutorial
tags: [""cueckoo"", "*your*", "central", "github", "lower-cased."]
---

# Publishing modules to the Central Registry | CUE

> **Context**: **Source:** https://cuelang.org/docs/tutorial/publishing-modules-to-the-central-registry/


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

// #Config defines the schema for the FrostyApp configuration.
#Config: {
	// appName defines the name of the application.
	appName!: string
	// port holds the port number the application listens on.
	port!: int
	// debug holds whether to enable debug mode.
	debug?: bool
	// features holds optional feature settings
	features?: {
		// logging enables or disables logging.
		logging?: bool
		// analytics enables or disables analytics.
		analytics?: bool
	}
}

4

As a one-off, login to the Central Registry:

TERMINAL

Copy code
Copied!

$ cue login

The Central Registry is in beta testing -
please give us your feedback about the service in the
#modules channel on Slack [/s/slack] or on Discord [/s/discord]!

5

Ensure the module.cue file is tidy:

TERMINAL

Copy code
Copied!

$ cue mod tidy

6
If you haven’t already done so,
create a repository [https://github.com/new?org=]
called frostyconfig under your personal username at GitHub.
It doesn’t matter if the repository is public or private.
7

Create a git commit:

TERMINAL

Copy code
Copied!

$ git add -A
$ git commit -q -m 'Initial commit'

Earlier, you initialized this module with --source=git, which told the cue
command that it should publish only those files that git knows about. The git
commit you just created leaves the directory in a “clean” state, which is
necessary for cue to know exactly which files to include in the published
module.

8

Publish the first version of this module:

TERMINAL

Copy code
Copied!

$ cue mod publish v0.0.1
...

This command should mention your GitHub username,
and should publish the module successfully.

If the command fails with an error message that mentions your GitHub username
then you probably haven’t created the frostyconfig repository under your GitHub username.
Create it, and try the step again.

If the command fails with an error message that mentions cueckoo/frostyconfig
then you probably forgot to adapt the command in step 3, above.
Don’t worry - this isn’t a serious problem!

The easiest way to fix this is to delete your frostyconfig directory
and restart the tutorial from step 1.

CREATE A NEW FROSTYAPP MODULE THAT DEPENDS ON THE FIRST MODULE

Define the FrostyApp configuration, constrained by the schema you just
published.

9

Create a directory for the new module and initalize it,
changing cueckoo to your GitHub username, lower-cased:

TERMINAL

Copy code
Copied!

$ mkdir ../frostyapp
$ cd ../frostyapp
$ git init -q
$ cue mod init --source=git github.com/cueckoo/frostyapp@v0

10

Create the code for the new module:

Copied!
frostyapp/config.cue

Copy code
Copied!

 1
 2
 3
 4
 5
 6
 7
 8
 9
10


package frostyapp

// Adapt this line to your GitHub username, lower-cased.
import "github.com/cueckoo/frostyconfig@v0"

config: frostyconfig.#Config & {
	appName: "alpha"
	port:    80
	features: logging: true
}

Remember to change cueckoo to your GitHub username, lower-cased, on line 4.

11

Ensure the module is tidy, adding missing dependencies:

TERMINAL

Copy code
Copied!

$ cue mod tidy

We can see that the dependencies have now been added to the
cue.mod/module.cue file:

TERMINAL

Copy code
Copied!

$ cat cue.mod/module.cue
module: "github.com/cueckoo/frostyapp@v0"
language: {
	version: "v0.15.3"
}
source: {
	kind: "git"
}
deps: {
	"github.com/cueckoo/frostyconfig@v0": {
		v: "v0.0.1"
	}
}

EVALUATE THE CONFIGURATION

12

Export the configuration as YAML:

TERMINAL

Copy code
Copied!

$ cue export --out yaml
config:
  appName: alpha
  port: 80
  features:
    logging: true

We can use this new module code just like any other CUE code.

CONGRATULATIONS!

That’s it! You have just created a module and published it to the Central
Registry, and then used the newly published module to check a concrete
configuration held in a different module.

RELATED CONTENT

 * Tutorial: Working with modules and the Central Registry [/docs/tutorial/working-with-the-central-registry/]
 * Tutorial: Working with a custom module registry [/docs/tutorial/working-with-a-custom-module-registry/]
 * Reference: CUE Modules [/docs/reference/modules/]

Last modified December 30, 2025 [https://github.com/cue-lang/cuelang.org/commit/72e9d5a34edd0ed8ba86fe0990b9ef3a945a37fa]

 * cue command [/search?q=tag:%22cue%20command%22]
 * tooling [/search?q=tag:tooling]
 * modules [/search?q=tag:modules]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/tutorial/publishing-modules-to-the-central-registry/&text=%20Introduction%20In%20this%20tutorial%20you%20will%20publish%20a%20module%20to%20the%20Central%20Registry%20and%20then%20create%20a%20second%20module%20that%20depends%20on%20the%20first.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/tutorial/publishing-modules-to-the-central-registry/&summary=%20Introduction%20In%20this%20tutorial%20you%20will%20publish%20a%20module%20to%20the%20Central%20Registry%20and%20then%20create%20a%20second%20module%20that%20depends%20on%20the%20first.%0a]


Loading CUE via the Go API
[/docs/tutorial/loading-cue-go-api/]Using modules with the Go API
[/docs/tutorial/using-modules-with-go-api/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
   * New to CUE? [/docs/tutorial/new-to-cue/]
   * Publishing modules to the Central Registry [/docs/tutorial/publishing-modules-to-the-central-registry/]
      1. Introduction
      2. Prerequisites
      3. Create the module for the schema code
      4. Create a new frostyapp module that depends on the first module
      5. Evaluate the configuration
      6. Congratulations!
      7. Related content
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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Ftutorial%2Fpublishing-modules-to-the-central-registry%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


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
## See Also

- [Documentation Index](./COMPASS.md)
