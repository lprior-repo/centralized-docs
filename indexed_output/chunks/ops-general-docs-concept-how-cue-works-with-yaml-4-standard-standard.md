---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#4-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 618
summary: $ cue vet -c furniture. painting: invalid value \"width: 34\nHEIGHT: 12\ndepth: 0
---

	width: 34
	HEIGHT: 12
	depth: 0.2
	"""

TERMINAL

Copy code
Copied!

$ cue vet -c furniture.cue
item.painting: invalid value "width: 34\nHEIGHT: 12\ndepth: 0.2" (does not satisfy encoding/yaml.Validate): error in call to encoding/yaml.Validate: field not allowed:
    ./furniture.cue:10:17
    ./furniture.cue:25:17
    yaml.Validate:2:1
item.table: invalid value "width: \"34\"\nheight: 23\ndepth: 0.2" (does not satisfy encoding/yaml.Validate): error in call to encoding/yaml.Validate: conflicting values "34" and number (mismatched types string and number):
    ./furniture.cue:10:17
    ./furniture.cue:4:10
    ./furniture.cue:19:14
    yaml.Validate:1:8

OTHER YAML FUNCTIONS

The
yaml package [https://pkg.go.dev/cuelang.org/go/pkg/encoding/yaml]
contains other useful functions which are demonstrated in guides that you can
discover through the site’s search page:
🔍 
search for how-to guides mentioning “encoding/yaml” [/search/?q=encoding/yaml%20contentType:%22How-to%20Guides%22]

CONVERTING YAML FILES TO CUE

cue import [/docs/reference/command/cue-help-import/]
can create a CUE file for each YAML file it’s given, and can even recognise
encoded YAML and JSON fields, and convert those structures recursively.

Examples of this command being used can be found in the
cue import CLI reference documentation [/docs/reference/command/cue-help-import/].

RELATED CONTENT

 * Concept Guide: How CUE works with JSON [/docs/concept/how-cue-works-with-json/]
 * Concept Guide: How CUE works with TOML [/docs/concept/how-cue-works-with-toml/]

Last modified November 6, 2025 [https://github.com/cue-lang/cuelang.org/commit/c4d2e727a59f5158a2be4946992d96c4612a9b88]

 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/how-cue-works-with-yaml/&text=%20Reading%20and%20writing%20YAML%20The%20cue%20tool%20natively%20supports%20reading%20and%20writing%20YAML%20files,%20including%20those%20containing%20multiple%20documents.%0aThis%20allows%20YAML%20files%20to%20be%20processed%20by%20CUE&rsquo;s%20wide%20range%20of%20data,%20schema,%20and%20policy%20validation%20capabilities,%20and%20to%20convert%20input%20formats%20to%20YAML%20-%20as%20demonstrated%20here%20by%20cue%20export%20unifying%20all%20its%20YAML,%20JSON,%20and%20CUE%20input%20files%20as%20YAML:%0a]
