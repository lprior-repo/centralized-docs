---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#47-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 128
summary: (*\"tcp\"|\"udp\") & (\"udp\"|*\"tcp\")  \"tcp\". (*\"tcp\"|\"udp\") & (\"udp\"|\"tcp\")   \"tcp\"
---


(*"tcp"|"udp") & ("udp"|*"tcp")  "tcp"
(*"tcp"|"udp") & ("udp"|"tcp")   "tcp"
(*"tcp"|"udp") & "tcp"           "tcp"
(*"tcp"|"udp") & (*"udp"|"tcp")  "tcp" | "udp" // default is _|_

(*true | false) & bool           true
(*true | false) & (true | false) true

{a: 1} | {b: 1}                  {a: 1} | {b: 1}
{a: 1} | *{b: 1}                 {b:1}
*{a: 1} | *{b: 1}                {a: 1} | {b: 1}
({a: 1} | {b: 1}) & {a:1}        {a:1}  | {a: 1, b: 1}
({a:1}|*{b:1}) & ({a:1}|*{b:1})  {b:1}

BOTTOM AND ERRORS

