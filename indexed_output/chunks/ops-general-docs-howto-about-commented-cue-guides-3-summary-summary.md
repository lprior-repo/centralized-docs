---
doc_id: ops/general/docs-howto-about-commented-cue-guides
chunk_id: ops/general/docs-howto-about-commented-cue-guides#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: f1: conflicting values 123 and string (mismatched types int and string):. f2: conflicting values \"some string value\" and int (mismatched types string and int):
---

f1: conflicting values 123 and string (mismatched types int and string):
    ./data.yml:2:5
    ./example.cue:7:5
f2: conflicting values "some string value" and int (mismatched types string and int):
    ./data.yml:5:5
    ./example.cue:11:6

Commented CUE guides require you, the reader, to modify their examples
before you use them. Don’t use the CUE exactly as it’s presented, but first
adapt the variable names and data structures to suit your configuration.

Unlike some of our longer, step-by-step guides, the features showcased in
