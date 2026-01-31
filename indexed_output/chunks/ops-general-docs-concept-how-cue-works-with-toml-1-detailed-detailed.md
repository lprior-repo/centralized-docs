---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1024
summary: $ cue export data. toml transform
---

Copied!

a = 5
b = 4

TERMINAL

Copy code
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
containing functions that generate and parse TOML from within CUE, as shown here.

GENERATING ENCODED TOML

In this example a Kubernetes ConfigMap contains a TOML file encoded as a
single string field, in a YAML document. This is enabled by the
toml.Marshal function:

Copied!
config.cue

Copy code
Copied!

import "encoding/toml"

configMap: data: "point.toml": toml.Marshal({
	x: 1.2
	y: 3.45
})

TERMINAL

Copy code
Copied!

$ cue export config.cue --out yaml
configMap:
  data:
    point.toml: |
      x = 1.2
      y = 3.45

PARSING ENCODED TOML

The toml.Unmarshal function performs the reverse operation to toml.Marshal:
it turns a string containing TOML into the structure represented by
the encoded data.

Here, some encoded TOML data (a Rust crate manifest) is emitted as YAML:

Copied!
config.cue

Copy code
Copied!

import "encoding/toml"

_cargo: """
	[package]
	name = "hello_world"
	version = "0.1.0"
	edition = "2021"

	[dependencies]
	serde = "1.0"
	"""
output: cargo: toml.Unmarshal(_cargo)

TERMINAL

Copy code
Copied!

$ cue export config.cue --out yaml
output:
  cargo:
    package:
      name: hello_world
      version: 0.1.0
      edition: "2021"
    dependencies:
      serde: "1.0"

CONVERTING TOML FILES TO CUE

The cue import [/docs/reference/command/cue-help-import/]
command can create a CUE file for each TOML file it’s given – and can even
recognise encoded YAML and JSON fields, and convert those structures
recursively.

Examples of this command being used can be found in the
cue import [/docs/reference/command/cue-help-import/]
reference documentation.

RELATED CONTENT

 * Concept Guide: How CUE works with YAML [/docs/concept/how-cue-works-with-yaml/]
 * Concept Guide: How CUE works with JSON [/docs/concept/how-cue-works-with-json/]

Last modified November 6, 2025 [https://github.com/cue-lang/cuelang.org/commit/c4d2e727a59f5158a2be4946992d96c4612a9b88]

 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]

