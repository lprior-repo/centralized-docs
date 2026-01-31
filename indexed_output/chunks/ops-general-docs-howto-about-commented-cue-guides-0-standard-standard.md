---
doc_id: ops/general/docs-howto-about-commented-cue-guides
chunk_id: ops/general/docs-howto-about-commented-cue-guides#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 523
summary: # Commented CUE Guides | CUE. **Source:** https://cuelang
---

# Commented CUE Guides | CUE

**Source:** https://cuelang.org/docs/howto/about-commented-cue-guides/

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

 1. How-to Guides [https://cuelang.org/docs/howto/]


 2. COMMENTED CUE GUIDES

Some how-to guides use CUE’s useful style of
“literate [https://en.wikipedia.org/wiki/Literate_programming] configuration”.
We refer to them as commented cue
[/search?q=tag:%22commented%20cue%22]guides.

Commented CUE guides tend to be short, presenting commented CUE code alongside
some input data, with a cue command showing the concrete result of the
technique they’re demonstrating. Often that result is an error message, showing
that cue has caught the deliberate mistakes in our example data.

Here’s an example:

Copied!
example.cue

Copy code
Copied!

package example

// CUE comments start with "//"
// and run to the end of the line

// f1 is a regular field which must be a string
f1: string

// f2 is a required field which must be an
// integer over 10
f2!: int & >10

Copied!
data.yml

Copy code
Copied!

# f1 is actually an integer
f1: 123

# f2 is actually a string
f2: "some string value"

TERMINAL

Copy code
Copied!

$ cue vet -c .:example data.yml
f1: conflicting values 123 and string (mismatched types int and string):
    ./data.yml:2:5
    ./example.cue:7:5
f2: conflicting values "some string value" and int (mismatched types string and int):
    ./data.yml:5:5
    ./example.cue:11:6

Commented CUE guides require you, the reader, to modify their examples
before you use them. Don’t use the CUE exactly as it’s presented, but first
adapt the variable names and data structures to suit your configuration.
