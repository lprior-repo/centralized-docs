---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#3-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 525
summary: CREATE A NEW FROSTYAPP MODULE THAT DEPENDS ON THE FIRST MODULE. Define the FrostyApp configuration, constrained by the schema you just
---


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
