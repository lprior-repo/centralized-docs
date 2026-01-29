---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/response-streaming-http.html
title: Set up an HTTP proxy integration with payload response streaming in API Gateway
word_count: 848
filtered: true
elements_removed: 0
density_score: 0.88
---

Set up an HTTP proxy integration with payload response streaming in API Gateway - Amazon API Gateway
Set up an HTTP proxy integration with payload response streaming in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#response-streaming-http)
[Create an HTTP proxy integration with payload response streaming](#response-streaming-http-create)[Update the response transfer mode for an HTTP proxy integration](#response-streaming-http-update)
# Set up an HTTP proxy integration with payload response streaming in API Gateway
When you set up response payload streaming, you specify the response transfer mode in the integration request
of your method. You configure these settings in the integration request to control how API Gateway behaves before and
during the integration response. When you use response streaming, you can configure the integration timeout up to
15 minutes.
When you use payload response streaming with an `HTTP\_PROXY` integration, API Gateway won't send the HTTP
response status code or any HTTP response headers until it fully receives all headers.
## Create an HTTP proxy integration with payload response streaming
The following procedure shows you how to import a new API with the `responseTransferMode` set
to `STREAM`. If you have an existing integration API and want to modify the `responseTransferMode`,
see [Update the response transfer mode for an HTTP proxy integration](#response-streaming-http-update).
AWS Management Console
###### To create an HTTP proxy integration with payload response streaming
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Choose **Create resource**.
4. For **Resource name**, enter `streaming`.
5. Choose **Create resource**.
6. With the **/streaming** resource selected, choose **Create method**.
7. For **Method type**, choose **ANY**.
8. For **Integration type**, choose **HTTP**.
9. Choose **HTTP proxy integration**.
10. For **Response transfer mode**, choose **Stream**.
11. For **HTTP method**, choose a method.
12. For **Endpoint URL**, enter an integration endpoint. Make sure that you choose an
endpoint that produces a large payload to be streamed back to you.
13. Choose **Create method**.
After you create your method, deploy your API.
###### To deploy your API
1. Choose **Deploy API**.
2. For **Stage**, select **New stage**.
3. For **Stage name**, enter `prod`.
4. (Optional) For **Description**, enter a description.
5. Choose **Deploy**.
AWS CLI
###### To create a new API with payload response streaming
1. Copy the following Open API file, and then save it as `ResponseStreamDemoSwagger.yaml`. In
this file, `responseTransferMode` is set to
`STREAM`. The integration endpoint is set to `https://example.com`, but we
recommend that you modify it to an endpoint that produces a large payload to be streamed back to you.
```
`openapi: "3.0.1"
info:
title: "ResponseStreamingDemo"
version: "2025-04-28T17:28:25Z"
servers:
- url: "{basePath}"
variables:
basePath:
default: "prod"
paths:
/streaming:
get:
x-amazon-apigateway-integration:
httpMethod: "GET"
uri: "https://example.com"
type: "http\_proxy"
timeoutInMillis: 900000
responseTransferMode: "STREAM"`
```
2. Use the following `import-rest-api` command to import your OpenAPI definition:
```
`aws apigateway import-rest-api \\
--body 'fileb://\~/ResponseStreamDemoSwagger.yaml' \\
--parameters endpointConfigurationTypes=REGIONAL \\
--region us-west-1`
```
3. Use the following `create-deployment` command to deploy your new API to a stage:
```
`aws apigateway create-deployment \\
--rest-api-id `a1b2c3` \\
--stage-name prod \\
--region us-west-1`
```
## Update the response transfer mode for an HTTP proxy integration
The following procedure shows how to update the response transfer mode for an HTTP proxy integration.
AWS Management Console
###### To update the response transfer mode for an HTTP proxy integration
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Choose a method.
4. On the **Integration request** tab, under **Integration request settings**, choose **Edit**.
5. For **Response transfer mode**, choose **Stream**.
6. Choose **Save**.
After you update your method, deploy your API.
###### To deploy your API
1. Choose **Deploy API**.
2. For **Stage**, select **New stage**.
3. For **Stage name**, enter `prod`.
4. (Optional) For **Description**, enter a description.
5. Choose **Deploy**.
AWS CLI
The following `update-integration` command updates the transfer mode of an integration from
`BUFFERED` to `STREAM`. For any existing APIs, the response transfer mode for all
integrations is set to `BUFFERED`.
```
`aws apigateway update-integration \\
--rest-api-id `a1b2c3` \\
--resource-id `aaa111` \\
--http-method GET \\
--patch-operations "op='replace',path='/responseTransferMode',value=STREAM" \\
--region us-west-1`
```
You need to redeploy your API for the changes to take effect. If you customized the integration timeout,
this timeout value is removed, as API Gateway streams your response for up to 5 minutes.
The following `update-integration` command updates the transfer mode of an integration from
`STREAM` to `BUFFERED`:
```
`aws apigateway update-integration \\
--rest-api-id `a1b2c3` \\
--resource-id `aaa111` \\
--http-method GET \\
--patch-operations "op='replace',path='/responseTransferMode',value=BUFFERED" \\
--region us-west-1`
```
You need to redeploy your API for the changes to take effect.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Stream the integration response for your proxy integrations
Set up a Lambda proxy integration with payload response streaming
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.