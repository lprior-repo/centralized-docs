---
url: https://docs.aws.amazon.com/lambda/latest/dg/configuration-response-streaming.html
title: Response streaming for Lambda functions
word_count: 716
filtered: true
elements_removed: 0
density_score: 0.87
---

Response streaming for Lambda functions - AWS Lambda
Response streaming for Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#configuration-response-streaming)
[Bandwidth limits for response streaming](#config-rs-bandwidth-cap)[VPC compatibility with response streaming](#config-rs-vpc-compatibility)
# Response streaming for Lambda functions
Lambda functions can natively stream response payloads back to clients through [Lambda function URLs](./urls-configuration.html) or by using the [InvokeWithResponseStream](https://docs.aws.amazon.com/lambda/latest/api/API_InvokeWithResponseStream.html) API (via the AWS SDK or direct API calls). Your Lambda function can also stream response payloads through the [Amazon API Gateway proxy integration](https://docs.aws.amazon.com/apigateway/latest/developerguide/response-transfer-mode-lambda.html), which uses the [InvokeWithResponseStream](https://docs.aws.amazon.com/lambda/latest/api/API_InvokeWithResponseStream.html) API to invoke your function. Response
streaming can benefit latency sensitive applications by improving time to first byte (TTFB)
performance. This is because you can send partial responses back to the client as they become
available. Additionally, response streaming functions can return payloads up to 200 MB, compared to the 6 MB maximum for
buffered responses. Streaming a response also means that your function doesn’t need to fit the entire
response in memory. For very large responses, this can reduce the amount of memory you need to
configure for your function.
###### Note
Lambda response streaming is not yet available in all AWS Regions. Please refer to Builder Center's [AWS Capabilities by Region](https://builder.aws.com/build/capabilities) for feature availability by Region.
The speed at which Lambda streams your responses depends on the response size. The streaming rate for
the first 6 MB of your function’s response is uncapped. For responses larger than 6 MB, the remainder of the response
is subject to a bandwidth cap. For more information on streaming bandwidth, see [Bandwidth limits for response streaming](#config-rs-bandwidth-cap).
Streaming responses incur cost and streamed responses are not interrupted or stopped when the invoking client connection is broken. Customers will be billed for the full function duration, so customers should exercise caution when configuring long function timeouts.
Lambda supports response streaming on Node.js managed runtimes. For other languages, including Python, you can [use a
custom runtime with a custom Runtime API integration](./runtimes-custom.html#runtimes-custom-response-streaming) to stream responses or use the [Lambda Web Adapter](https://github.com/awslabs/aws-lambda-web-adapter).
###### Note
When testing your function through the Lambda console, you'll always see responses as buffered.
###### Topics
* [Bandwidth limits for response streaming](#config-rs-bandwidth-cap)
* [VPC compatibility with response streaming](#config-rs-vpc-compatibility)
* [Writing response streaming-enabled Lambda functions](./config-rs-write-functions.html)
* [Invoking a response streaming enabled function using Lambda function URLs](./config-rs-invoke-furls.html)
* [Tutorial: Creating a response streaming Lambda
function with a function URL](./response-streaming-tutorial.html)
## Bandwidth limits for response streaming
The first 6 MB of your function’s response payload has uncapped bandwidth. After this initial burst, Lambda streams your response at a
maximum rate of 2 MBps. If your function responses never exceed 6 MB, then this bandwidth limit never applies.
###### Note
Bandwidth limits only apply to your function’s response payload, and not to network access by your function.
The rate of uncapped bandwidth varies depending on a number of factors, including your function’s processing speed. You can normally
expect a rate higher than 2 MBps for the first 6 MB of your function’s response. If your function is streaming a response to a destination
outside of AWS, the streaming rate also depends on the speed of the external internet connection.
## VPC compatibility with response streaming
When using Lambda functions in a VPC environment, there are important considerations for response streaming:
* Lambda function URLs do not support response streaming within a VPC environment.
* You can use response streaming within a VPC by invoking your Lambda function through the AWS SDK using the `InvokeWithResponseStream` API. This requires setting up the appropriate VPC endpoints for Lambda.
* For VPC environments, you'll need to create an interface VPC endpoint for Lambda to enable communication between your resources in the VPC and the Lambda service.
A typical architecture for response streaming in a VPC might include:
```
`Client in VPC -&gt; Interface VPC endpoint for Lambda -&gt; Lambda function -&gt; Response streaming back through the same path`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tags
Writing functions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.