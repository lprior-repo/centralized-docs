---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-using-xray-maps.html
title: Use AWS X-Ray service maps and trace views
word_count: 523
filtered: true
elements_removed: 0
density_score: 0.81
---

Use AWS X-Ray service maps and trace views with API Gateway - Amazon API Gateway
Use AWS X-Ray service maps and trace views with API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-using-xray-maps)
[Example X-Ray service map](#apigateway-using-xray-maps-active)[Example X-Ray trace
view](#apigateway-using-xray-trace-view-active)
# Use AWS X-Ray service maps and trace views
with API Gateway
In this section you can find detailed information on how to use [AWS X-Ray](https://docs.aws.amazon.com/xray/latest/devguide/xray-services-apigateway.html) service maps and
trace views with API Gateway.
###### Topics
* [Example X-Ray service map](#apigateway-using-xray-maps-active)
* [Example X-Ray trace
view](#apigateway-using-xray-trace-view-active)
## Example X-Ray service map
AWS X-Ray service maps show information about your API and all of its downstream
services. When X-Ray is enabled for an API stage in API Gateway, you'll see a node in the
service map containing information about the overall time spent in the API Gateway service.
You can get detailed information about the response status and a histogram of the API
response time for the selected timeframe. For APIs integrating with AWS services such as
AWS Lambda and Amazon DynamoDB, you will see more nodes providing performance metrics related
to those services. There will be a service map for each API stage.
The following example shows a service map for the `test` stage of an API called `xray`.
This API has two Lambda integrations. The nodes represent the API Gateway service and the two Lambda functions.
For a detailed explanation of service map structure, see
[Use
the X-Ray trace map](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray-interface-console.html#xray-console-servicemap).
![Service map example of an API Gateway API stage](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/apigateway-xray-servicemap-2.png)
From the service map, you can zoom in to see a trace view of your API stage. The trace
will display in-depth information regarding your API, represented as segments and
subsegments. For example, the trace for the service map shown above would include
segments for the Lambda service and Lambda function. For more information, see [AWS Lambda and AWS X-Ray](https://docs.aws.amazon.com/xray/latest/devguide/xray-services-lambda.html).
If you choose a node or edge on an X-Ray service map, the X-Ray console shows a
latency distribution histogram. You can use a latency histogram to see how long it takes
for a service to complete its requests. Following is a histogram of the API Gateway stage
named `xray/test` in the previous service map. For a detailed explanation of
latency distribution histograms, see
[Use
Latency Histograms](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray-interface-console.html#xray-console-histograms).
![X-Ray histogram of an API Gateway API stage](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/apigateway-xray-histogram-1.png)
## Example X-Ray trace
view
The following diagram shows a trace view generated for the example API described
above, with a Lambda backend function. A successful API
method request is shown with a response code of 200.
For a detailed explanation of trace views, see
[View traces and trace details](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray-interface-console.html#xray-console-traces).
![API Gateway with active tracing enabled](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/apigateway-xray-traceview-1.png)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up AWS X-Ray
Configure AWS X-Ray sampling rules
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.