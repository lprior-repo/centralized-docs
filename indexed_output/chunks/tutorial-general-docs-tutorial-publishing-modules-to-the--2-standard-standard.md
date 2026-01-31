---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#2-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 516
summary: 	// appName defines the name of the application. 	appName!: string
---

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
