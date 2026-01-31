---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#5-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 133
summary: 	repository!: =~#\"^source\. #Region: \"APAC\" | \"IMEA\"
---

	repository!: =~#"^source\.company\.example/"#
	tags?: [...#Tags]
}
#Region: "APAC" | "IMEA"
#Tags:   "prod" | "stage" | "qa" | "test" | "dev"

 * 
   
   Copied!
   config-a.json
 * 
   
   Copied!
   config-b.json
 * 
   
   Copied!
   config-c.json

Copy code
Copied!

{
    "cluster": "live05",
    "region": "IMEA",
    "repository": "source.company.example/alpha",
    "tags": [
        "prod"
    ]
}


Copy code
Copied!

{
    "cluster": "live03333333333333",
    "repository": "github.com/Alex_Personal_Account/alpha-fork",
