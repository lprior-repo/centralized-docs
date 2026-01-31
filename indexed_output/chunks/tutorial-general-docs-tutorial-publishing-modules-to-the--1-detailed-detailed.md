---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1027
summary: // #Config defines the schema for the FrostyApp configuration. 	// appName defines the name of the application
---


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
