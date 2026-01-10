---
id: ops/general/docs-concept-how-cue-works-with-toml
title: Docs Concept How Cue Works With Toml
category: ops
tags: ["ops", "works"]
---

# Docs Concept How Cue Works With Toml

> **Context**: **Source:** https://cuelang.org/docs/concept/how-cue-works-with-toml/

# How CUE works with TOML | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-toml/

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

 1. Concept Guides [https://cuelang.org/docs/concept/]


 2. HOW CUE WORKS WITH TOML

jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews
jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews

Github profile

[https://github.com/jpluscplusm]

Search all content by this author

[/search/?q=author:jpluscplusm]
 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]

READING AND WRITING TOML

The cue command natively supports reading and writing TOML files and data.
TOML can be processed by CUE’s wide range of data, schema, and policy
validation capabilities.
Data in any supported encoding can be read and exported as TOML
– as demonstrated here by
cue export [/docs/reference/command/cue-export/]
unifying its TOML, JSON, and CUE input files and producing TOML:

 * 
   
   Copied!
   a.toml
 * 
   
   Copied!
   b.json
 * 
   
   Copied!
   c.cue

Copy code
Copied!

a = "1"

[b]
c = 2.2

[b.d]
e = 3


Copy code
Copied!

{
    "f": "4",
    "g": 5.5
}


Copy code
Copied!

b: _
g: _

h: "six"
b: d: i: g + b.d.e

TERMINAL

Copy code
Copied!

$ cue export --out toml a.toml b.json c.cue
a = '1'
f = '4'
g = 5.5
h = 'six'

[b]
c = 2.2

[b.d]
e = 3
i = 8.5

The cue command can read and write
a range of other formats [/docs/integration/]
as well as TOML.

VALIDATING TOML FILES AGAINST A SCHEMA

CUE is often used to make systems safer without having to teach the underlying
system components about CUE. Because the cue tool can validate TOML files
using CUE’s powerful and compact constraint syntax, it’s easy to add
“pre-flight” checks to existing processes with CUE.

In this example,
cue vet [/docs/reference/command/cue-help-vet/]
is used to check that a hypothetical system’s TOML input files are valid - and
catches a problematic deployment early in the process:

Copied!
schema.cue

Copy code
Copied!

import "strings"

#Config: {
	cluster!:    strings.MaxRunes(16)
	region!:     #Region
	repository!: =~#"^source\.company\.example/"#
	tags?: [...#Tags]
}
#Region: "APAC" | "IMEA"
#Tags:   "prod" | "stage" | "qa" | "test" | "dev"

 * 
   
   Copied!
   config-a.toml
 * 
   
   Copied!
   config-b.toml
 * 
   
   Copied!
   config-c.toml

Copy code
Copied!

cluster = 'live05'
region = 'IMEA'
repository = 'source.company.example/alpha'
tags = ['prod']


Copy code
Copied!

cluster = 'live03333333333333'
region = 'UK'
repository = 'github.com/Alex_Personal_Account/alpha-fork'
tags = ['dev']


Copy code
Copied!

cluster = 'live05'
region = 'APAC'
repository = 'source.company.example/alpha'

TERMINAL

Copy code
Copied!

$ cue vet -c schema.cue -d '#Config' config-a.toml config-b.toml config-c.toml
region: 2 errors in empty disjunction:
region: conflicting values "APAC" and "UK":
    ./config-b.toml:2:10
    ./schema.cue:5:15
    ./schema.cue:9:10
region: conflicting values "IMEA" and "UK":
    ./config-b.toml:2:10
    ./schema.cue:5:15
    ./schema.cue:9:19
cluster: invalid value "live03333333333333" (does not satisfy strings.MaxRunes(16)):
    ./schema.cue:4:15
    ./config-b.toml:1:11
    ./schema.cue:4:32
repository: invalid value "github.com/Alex_Personal_Account/alpha-fork" (out of bound =~"^source\\.company\\.example/"):
    ./schema.cue:6:15
    ./config-b.toml:3:14

PROCESSING AND TRANSFORMING TOML FILES

The cue tool can read and transform TOML files, producing output data in any
shape that’s required. For example:

Copied!
transform.cue

Copy code
Copied!

a: int
b: int
c: 1 + a*b

Copied!
data.toml

Copy code
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


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/how-cue-works-with-toml/&text=%20Reading%20and%20writing%20TOML%20The%20cue%20command%20natively%20supports%20reading%20and%20writing%20TOML%20files%20and%20data.%20TOML%20can%20be%20processed%20by%20CUE&rsquo;s%20wide%20range%20of%20data,%20schema,%20and%20policy%20validation%20capabilities.%20Data%20in%20any%20supported%20encoding%20can%20be%20read%20and%20exported%20as%20TOML%20&ndash;%20as%20demonstrated%20here%20by%20cue%20export%20unifying%20its%20TOML,%20JSON,%20and%20CUE%20input%20files%20and%20producing%20TOML:%0aCopied!%20a.toml%20Copied!%20b.json%20Copied!%20c.cue%20Copy%20code%20Copied!%20a%20=%20&#34;1&#34;%20[b]%20c%20=%202.2%20[b.d]%20e%20=%203%20Copy%20code%20Copied!%20%7b%20&#34;f&#34;:%20&#34;4&#34;,%20&#34;g&#34;:%205.5%20%7d%20Copy%20code%20Copied!%20b:%20_%20g:%20_%20h:%20&#34;six&#34;%20b:%20d:%20i:%20g%20+%20b.d.e%20TERMINAL%20Copy%20code%20Copied!%20$%20cue%20export%20--out%20toml%20a.toml%20b.json%20c.cue%20a%20=%20&#39;1&#39;%20f%20=%20&#39;4&#39;%20g%20=%205.5%20h%20=%20&#39;six&#39;%20[b]%20c%20=%202.2%20[b.d]%20e%20=%203%20i%20=%208.5%20The%20cue%20command%20can%20read%20and%20write%20a%20range%20of%20other%20formats%20as%20well%20as%20TOML.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/how-cue-works-with-toml/&summary=%20Reading%20and%20writing%20TOML%20The%20cue%20command%20natively%20supports%20reading%20and%20writing%20TOML%20files%20and%20data.%20TOML%20can%20be%20processed%20by%20CUE&rsquo;s%20wide%20range%20of%20data,%20schema,%20and%20policy%20validation%20capabilities.%20Data%20in%20any%20supported%20encoding%20can%20be%20read%20and%20exported%20as%20TOML%20&ndash;%20as%20demonstrated%20here%20by%20cue%20export%20unifying%20its%20TOML,%20JSON,%20and%20CUE%20input%20files%20and%20producing%20TOML:%0aCopied!%20a.toml%20Copied!%20b.json%20Copied!%20c.cue%20Copy%20code%20Copied!%20a%20=%20&#34;1&#34;%20[b]%20c%20=%202.2%20[b.d]%20e%20=%203%20Copy%20code%20Copied!%20%7b%20&#34;f&#34;:%20&#34;4&#34;,%20&#34;g&#34;:%205.5%20%7d%20Copy%20code%20Copied!%20b:%20_%20g:%20_%20h:%20&#34;six&#34;%20b:%20d:%20i:%20g%20+%20b.d.e%20TERMINAL%20Copy%20code%20Copied!%20$%20cue%20export%20--out%20toml%20a.toml%20b.json%20c.cue%20a%20=%20&#39;1&#39;%20f%20=%20&#39;4&#39;%20g%20=%205.5%20h%20=%20&#39;six&#39;%20[b]%20c%20=%202.2%20[b.d]%20e%20=%203%20i%20=%208.5%20The%20cue%20command%20can%20read%20and%20write%20a%20range%20of%20other%20formats%20as%20well%20as%20TOML.%0a]


How CUE works with Protocol Buffers
[/docs/concept/how-cue-works-with-protocol-buffers/]How CUE works with YAML
[/docs/concept/how-cue-works-with-yaml/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
   * Popular guides [/docs/concept/popular-guides/]
   * The Logic of CUE [/docs/concept/the-logic-of-cue/]
   * Modules [/docs/concept/modules/]
   * Frequently Asked Questions [/docs/concept/faq/]
   * How CUE works with TOML [/docs/concept/how-cue-works-with-toml/]
      1. Reading and writing TOML
      2. Validating TOML files against a schema
      3. Processing and transforming TOML files
      4. Embedding TOML file data inside CUE
      5. Encoding TOML inside CUE
      6. Converting TOML files to CUE
      7. Related content
 * References [/docs/reference/]

Hide side navigation


Show side navigation

Get Started

 * Documentation [/docs/]
 * Language Tour [/docs/tour/]
 * Playground [/play/]
 * Install CUE [/docs/introduction/installation/]

Community

 * The CUE Community [/community]
 * Contributing [https://github.com/cue-lang/cue/blob/master/CONTRIBUTING.md#contribution-guide]
 * Code of Conduct [/docs/reference/code-of-conduct/]
 * Slack Workspace [/s/slack]
 * Discord Server [/s/discord]

Connect

 * GitHub [https://github.com/cue-lang/cue]
 * X (Twitter) [https://twitter.com/cue_lang]
 * Bluesky [https://bsky.app/profile/cuelang.org]
 * YouTube [https://www.youtube.com/@cuelang/videos]

 * © 2025 CUE
 * Privacy policy [/privacy-policy/]
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Fconcept%2Fhow-cue-works-with-toml%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


Homepage of CUE [/]
CUE v0.15 is now available – learn more about its new features and improvements [https://github.com/cue-lang/cue/releases/tag/v0.15.0]
Install CUE

[/docs/introduction/installation/]

Close

Homepage of CUE [/]


Hide menu
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]
 * Install [/docs/introduction/installation/]
 * 

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

## See Also

- [Documentation Index](./COMPASS.md)

