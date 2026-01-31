---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#2-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 515
summary: $ cue export --out yaml data. yaml transform
---


Copied!
data.yaml

Copy code
Copied!

a: 5
b: 4

TERMINAL

Copy code
Copied!

$ cue export --out yaml data.yaml transform.cue
a: 5
b: 4
c: 21

Learn more about transforming data with CUE in these How-to guides:

 * Transforming YAML with CUE [/docs/howto/transform-yaml-with-cue/]
 * Combining multiple YAML files into a list [/docs/howto/combine-multiple-yaml-files-into-a-list/]
 * Combining multiple YAML files by using file metadata [/docs/howto/combine-multiple-yaml-files-by-using-file-metadata/]

EMBEDDING YAML FILE DATA INSIDE CUE

Requires CUE v0.12.0 or later

The file embedding [/docs/howto/embed-files-in-cue-evaluation/]
feature allows data files (including YAML) to be read when some CUE is evaluated.
This provides an alternative way to use CUE to validate data files against
schemas and constraints, and also gives CUE configurations access to data
stored in non-CUE files:

Copied!
example.cue

Copy code
Copied!

@extern(embed)

package p

_conf: _ @embed(file=config.yaml)
_data: _ @embed(glob=env/*.yml)

info: {
	version: _conf.version
	source:  _data["env/\(_conf.source).yml"].text
}

 * 
   
   Copied!
   config.yaml
 * 
   
   Copied!
   env/foo.yml
 * 
   
   Copied!
   env/bar.yml

Copy code
Copied!

version: 1.42.0
source: "bar"


Copy code
Copied!

text: "Some foo"


Copy code
Copied!

text: "A bar"

TERMINAL

Copy code
Copied!

$ cue export --out yaml
info:
  version: 1.42.0
  source: A bar

File embedding is available from CUE v0.12.0 onwards.
Find out more about this powerful validation feature in
Embedding files in a CUE evaluation [/docs/howto/embed-files-in-cue-evaluation/].

ENCODING YAML INSIDE CUE

CUE is frequently used to generate configuration files. Some systems allow
their configuration files to contain YAML encoded in string fields,
irrespective of the file’s main data format.

CUE’s standard library provides
a built-in yaml package [https://pkg.go.dev/cuelang.org/go/pkg/encoding/yaml]
containing functions that generate, parse, and validate YAML from
within CUE - some of which are shown here.
