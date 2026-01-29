---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-xray.html
title: Trace user requests to REST APIs using X-Ray in API Gateway
word_count: 354
filtered: true
elements_removed: 0
density_score: 0.82
---

Trace user requests to REST APIs using X-Ray in API Gateway - Amazon API Gateway
Trace user requests to REST APIs using X-Ray in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-xray)
# Trace user requests to REST APIs using X-Ray in API Gateway
You can use [AWS X-Ray](https://docs.aws.amazon.com/xray/latest/devguide/xray-services-apigateway.html) to
trace and analyze user requests as they travel through your Amazon API Gateway REST APIs to the underlying
services. API Gateway supports X-Ray tracing for all API Gateway REST API endpoint types: Regional,
edge-optimized, and private. You can use X-Ray with Amazon API Gateway in all AWS Regions where
X-Ray is available.
Because X-Ray gives you an end-to-end view of an entire request, you can analyze
latencies in your APIs and their backend services. You can use an X-Ray service map to view
the latency of an entire request and that of the downstream services that are integrated
with X-Ray. You can also configure sampling rules to tell X-Ray which requests to record
and at what sampling rates, according to criteria that you specify.
If you call an API Gateway API from a service that's already being traced, API Gateway passes the
trace through, even if X-Ray tracing isn't enabled on the API.
You can enable X-Ray for an API stage by using the API Gateway console, or by using the API Gateway
API or CLI.
###### Topics
* [Set up AWS X-Ray with API Gateway REST APIs](./apigateway-enabling-xray.html)
* [Use AWS X-Ray service maps and trace views
with API Gateway](./apigateway-using-xray-maps.html)
* [Configure AWS X-Ray sampling
rules for API Gateway APIs](./apigateway-configuring-xray-sampling-rules.html)
* [AWS X-Ray traces for
Amazon API Gateway APIs](./apigateway-understanding-xray-traces.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Variables for access logging for API Gateway
Set up AWS X-Ray
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.