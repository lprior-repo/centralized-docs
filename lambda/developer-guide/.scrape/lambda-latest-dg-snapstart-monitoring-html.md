---
url: https://docs.aws.amazon.com/lambda/latest/dg/snapstart-monitoring.html
title: Monitoring for Lambda SnapStart
word_count: 728
filtered: true
elements_removed: 0
density_score: 0.90
---

Monitoring for Lambda SnapStart - AWS Lambda
Monitoring for Lambda SnapStart - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#snapstart-monitoring)
[CloudWatch logs](#snapstart-cloudwatch)[AWS X-Ray](#snapstart-xray)[Telemetry API](#snapstart-telemetry)[API Gateway and function URL metrics](#snapstart-metrics)
# Monitoring for Lambda SnapStart
You can monitor your Lambda SnapStart functions using Amazon CloudWatch, AWS X-Ray, and the [Accessing real-time telemetry data for extensions using the Telemetry API](./telemetry-api.html).
###### Note
The `AWS\_LAMBDA\_LOG\_GROUP\_NAME` and `AWS\_LAMBDA\_LOG\_STREAM\_NAME` [environment variables](./configuration-envvars.html#configuration-envvars-runtime) are not available in Lambda SnapStart functions.
## Understanding logging and billing behavior with SnapStart
There are a few differences with the [CloudWatch log stream](./monitoring-cloudwatchlogs.html) format
for SnapStart functions:
* Initialization logs – When a new execution environment is created, the `REPORT` doesn't include the `Init Duration` field. That's because Lambda initializes SnapStart functions when you create a version instead of during function invocation. For SnapStart functions, the `Init Duration` field is in the `INIT\_REPORT` record. This record shows duration details for the [Init phase](./lambda-runtime-environment.html#runtimes-lifecycle-ib), including the duration of any `beforeCheckpoint` [runtime hooks](./snapstart-runtime-hooks.html).
* Invocation logs – When a new execution environment is created, the `REPORT` includes the `Restore Duration` and `Billed Restore Duration` fields:
* `Restore Duration`: The time it takes for Lambda to restore a snapshot, load the runtime, and run any after-restore [runtime hooks](./snapstart-runtime-hooks.html). The process of restoring snapshots can include time spent on activities outside the MicroVM. This time is reported in `Restore Duration`.
* `Billed Restore Duration`: The time it takes for Lambda to load the runtime and run any after-restore [runtime hooks](./snapstart-runtime-hooks.html).
###### Note
As with all Lambda functions, duration charges apply to code that runs in the function handler. For SnapStart functions, duration charges also apply to initialization code that's declared outside of the handler, the time it takes for the runtime to load, and any code that runs in a [runtime hook](./snapstart-runtime-hooks.html).
The cold start duration is the sum of `Restore Duration` + `Duration`.
The following example is a Lambda Insights query that returns the latency percentiles for SnapStart
functions. For more information about Lambda Insights queries, see [Example workflow using queries to troubleshoot a function](./monitoring-insights.html#monitoring-insights-queries).
```
`filter @type = "REPORT"
| parse @log /\\d+:\\/aws\\/lambda\\/(?&lt;&lt;function&gt;&gt;.\*)/
| parse @message /Restore Duration: (?&lt;&lt;restoreDuration&gt;&gt;.\*?) ms/
| stats
count(\*) as invocations,
pct(@duration+coalesce(@initDuration,0)+coalesce(restoreDuration,0), 50) as p50,
pct(@duration+coalesce(@initDuration,0)+coalesce(restoreDuration,0), 90) as p90,
pct(@duration+coalesce(@initDuration,0)+coalesce(restoreDuration,0), 99) as p99,
pct(@duration+coalesce(@initDuration,0)+coalesce(restoreDuration,0), 99.9) as p99.9
group by function, (ispresent(@initDuration) or ispresent(restoreDuration)) as coldstart
| sort by coldstart desc`
```
## X-Ray active tracing for SnapStart
You can use [X-Ray](./services-xray.html) to trace requests to Lambda SnapStart functions. There are a few differences with the X-Ray subsegments for SnapStart functions:
* There is no `Initialization` subsegment for SnapStart functions.
* The `Restore` subsegment shows the time it takes for Lambda to restore a snapshot, load the runtime, and run any after-restore [ runtime hooks](./snapstart-runtime-hooks.html). The process of restoring snapshots can include time spent on activities outside the MicroVM. This time is reported in the `Restore` subsegment. You aren't charged for the time spent outside the microVM to restore a snapshot.
## Telemetry API events for SnapStart
Lambda sends the following SnapStart events to the [Telemetry API](./telemetry-api.html):
* [platform.restoreStart](./telemetry-schema-reference.html#platform-restoreStart) – Shows the time when the [Restore phase](./lambda-runtime-environment.html#runtimes-lifecycle-restore) started.
* [platform.restoreRuntimeDone](./telemetry-schema-reference.html#platform-restoreRuntimeDone) – Shows whether the `Restore` phase
was successful. Lambda sends this message when the runtime sends a `restore/next` runtime API
request. There are three possible statuses: success, failure, and timeout.
* [platform.restoreReport](./telemetry-schema-reference.html#platform-restoreReport) – Shows how long the `Restore` phase lasted
and how many milliseconds you were billed for during this phase.
## Amazon API Gateway and function URL metrics
If you create a web API [using API Gateway](./services-apigateway.html), then you can use the [IntegrationLatency](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-metrics-and-dimensions.html) metric to measure end-to-end latency (the time between when API Gateway relays a request
to the backend and when it receives a response from the backend).
If you're using a [Lambda function URL](./urls-configuration.html), then you can use the [UrlRequestLatency](./urls-monitoring.html) metric to measure end-to-end latency (the time between when
the function URL receives a request and when the function URL returns a response).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
.NET
Security model
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.