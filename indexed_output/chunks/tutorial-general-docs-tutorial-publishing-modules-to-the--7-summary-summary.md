---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#7-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary: git, when publishing this module. The GitHub user cueckoo controls all the repositories under
---

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
