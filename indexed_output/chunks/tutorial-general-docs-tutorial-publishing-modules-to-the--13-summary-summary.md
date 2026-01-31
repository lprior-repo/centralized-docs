---
doc_id: tutorial/general/docs-tutorial-publishing-modules-to-the-
chunk_id: tutorial/general/docs-tutorial-publishing-modules-to-the-#13-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: Remember to change cueckoo to your GitHub username, lower-cased, on line 4. Ensure the module is tidy, adding missing dependencies:
---

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
