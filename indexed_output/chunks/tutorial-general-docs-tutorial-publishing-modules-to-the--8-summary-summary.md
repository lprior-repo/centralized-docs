---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#8-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: 	// debug holds whether to enable debug mode. 	debug?: bool
---

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
