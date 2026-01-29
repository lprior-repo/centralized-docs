---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-key-usage-plan-oas.html
title: Configure a method to use API keys with an OpenAPI definition
word_count: 453
filtered: true
elements_removed: 0
density_score: 0.74
---

Configure a method to use API keys with an OpenAPI definition - Amazon API Gateway
Configure a method to use API keys with an OpenAPI definition - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-key-usage-plan-oas)
# Configure a method to use API keys with an OpenAPI definition
You can use an OpenAPI definition to require API keys on a method.
For each method, create a security
requirement object to require an API key to invoke that method. Then, define `api\_key` in the security
definition. After you create your API, add the new API stage to your usage plan.
The following example creates an API and requires an API key for the `POST` and
`GET` methods:
OpenAPI 2.0
```
`{
"swagger" : "2.0",
"info" : {
"version" : "2024-03-14T20:20:12Z",
"title" : "keys-api"
},
"basePath" : "/v1",
"schemes" : [ "https" ],
"paths" : {
"/pets" : {
"get" : {
"responses" : { },
"security" : [ {
"api\_key" : [ ]
} ],
"x-amazon-apigateway-integration" : {
"type" : "http\_proxy",
"httpMethod" : "GET",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets/",
"passthroughBehavior" : "when\_no\_match"
}
},
"post" : {
"responses" : { },
"security" : [ {
"api\_key" : [ ]
} ],
"x-amazon-apigateway-integration" : {
"type" : "http\_proxy",
"httpMethod" : "GET",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets/",
"passthroughBehavior" : "when\_no\_match"
}
}
}
},
"securityDefinitions" : {
"api\_key" : {
"type" : "apiKey",
"name" : "x-api-key",
"in" : "header"
}
}
}`
```
OpenAPI 3.0
```
`{
"openapi" : "3.0.1",
"info" : {
"title" : "keys-api",
"version" : "2024-03-14T20:20:12Z"
},
"servers" : [ {
"url" : "{basePath}",
"variables" : {
"basePath" : {
"default" : "v1"
}
}
} ],
"paths" : {
"/pets" : {
"get" : {
"security" : [ {
"api\_key" : [ ]
} ],
"x-amazon-apigateway-integration" : {
"httpMethod" : "GET",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets/",
"passthroughBehavior" : "when\_no\_match",
"type" : "http\_proxy"
}
},
"post" : {
"security" : [ {
"api\_key" : [ ]
} ],
"x-amazon-apigateway-integration" : {
"httpMethod" : "GET",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets/",
"passthroughBehavior" : "when\_no\_match",
"type" : "http\_proxy"
}
}
}
},
"components" : {
"securitySchemes" : {
"api\_key" : {
"type" : "apiKey",
"name" : "x-api-key",
"in" : "header"
}
}
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create and configure API keys and usage plans with CloudFormation
Test usage plans for REST APIs in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.