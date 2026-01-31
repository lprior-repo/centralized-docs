---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#14-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: 	// Render as indented JSON. 	b, err := json
---

		log.Fatal(err)
	}

	// Render as indented JSON
	b, err := json.MarshalIndent(topValue, "", "  ")
	if err != nil {
		log.Fatal(err)
	}
	b = append(b, '\n')
	os.Stdout.Write(b)
}

Running this code successfully expresses the constraints in our original
schema.cue file as an OpenAPI document:

TERMINAL

Copy code
Copied!

$ go run .
{
  "openapi": "3.0.0",
  "info": {
    "title": "A schema for the pet API.",
    "version": "v1.2.3"
  },
  "paths": {},
  "components": {
    "schemas": {
      "Kind": {
...

