---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#12-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: configMap: data: \"point. yaml\": yaml
---


configMap: data: "point.yaml": yaml.Marshal({
	x: 1.2
	y: 3.45
})

TERMINAL

Copy code
Copied!

$ cue export config.cue --out json
{
    "configMap": {
        "data": {
            "point.yaml": "x: 1.2\n\"y\": 3.45\n"
        }
    }
}

PARSING ENCODED YAML

The yaml.Unmarshal function performs the reverse operation to yaml.Marshal:
it turns a string containing YAML into the structure represented by
the encoded data.

Here, some encoded YAML data is emitted as JSON:

Copied!
file.cue

Copy code
Copied!

