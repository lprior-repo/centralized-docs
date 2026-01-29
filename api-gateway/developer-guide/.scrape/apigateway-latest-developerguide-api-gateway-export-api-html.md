---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-export-api.html
title: Export a REST API from API Gateway
word_count: 787
filtered: true
elements_removed: 0
density_score: 0.83
---

Export a REST API from API Gateway - Amazon API Gateway
Export a REST API from API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-export-api)
[Request to export a REST API](#api-gateway-export-api-request)[Download REST API OpenAPI
definition in JSON](#api-gateway-export-api-download-swagger-json)[Download REST API OpenAPI
definition in YAML](#api-gateway-export-api-download-swagger-yaml)[Download REST API
OpenAPI definition with Postman extensions in JSON](#api-gateway-export-api-download-swagger-json-with-postman)[Download REST API
OpenAPI definition with API Gateway integration in YAML](#api-gateway-export-api-download-swagger-yaml-with-apig)[Export REST API using the API Gateway
console](#api-gateway-export-api-from-console)
# Export a REST API from API Gateway
Once you created and configured a REST API in API Gateway, using the API Gateway console or
otherwise, you can export it to an OpenAPI file using the API Gateway Export API, which is part
of the Amazon API Gateway Control Service. To use the API Gateway Export API, you need to sign your API requests. For more information about signing requests, see [Signing AWS API requests](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-signing.html) in the *IAM User Guide*. You have options to include the API Gateway
integration extensions, as well as the [Postman](https://www.postman.com) extensions, in the exported OpenAPI definition file.
###### Note
When exporting the API using the AWS CLI, be sure to include the extensions parameter as shown in the following example, to ensure that the `x-amazon-apigateway-request-validator` extension is included:
```
`aws apigateway get-export --parameters extensions='apigateway' --rest-api-id abcdefg123 --stage-name dev --export-type swagger latestswagger2.json`
```
You cannot export an API if its payloads are not of the `application/json` type. If you try, you will get an error response stating that JSON body models are not found.
## Request to export a REST API
With the Export API, you export an existing REST API by submitting a GET request,
specifying the to-be-exported API as part of URL paths. The request URL is of the
following format:
OpenAPI 3.0
```
`
https://`&lt;host&gt;`/restapis/`&lt;&lt;restapi\_id&gt;&gt;`/stages/`&lt;&lt;stage\_name&gt;&gt;`/exports/oas30
`
```
OpenAPI 2.0
```
`
https://`&lt;host&gt;`/restapis/`&lt;&lt;restapi\_id&gt;&gt;`/stages/`&lt;&lt;stage\_name&gt;&gt;`/exports/swagger
`
```
You can append the `extensions` query string to specify whether to
include API Gateway extensions (with the `integration` value) or Postman
extensions (with the `postman` value).
In addition, you can set the `Accept` header to
`application/json` or `application/yaml` to receive the
API definition output in JSON or YAML format, respectively.
For more information about submitting GET requests using the API Gateway Export
API, see [GetExport](https://docs.aws.amazon.com/apigateway/latest/api/API_GetExport.html).
###### Note
If you define models in your API, they must be for the content type of "application/json" for API Gateway to export the model.
Otherwise, API Gateway throws an exception with the "Only found non-JSON body models for ..." error message.
Models must contain properties or be defined as a particular JSONSchema type.
## Download REST API OpenAPI
definition in JSON
To export and download a REST API in OpenAPI definitions in JSON format:
OpenAPI 3.0
```
`
GET /restapis/`&lt;&lt;restapi\_id&gt;&gt;`/stages/`&lt;&lt;stage\_name&gt;&gt;`/exports/oas30
Host: apigateway.`&lt;region&gt;`.amazonaws.com
Accept: application/json
`
```
OpenAPI 2.0
```
`
GET /restapis/`&lt;&lt;restapi\_id&gt;&gt;`/stages/`&lt;&lt;stage\_name&gt;&gt;`/exports/swagger
Host: apigateway.`&lt;region&gt;`.amazonaws.com
Accept: application/json
`
```
Here, ``&lt;region&gt;`` could be, for
example, `us-east-1`. For all the regions where API Gateway is available, see
[Regions and Endpoints](https://docs.aws.amazon.com/general/latest/gr/rande.html#apigateway_region).
## Download REST API OpenAPI
definition in YAML
To export and download a REST API in OpenAPI definitions in YAML format:
OpenAPI 3.0
```
`
GET /restapis/`&lt;&lt;restapi\_id&gt;&gt;`/stages/`&lt;&lt;stage\_name&gt;&gt;`/exports/oas30
Host: apigateway.`&lt;region&gt;`.amazonaws.com
Accept: application/yaml
`
```
OpenAPI 2.0
```
`
GET /restapis/`&lt;&lt;restapi\_id&gt;&gt;`/stages/`&lt;&lt;stage\_name&gt;&gt;`/exports/swagger
Host: apigateway.`&lt;region&gt;`.amazonaws.com
Accept: application/yaml
`
```
## Download REST API
OpenAPI definition with Postman extensions in JSON
To export and download a REST API in OpenAPI definitions with Postman in
JSON format:
OpenAPI 3.0
```
`
GET /restapis/`&lt;&lt;restapi\_id&gt;&gt;`/stages/`&lt;&lt;stage\_name&gt;&gt;`/exports/oas30?extensions=postman
Host: apigateway.`&lt;region&gt;`.amazonaws.com
Accept: application/json
`
```
OpenAPI 2.0
```
`
GET /restapis/`&lt;&lt;restapi\_id&gt;&gt;`/stages/`&lt;&lt;stage\_name&gt;&gt;`/exports/swagger?extensions=postman
Host: apigateway.`&lt;region&gt;`.amazonaws.com
Accept: application/json
`
```
## Download REST API
OpenAPI definition with API Gateway integration in YAML
To export and download a REST API in OpenAPI definitions with API Gateway integration in YAML
format:
OpenAPI 3.0
```
`
GET /restapis/`&lt;&lt;restapi\_id&gt;&gt;`/stages/`&lt;&lt;stage\_name&gt;&gt;`/exports/oas30?extensions=integrations
Host: apigateway.`&lt;region&gt;`.amazonaws.com
Accept: application/yaml
`
```
OpenAPI 2.0
```
`
GET /restapis/`&lt;&lt;restapi\_id&gt;&gt;`/stages/`&lt;&lt;stage\_name&gt;&gt;`/exports/swagger?extensions=integrations
Host: apigateway.`&lt;region&gt;`.amazonaws.com
Accept: application/yaml
`
```
## Export REST API using the API Gateway
console
After [deploying your REST API to a stage](./set-up-deployments.html#create-deployment), you can proceed to export the API in the stage to an OpenAPI file using the API Gateway console.
In the **Stages** pane in the API Gateway console,
choose **Stage actions**, **Export**.
![Export REST API using the API Gateway console](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/export-new-console.png)
Specify an
**API specification type**, **Format**, and **Extensions** to
download your API's OpenAPI definition.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Errors and warnings from importing your API into API Gateway
Publish
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.