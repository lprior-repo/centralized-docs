---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-api-routing-rules-examples.html
title: Examples of how API Gateway evaluates routing rules
word_count: 1194
filtered: true
elements_removed: 0
density_score: 0.80
---

Examples of how API Gateway evaluates routing rules - Amazon API Gateway
Examples of how API Gateway evaluates routing rules - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#rest-api-routing-rules-examples)
[Example 1: Routing rules only](#rest-api-routing-rules-examples-rule-only)[Example 2: Routing rules and API mappings](#rest-api-routing-rules-examples-rule-and-mappings)[Example 3: Routing rules and API mappings with multiple level](#rest-api-routing-rules-examples-rule-and-mappings-with-multiple-levels)[Example 4: Routing rules for wildcard
domain names](#rest-api-routing-rules-examples-rule-for-wildcard-domains)
# Examples of how API Gateway evaluates routing rules
The following section shows four examples of how API Gateway evaluates routing rules and API mappings.
## Example 1: Routing rules only
In this example, the custom domain name `https://petstore.example.com` has the routing mode set to
`ROUTING\_RULE\_ONLY` and has the following routing
rules and priorities.
|
Rule ID
|
Priority
|
Conditions
|
Action
|
|
`abc123`
|
10
|
If request contains header: `Hello:World`
|
Target API 1
|
|
`zzz000`
|
50
|
If request contains headers: `Accept:image/webp` and
`Pet:Dog-\*`
and if the base path contains `PetStoreShopper`
|
Target API 2
|
|
`efg456`
|
100
|
None
|
Target API 3
|
The following table shows how API Gateway applies the previous routing rules to example requests.
|Request|Selected API|Explanation|
|
`https://petstore.example.com -h "Hello:World"`
|
Target API 1
|
The request matches the routing rule `abc123`.
|
|
`https://petstore.example.com/PetStoreShopper -h "Hello:World", "Pet:Dog-Bella", "Accept:image/webp"`
|
Target API 1
|
API Gateway evaluates all routing rules in priority order. Routing rule `abc123` has the
first priority and the conditions match, so API Gateway invokes Target API 1.
Although the conditions of the
request also match routing rule `zzz000`, API Gateway doesn't evaluate any other routing rules after
it makes a match.
|
|
`https://petstore.example.com/PetStoreShopper -h "Pet:Dog-Bella", "Accept:image/webp"`
|
Target API 2
|
The request matches the routing rule `zzz000`. This was a match because the
`Pet:Dog-Bella` was a string match to `Pet:Dog-\*`
|
|
`https://petstore.example.com/PetStoreShopper -h "Pet:Dog-Bella"`
|
Target API 3
|
The request doesn't match the routing rule `abc123`. The request doesn't match routing
rule `zzz000` as all the required headers aren't present. The next priority rule matches
all incoming requests, so API Gateway invokes Target API 3.
|
## Example 2: Routing rules and API mappings
In this example, the custom domain name `https://petstore.diagram.example.com` has the routing mode set to
`ROUTING\_RULE\_THEN\_API\_MAPPING` and has the following
routing rules and API mappings.
|
Rule ID
|
Priority
|
Conditions
|
Action
|
|
`abc123`
|
1
|
If request the base contains `pets`
|
Invoke the `Prod` stage of the `PetStore` API.
|
|
`000zzz`
|
5
|
If request contains headers: `Cookie`:`\*ux=beta\*` and
and if the base path contains `/refunds`
|
Invoke the `Beta` stage of the `Refunds` API.
|
The following table shows API mappings for `https://petstore.backup.example.com`.
|
API mapping
|
Selected API
|
|
`/refunds`
|
Invoke the `Prod` stage of the `Refunds` API.
|
|
`(none)`
|
Invoke the `Prod` stage of the `Search` API.
|
The following diagram shows how API Gateway applies the previous routing rules and API mappings to example
requests. The example requests are summarized in the table after this diagram.
![Diagram of how API Gateway applies the previous routing rules and API mappings.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/rr-diagram.png)
The following table shows how API Gateway applies the previous routing rules and API mappings to example requests.
|Request|Selected API|Explanation|
|
`https://petstore.diagram.com/pets`
|
The `Prod` stage of the `PetStore` API.
|
The request matches routing rule `abc123`.
|
|
`https://petstore.diagram.example.com/refunds
-h "Cookie:lang=en-us;ux=beta"`
|
The `Beta` stage of the `Refunds` API.
|
The request matches routing rule `000zzz`. The `Cookie` header contains the
correct `\*contains\*` match and base path match for this condition.
|
|
`https://petstore.diagram.example.com/refunds`
|
The `Prod` stage of the `Refunds` API.
|
The request doesn't have the required headers to match the routing rule `zzz000`. If
API Gateway can't successfully match a routing rule, it falls back to API mappings. API Gateway can
map the base path to the `Prod` stage of the `Refunds` API.
|
|
`https://petstore.diagram.example.com/`
|
The `Prod` stage of the `Search` API.
|
The request matches the API mapping to the empty path `(none)`.
|
## Example 3: Routing rules and API mappings with multiple level
In this example, the custom domain name `https://petstore.backup.example.com` has the routing mode set to
`ROUTING\_RULE\_THEN\_API\_MAPPING` and has the following
routing rules and API mappings.
The following table shows routing rules for `https://petstore.backup.example.com`.
|
Rule ID
|
Priority
|
Conditions
|
Action
|
|
`abc123`
|
10
|
If request contains header: `Hello:World`
|
Target API 1
|
|
`000zzz`
|
50
|
If request contains headers: `Accept`:`image/webp` and `Pet:Dog-\*`
and if the base path contains `PetStoreShopper`
|
Target API 2
|
The following table shows API mappings for `https://petstore.backup.example.com`.
|
API mapping
|
Selected API
|
|
`PetStoreShopper`
|
Target API 3
|
|
`PetStoreShopper/cats`
|
Target API 4
|
The following table shows how API Gateway applies the previous routing rules and API mappings to example requests.
|Request|Selected API|Explanation|
|
`https://petstore.example.com/PetStoreShopper -h "Accept:image/webp", "Pet:Cats" `
|
Target API 3
|
The request doesn't have the required headers to match the routing rule `zzz000`. If
API Gateway can't successfully match a routing rule, it falls back to API mappings. API Gateway can map the base
path to Target API 3.
|
|
`https://petstore.example.com/PetStoreShopper/cats -h "Hello:World"`
|
Target API 1
|
The request matches routing rule `abc123`. If the routing mode is set to
`ROUTING\_RULE\_THEN\_API\_MAPPING`, routing rules always take priority over API
mappings.
|
|
`https://petstore.example.com/Admin -h "Pet:Dog-Bella"`
|
None
|
The request doesn't match any routing rules or API mappings. Since there is no default routing
rule, API Gateway rejects the call and sends the caller a `403 Forbidden` status code.
|
## Example 4: Routing rules for wildcard
domain names
In this example, the custom domain name `https://\*.example.com` is a wildcard domain name. The
wildcard supports all subdomains which route back to the same domain. The following example routing rules change this
behavior to allow subdomains to route to different target APIs by using the `Host` header.
The following table shows routing rules for `https://\*.example.com`.
|
Rule ID
|
Priority
|
Conditions
|
Action
|
|
`abc123`
|
10
|
If request contains header: `Host:a.example.com`
|
Target API 1
|
|
`000zzz`
|
50
|
If request contains headers: `Host:b.example.com`
|
Target API 2
|
|
`efg456`
|
500
|
None
|
Target API 3
|
The following table shows how API Gateway applies the previous routing rules to example requests.
|Request|Selected API|Explanation|
|
`https://a.example.com`
|
Target API 1
|
The `Host` header is `a.example.com`. This request matches routing rule
`abc123`.
|
|
`https://b.example.com`
|
Target API 2
|
The `Host` header is `b.example.com`. This request matches routing rule
`000zzz`.
|
|
`https://testing.example.com`
|
Target API 3
|
This matches the catch-all routing rule
`efg456`.
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Routing rules
How to use routing rules
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.