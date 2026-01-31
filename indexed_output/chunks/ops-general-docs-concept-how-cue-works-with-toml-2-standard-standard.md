---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#2-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 515
summary: $ cue export data. toml transform
---

Copied!

$ cue export data.toml transform.cue --out toml
a = 5
b = 4
c = 21

Learn more about transforming data with CUE in these How-to guides:

 * Transforming JSON with CUE [/docs/howto/transform-json-with-cue/]
 * Combining multiple JSON files into a list [/docs/howto/combine-multiple-json-files-into-a-list/]
 * Combining multiple JSON files by using file metadata [/docs/howto/combine-multiple-json-files-by-using-file-metadata/]

These guides explain things in terms of JSON data, but the techniques they
demonstrate apply equally to TOML because CUE treats all data encodings as
equivalent.

EMBEDDING TOML FILE DATA INSIDE CUE

Requires CUE v0.12.0 or later

The file embedding [/docs/howto/embed-files-in-cue-evaluation/]
feature allows data files (including TOML) to be read when some CUE is evaluated.
This provides an alternative way to use CUE to validate data files against
schemas and constraints, and also gives CUE configurations access to data
stored in non-CUE files:

Copied!
example.cue

Copy code
Copied!

@extern(embed)

package p

_conf: _ @embed(file=config.toml)
_data: _ @embed(glob=env/*.toml)

info: {
	version: _conf.version
	source:  _data["env/\(_conf.source).toml"].text
}

 * 
   
   Copied!
   config.toml
 * 
   
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

CUE is frequently used to generate configuration files. Some systems allow
their configuration files to contain TOML encoded in string fields,
irrespective of the file’s main data format.

CUE’s standard library provides
a built-in toml package [https://pkg.go.dev/cuelang.org/go/pkg/encoding/toml]
