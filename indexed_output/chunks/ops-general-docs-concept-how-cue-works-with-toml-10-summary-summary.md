---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#10-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: source = 'bar'. version = '1
---

   
   Copied!
   env/foo.toml
 * 
   
   Copied!
   env/bar.toml

Copy code
Copied!

source = 'bar'
version = '1.42.0'


Copy code
Copied!

text = 'Some foo'


Copy code
Copied!

text = 'A bar'

TERMINAL

Copy code
Copied!

$ cue export --out toml
[info]
source = 'A bar'
version = '1.42.0'

File embedding is available from CUE v0.12.0 onwards.
Find out more about this powerful validation feature in
Embedding files in a CUE evaluation [/docs/howto/embed-files-in-cue-evaluation/].

ENCODING TOML INSIDE CUE

