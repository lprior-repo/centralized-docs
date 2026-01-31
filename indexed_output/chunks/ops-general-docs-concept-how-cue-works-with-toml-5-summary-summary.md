---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#5-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 143
summary: cluster = 'live05'. region = 'IMEA'
---


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
