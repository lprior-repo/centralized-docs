---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#55-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: 2 & >=2 & <=5           // 2, where 2 is either an int or float. 5 & >=1 & <=5         // 2
---



Copy code
Copied!

2 & >=2 & <=5           // 2, where 2 is either an int or float.
2.5 & >=1 & <=5         // 2.5
2 & >=1.0 & <3.0        // 2.0
2 & >1 & <3.0           // 2.0
2.5 & int & >1 & <5     // _|_
2.5 & float & >1 & <5   // 2.5
int & 2 & >1.0 & <3.0   // _|_
2.5 & >=(int & 1) & <5  // _|_
>=0 & <=7 & >=3 & <=10  // >=3 & <=7
!=null & 1              // 1
==[1, 2] & [1]          // _|_
!=[1, 2] & [1]          // [1]

STRUCTS

A struct is a set of elements called fields, each of
which has a name, called a label, and value.
