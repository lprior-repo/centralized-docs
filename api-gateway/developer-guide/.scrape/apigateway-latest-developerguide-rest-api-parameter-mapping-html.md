---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-api-parameter-mapping.html
title: rest api parameter mapping.html
word_count: 348
filtered: true
elements_removed: 0
density_score: 0.87
---

Parameter mapping for REST APIs in API Gateway - Amazon API Gateway
Parameter mapping for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#rest-api-parameter-mapping)
###### Note
If you are using an HTTP API, see [Transform API requests and responses for HTTP APIs in API Gateway](./http-api-parameter-mapping.html).
In parameter mapping, you map request or response parameters. You can map parameters using parameter mapping
expressions or static values. For a list of mapping expressions, see [Parameter mapping source reference for REST APIs in API Gateway](./rest-api-parameter-mapping-sources.html). You can use parameter mapping in your integration request for
proxy and non-proxy integrations, but to use parameter mapping for an integration response, you need a non-proxy
integration.
For example, you can map the method request header parameter `puppies` to the integration request
header parameter `DogsAge0`. Then, if a client sends the header `puppies:true` to your API,
the integration request sends the request header `DogsAge0:true` to the integration endpoint. The following diagram shows the request lifecycle of this example.
![Diagram of API Gateway parameter mapping example for a request](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/parameter-mapping-example1.png)
To create this example using API Gateway, see [Example 1: Map a method request parameter to an
integration request parameter](./request-response-data-mappings.html#request-response-data-mappings-example-1).
As another
example, you can also map the integration response header parameter `kittens` to the method response
header parameter `CatsAge0`. Then, if the integration endpoint returns `kittens:false`, the
client receives the header `CatsAge0:false`. The following diagram shows the request lifecycle of this example.
![Diagram of API Gateway parameter mapping example for a response](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/parameter-mapping-example2.png)
###### Topics
* [Parameter mapping examples for REST APIs in API Gateway](./request-response-data-mappings.html)
* [Parameter mapping source reference for REST APIs in API Gateway](./rest-api-parameter-mapping-sources.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Data transformations
Parameter mapping examples
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.