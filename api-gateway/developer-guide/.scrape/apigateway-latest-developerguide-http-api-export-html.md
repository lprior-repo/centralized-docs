---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-export.html
title: Export HTTP APIs from API Gateway
word_count: 496
filtered: true
elements_removed: 0
density_score: 0.87
---

Export HTTP APIs from API Gateway - Amazon API Gateway
Export HTTP APIs from API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-export)
[Export an OpenAPI 3.0 definition of a
stage by using the AWS CLI](#http-api-export.stage.example)[Export an OpenAPI 3.0 definition of your API's latest changes by using the AWS CLI](#http-api-export.latest.example)[Export an OpenAPI 3.0 definition by using the API Gateway console](#http-api-export.console)
# Export HTTP APIs from API Gateway
After you've created an HTTP API, you can export an OpenAPI 3.0 definition of your
API from API Gateway. You can either choose a stage to export, or export the latest configuration
of your API. You can also import an exported API definition into API Gateway to create another,
identical API. To learn more about importing API definitions, see [Importing an HTTP API](./http-api-open-api.html#http-api-import).
## Export an OpenAPI 3.0 definition of a
stage by using the AWS CLI
The following
[export-api](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/export-api.html) command exports an OpenAPI definition of an API stage named
`prod` to a YAML file named `stage-definition.yaml`.
The exported definition file includes [API Gateway extensions](./api-gateway-swagger-extensions.html) by default.
```
`aws apigatewayv2 export-api \\
--api-id `api-id` \\
--output-type `YAML` \\
--specification OAS30 \\
--stage-name `prod` \\
`stage-definition.yaml``
```
## Export an OpenAPI 3.0 definition of your API's latest changes by using the AWS CLI
The following [export-api](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/export-api.html) command exports an
OpenAPI definition of an HTTP API to a JSON file named `latest-api-definition.json`.
Because the command doesn't specify a stage, API Gateway exports the latest configuration of your API, whether it has
been deployed to a stage or not. The exported definition file doesn't include [API Gateway extensions](./api-gateway-swagger-extensions.html).
```
`aws apigatewayv2 export-api \\
--api-id `api-id` \\
--output-type `JSON` \\
--specification OAS30 \\
--no-include-extensions \\
`latest-api-definition.json``
```
For more information, see [ExportAPI](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid-exports-specification.html#apis-apiid-exports-specification-http-methods) in the *Amazon API Gateway Version 2 API Reference*.
## Export an OpenAPI 3.0 definition by using the API Gateway console
The following procedure shows how to export an OpenAPI definition of an HTTP API.
###### To export an OpenAPI 3.0 definition using the API Gateway console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose an HTTP API.
3. On the main navigation pane, under **Develop**, choose **Export**.
4. Select from the following options to export your API:
![Export options for HTTP APIs.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/export-http-api.png)
1. For **Source**, select a source for the OpenAPI 3.0 definition. You can choose a stage to export, or export the latest configuration
of your API.
2. Turn on **Include API Gateway extensions** to include [API Gateway extensions](./api-gateway-swagger-extensions.html).
3. For **Output format**, select an output format.
4. Choose **Download**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
OpenAPI
Publish
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.