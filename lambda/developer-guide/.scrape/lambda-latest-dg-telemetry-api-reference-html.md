---
url: https://docs.aws.amazon.com/lambda/latest/dg/telemetry-api-reference.html
title: Lambda Telemetry API reference
word_count: 584
filtered: true
elements_removed: 0
density_score: 0.88
---

Lambda Telemetry API reference - AWS Lambda
Lambda Telemetry API reference - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#telemetry-api-reference)
[Subscribe](#telemetry-subscribe-api)
# Lambda Telemetry API reference
Use the Lambda Telemetry API endpoint to subscribe extensions to telemetry streams. You can retrieve the
Telemetry API endpoint from the `AWS\_LAMBDA\_RUNTIME\_API` environment variable. To send an API request,
append the API version (`2022-07-01/`) and `telemetry/`. For example:
```
`http://${AWS\_LAMBDA\_RUNTIME\_API}/2022-07-01/telemetry/`
```
For the OpenAPI Specification (OAS) definition of the subscription responses version
`2025-01-29`, see the following:
* **HTTP** –
[telemetry-api-http-schema.zip](samples/events_http_schema_v2025_01_29.zip)
* **TCP** –
[telemetry-api-tcp-schema.zip](samples/events_tcp_schema_v2025_01_29.zip)
## Subscribe
To subscribe to a telemetry stream, a Lambda extension can send a Subscribe API request.
* **Path** – `/telemetry`
* **Method** – `PUT`
* **Headers**
* `Content-Type`: `application/json`
* **Request body parameters**
* **schemaVersion**
* Required: Yes
* Type: String
* Valid values: `"2025-01-29"`, `"2022-12-13"`, or `"2022-07-01"`
* **Note:** Lambda Managed Instances require `"2025-01-29"`. This version is backward compatible with Lambda (default) functions.
* **destination** – The configuration settings that define the
telemetry event destination and the protocol for event delivery.
* Required: Yes
* Type: Object
```
`{
"protocol": "HTTP",
"URI": "http://sandbox.localdomain:8080"
}`
```
* **protocol** – The protocol that Lambda uses
to send telemetry data.
* Required: Yes
* Type: String
* Valid values: `"HTTP"`|`"TCP"`
* **URI** – The URI to send telemetry data to.
* Required: Yes
* Type: String
* For more information, see [Specifying a destination protocol](./telemetry-api.html#telemetry-api-destination).
* **types** – The types of telemetry that you want the extension to
subscribe to.
* Required: Yes
* Type: Array of strings
* Valid values: `"platform"`|`"function"`|`"extension"`
* **buffering** – The configuration settings for event
buffering.
* Required: No
* Type: Object
```
`{
"buffering": {
"maxItems": 1000,
"maxBytes": 256\*1024,
"timeoutMs": 100
}
}`
```
* **maxItems** – The maximum number of events to buffer in memory.
* Required: No
* Type: Integer
* Default: 1,000
* Minimum: 1,000
* Maximum: 10,000
* **maxBytes** – The maximum volume of telemetry (in bytes) to
buffer in memory.
* Required: No
* Type: Integer
* Default: 262,144
* Minimum: 262,144
* Maximum: 1,048,576
* **timeoutMs** – The maximum time (in milliseconds) to buffer
a batch.
* Required: No
* Type: Integer
* Default: 1,000
* Minimum: 25
* Maximum: 30,000
* For more information, see [Configuring memory usage and buffering](./telemetry-api.html#telemetry-api-buffering).
### Example Subscribe API request
```
`PUT http://${AWS\_LAMBDA\_RUNTIME\_API}/2022-07-01/telemetry HTTP/1.1
{
"schemaVersion": "2025-01-29",
"types": [
"platform",
"function",
"extension"
],
"buffering": {
"maxItems": 1000,
"maxBytes": 256\*1024,
"timeoutMs": 100
},
"destination": {
"protocol": "HTTP",
"URI": "http://sandbox.localdomain:8080"
}
}`
```
If the Subscribe request succeeds, the extension receives an HTTP 200 success response:
```
`HTTP/1.1 200 OK
"OK"`
```
If the Subscribe request fails, the extension receives an error response. For example:
```
`HTTP/1.1 400 OK
{
"errorType": "ValidationError",
"errorMessage": "URI port is not provided; types should not be empty"
}`
```
Here are some additional response codes that the extension can receive:
* 200 – Request completed successfully
* 202 – Request accepted. Subscription request response in local testing environment
* 400 – Bad request
* 500 – Service error
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Telemetry API
Event schema reference
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.