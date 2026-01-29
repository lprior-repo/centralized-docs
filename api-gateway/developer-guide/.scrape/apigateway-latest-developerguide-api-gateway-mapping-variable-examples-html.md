---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-mapping-variable-examples.html
title: Examples using variables for mapping template transformations for API Gateway
word_count: 1011
filtered: true
elements_removed: 0
density_score: 0.86
---

Examples using variables for mapping template transformations for API Gateway - Amazon API Gateway
Examples using variables for mapping template transformations for API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-mapping-variable-examples)
[Example 1: Pass multiple $context
variables to the integration endpoint](#context-variables-template-example)[Example 2: Pass all request parameters to
the integration endpoint via a JSON payload](#input-examples-mapping-templates)[Example 3: Pass a subsection of a method request to the
integration endpoint](#input-example-json-mapping-template)[Example 4: Use JSONPath expression to pass a subsection of
a method request to the integration endpoint](#input-example-inputs-mapping-template)[Example 5: Use a JSONPath expression to pass information
about a method request to the integration endpoint](#input-example-request-and-response)
# Examples using variables for mapping template transformations for API Gateway
The following examples show how to use `$context`, `input`, and `util`
variables in mapping templates. You can use a mock integration or a Lambda non-proxy integration that returns the
input event back to API Gateway. For a list of all supported variables for data transformations, see
[Variables for data transformations for API Gateway](./api-gateway-mapping-template-reference.html).
## Example 1: Pass multiple `$context`
variables to the integration endpoint
The following example shows a mapping template that maps incoming
`$context` variables to backend variables with slightly different names
in an integration request payload:
```
`{
"stage" : "$context.stage",
"request\_id" : "$context.requestId",
"api\_id" : "$context.apiId",
"resource\_path" : "$context.resourcePath",
"resource\_id" : "$context.resourceId",
"http\_method" : "$context.httpMethod",
"source\_ip" : "$context.identity.sourceIp",
"user-agent" : "$context.identity.userAgent",
"account\_id" : "$context.identity.accountId",
"api\_key" : "$context.identity.apiKey",
"caller" : "$context.identity.caller",
"user" : "$context.identity.user",
"user\_arn" : "$context.identity.userArn"
}`
```
The output of this mapping template should look like the following:
```
`{
stage: 'prod',
request\_id: 'abcdefg-000-000-0000-abcdefg',
api\_id: 'abcd1234',
resource\_path: '/',
resource\_id: 'efg567',
http\_method: 'GET',
source\_ip: '192.0.2.1',
user-agent: 'curl/7.84.0',
account\_id: '111122223333',
api\_key: 'MyTestKey',
caller: 'ABCD-0000-12345',
user: 'ABCD-0000-12345',
user\_arn: 'arn:aws:sts::111122223333:assumed-role/Admin/carlos-salazar'
}`
```
One of the variables is an API key. This example assumes that the method requires an API key.
## Example 2: Pass all request parameters to
the integration endpoint via a JSON payload
The following example passes all request parameters, including
`path`, `querystring`, and `header` parameters, through to
the integration endpoint via a JSON payload:
```
`#set($allParams = $input.params())
{
"params" : {
#end
}
}`
```
If a request has the following input parameters:
* A path parameter named `myparam`
* Query string parameters `querystring1=value1,value2`
* Headers `"header1" : "value1"`.
The output of this mapping template should look like the following:
```
`{"params":{"path":{"example2":"myparamm"},"querystring":{"querystring1":"value1,value2"},"header":{"header1":"value1"}}}
`
```
## Example 3: Pass a subsection of a method request to the
integration endpoint
The following example uses the input parameter `name` to retrieve only the `name`
parameter and the input parameter `input.json('$')` to retrieve the entire body of the method
request:
```
`{
"name" : "$input.params('name')",
"body" : $input.json('$')
}`
```
For a request that includes the query string parameters `name=Bella&amp;type=dog` and the following body:
```
`{
"Price" : "249.99",
"Age": "6"
}`
```
The output of this mapping template should look like the following:
```
`{
"name" : "Bella",
"body" : {"Price":"249.99","Age":"6"}
}`
```
This mapping template removes the query string parameter `type=dog`.
If the JSON input contains unescaped characters that cannot be parsed by
JavaScript, API Gateway might return a 400 response. Apply
`$util.escapeJavaScript($input.json('$'))` to ensure the
JSON input can be parsed properly.
The previous example with `$util.escapeJavaScript($input.json('$'))` applied is as follows:
```
`{
"name" : "$input.params('name')",
"body" : "$util.escapeJavaScript($input.json('$'))"
}`
```
In this case, the output of this mapping template should look like the following:
```
`{
"name" : "Bella",
"body": {"Price":"249.99","Age":"6"}
}`
```
## Example 4: Use JSONPath expression to pass a subsection of
a method request to the integration endpoint
The following example uses the JSONPath expressions to retrieve only the input parameter
`name` and the `Age` from the request body:
```
`{
"name" : "$input.params('name')",
"body" : $input.json('$.Age')
}`
```
For a request that includes the query string parameters `name=Bella&amp;type=dog` and the following body:
```
`{
"Price" : "249.99",
"Age": "6"
}`
```
The output of this mapping template should look like the following:
```
`{
"name" : "Bella",
"body" : "6"
}`
```
This mapping template removes the query string parameter `type=dog` and the
`Price` field from the body.
If a method request payload contains unescaped characters that cannot be parsed
by JavaScript, API Gateway might return a `400` response. Apply
`$util.escapeJavaScript()` to ensure the
JSON input can be parsed properly.
The previous example with `$util.escapeJavaScript($input.json('$.Age'))` applied is as follows:
```
`{
"name" : "$input.params('name')",
"body" : "$util.escapeJavaScript($input.json('$.Age'))"
}`
```
In this case, the output of this mapping template should look like the following:
```
`{
"name" : "Bella",
"body": "\\"6\\""
}`
```
## Example 5: Use a JSONPath expression to pass information
about a method request to the integration endpoint
The following example uses `$input.params()`, `$input.path()`, and
`$input.json()` to send information about a method request to the integration endpoint. This mapping
template uses the `size()` method to provide the number of elements in a list.
```
`{
"id" : "$input.params('id')",
"count" : "$input.path('$.things').size()",
"things" : $input.json('$.things')
}`
```
For a request that includes the path parameter `123` and the following body:
```
`{
"things": {
"1": {},
"2": {},
"3": {}
}
}`
```
The output of this mapping template should look like the following:
```
`{"id":"123","count":"3","things":{"1":{},"2":{},"3":{}}}`
```
If a method request payload contains unescaped characters that cannot be parsed
by JavaScript, API Gateway might return a `400` response. Apply
`$util.escapeJavaScript()` to ensure the
JSON input can be parsed properly.
The previous example with `$util.escapeJavaScript($input.json('$.things'))` applied is as follows:
```
`{
"id" : "$input.params('id')",
"count" : "$input.path('$.things').size()",
"things" : "$util.escapeJavaScript($input.json('$.things'))"
}`
```
The output of this mapping template should look like the following:
```
`{"id":"123","count":"3","things":"{\\"1\\":{},\\"2\\":{},\\"3\\":{}}"}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Modify the integration request and response
for integrations to AWS services
Variables for data
transformations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.