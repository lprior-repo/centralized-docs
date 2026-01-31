---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#5-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary: cluster: live05. region: IMEA
---


 * 
   
   Copied!
   config-a.yaml
 * 
   
   Copied!
   config-b.yaml
 * 
   
   Copied!
   config-c.yaml

Copy code
Copied!

cluster: live05
region: IMEA
repository: source.company.example/alpha
tags:
  - prod


Copy code
Copied!

cluster: live03333333333333
repository: github.com/Alex_Personal_Account/alpha-fork
region: UK
tags:
  - dev


Copy code
Copied!

cluster: live05
region: APAC
repository: source.company.example/alpha

TERMINAL

Copy code
Copied!

$ cue vet -c schema.cue -d '#Config' config-a.yaml config-b.yaml config-c.yaml
