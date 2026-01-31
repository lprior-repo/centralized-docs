---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#59-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 135
summary: Label           = [ identifier \"=\" ] LabelExpr . LabelExpr       = LabelName [ \"?\" | \"!\" ] | \"[\" AliasExpr \"]\" 
---

Label           = [ identifier "=" ] LabelExpr .
LabelExpr       = LabelName [ "?" | "!" ] | "[" AliasExpr "]" .
LabelName       = identifier | simple_string_lit | "(" AliasExpr ")" .

attribute       = "@" identifier "(" attr_tokens ")" .
attr_tokens     = { attr_token |
                    "(" attr_tokens ")" |
                    "[" attr_tokens "]" |
                    "{" attr_tokens "}" } .
attr_token      = /* any token except '(', ')', '[', ']', '{', or '}' */


Copy code
Copied!

Expression                             Result
