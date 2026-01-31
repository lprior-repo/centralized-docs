---
doc_id: ops/general/docs-tour-basics
chunk_id: ops/general/docs-tour-basics#1-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 647
summary: $ cue export file. cue --out json
---


$ cue export file.cue --out json
{
    "one": 1,
    "two": 2,
    "piPlusOne": 4.141592653589793238462643383279503,
    "quoted field names": {
        "two-and-a-half": 2.5,
        "three point three": 3.3,
        "four^four": 256
    },
    "aList": [
        1,
        2,
        3
    ]
}

Last modified May 23, 2025 [https://github.com/cue-lang/cuelang.org/commit/2bb11f637a7a1cfe18b3bed5c0717cf07f5ea21d]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/tour/basics/json-superset/&text=In%20its%20simplest%20form,%20CUE%20looks%20a%20lot%20like%20JSON.%20This%20is%20because%20CUE%20is%20a%20superset%20of%20JSON.%0aOr,%20put%20differently:%20all%20valid%20JSON%20is%20CUE%20%28but%20not%20vice%20versa%29.%0aCUE%20significantly%20reduces%20the%20pain%20of%20dealing%20with%20JSON%20by%20introducing%20several%20conveniences,%20including:%0aC-style%20comments%20are%20allowed%20field%20names%20without%20special%20characters%20don&rsquo;t%20need%20to%20be%20quoted%20commas%20after%20a%20field%20are%20optional%20%28and%20are%20usually%20omitted%29%20commas%20after%20the%20final%20element%20of%20a%20list%20are%20allowed%20the%20outermost%20curly%20braces%20in%20a%20CUE%20file%20are%20optional%20JSON%20objects%20are%20called%20structs%20or%20maps%20in%20CUE.%20JSON%20arrays%20are%20called%20lists%0aObject%20members%20are%20called%20fields,%20which%20link%20their%20name,%20or%20label,%20to%20a%20value.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/tour/basics/json-superset/&summary=In%20its%20simplest%20form,%20CUE%20looks%20a%20lot%20like%20JSON.%20This%20is%20because%20CUE%20is%20a%20superset%20of%20JSON.%0aOr,%20put%20differently:%20all%20valid%20JSON%20is%20CUE%20%28but%20not%20vice%20versa%29.%0aCUE%20significantly%20reduces%20the%20pain%20of%20dealing%20with%20JSON%20by%20introducing%20several%20conveniences,%20including:%0aC-style%20comments%20are%20allowed%20field%20names%20without%20special%20characters%20don&rsquo;t%20need%20to%20be%20quoted%20commas%20after%20a%20field%20are%20optional%20%28and%20are%20usually%20omitted%29%20commas%20after%20the%20final%20element%20of%20a%20list%20are%20allowed%20the%20outermost%20curly%20braces%20in%20a%20CUE%20file%20are%20optional%20JSON%20objects%20are%20called%20structs%20or%20maps%20in%20CUE.%20JSON%20arrays%20are%20called%20lists%0aObject%20members%20are%20called%20fields,%20which%20link%20their%20name,%20or%20label,%20to%20a%20value.%0a]
