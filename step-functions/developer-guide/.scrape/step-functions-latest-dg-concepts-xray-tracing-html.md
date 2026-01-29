---
url: https://docs.aws.amazon.com/step-functions/latest/dg/concepts-xray-tracing.html
title: Trace Step Functions request data in AWS X-Ray
word_count: 2665
filtered: true
elements_removed: 0
density_score: 0.81
---

Trace Step Functions request data in AWS X-Ray - AWS Step Functions
Trace Step Functions request data in AWS X-Ray - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#concepts-xray-tracing)
[Setup and configuration](#xray-concept-create)[Concepts](#xray-concepts)[Service integrations](#xray-concept-integrations)[Viewing the X-Ray console](#xray-concept-tracing-details)[Viewing X-Ray tracing information for
Step Functions](#concepts-xray-tracing-events)[Traces](#concepts-xray-traces)[Service map](#concepts-xray-tracing-overview)[Segments and subsegments](#concepts-xray-tracing-segments)[Analytics](#concepts-xray-tracing-analytics)[Configuration](#concepts-xray-tracing-config)[What if there is no data in the trace map or
service map?](#concepts-xray-troubleshooting)
# Trace Step Functions request data in AWS X-Ray
You can use [AWS X-Ray](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray.html) to visualize the components of your state machine, identify performance
bottlenecks, and troubleshoot requests that resulted in an error. Your state machine sends trace
data to X-Ray, and X-Ray processes the data to generate a service map and searchable trace
summaries.
With X-Ray enabled for your state machine, you can trace requests as they are executed in
Step Functions, in all AWS Regions where X-Ray is available. This gives you a detailed overview of an
entire Step Functions request. Step Functions will send traces to X-Ray for state machine executions, even when a
trace ID is not passed by an upstream service. You can use an X-Ray service map to view the
latency of a request, including any AWS services that are integrated with X-Ray. You can also
configure sampling rules to tell X-Ray which requests to record, and at what sampling rates,
according to criteria that you specify.
When X-Ray is not enabled for your state machine, and an upstream service does not pass a
trace ID, Step Functions will not send traces to X-Ray for state machine executions. However, if a trace
ID is passed by an upstream service, Step Functions will then send traces to X-Ray for state machine
executions.
You can use AWS X-Ray with Step Functions in regions where both are supported. See the [Step Functions](https://docs.aws.amazon.com/general/latest/gr/step-functions.html) and [X-Ray](https://docs.aws.amazon.com/general/latest/gr/xray.html) endpoints and quotas pages for information on region support for X-Ray and
Step Functions.
###### X-Ray and Step Functions Combined Quotas
You can add data to a trace for up to seven days, and query trace data going back thirty
days, the length of time that X-Ray stores trace data. Your traces will be subject to X-Ray
quotas. In addition to other quotas, X-Ray provides a minimum guaranteed trace size of 100 KiB
for Step Functions state machines. If more than 100 KiB of trace data is provided to X-Ray, this may
result in a frozen trace. See the service quotas section of the [X-Ray endpoints and quotas](https://docs.aws.amazon.com/general/latest/gr/xray.html#limits_xray) page for more
information on other quotas for X-Ray.
###### Important
Step Functions doesn't support X-Ray tracing for the child workflow executions started by a [Distributed Map state](./state-map-distributed.html) because it's easy to exceed the [Trace document size limit](https://docs.aws.amazon.com/general/latest/gr/xray.html#limits_xray) for such executions.
###### Topics
* [Setup and configuration](#xray-concept-create)
* [Concepts](#xray-concepts)
* [Service integrations](#xray-concept-integrations)
* [Viewing the X-Ray console](#xray-concept-tracing-details)
* [Viewing X-Ray tracing information for
Step Functions](#concepts-xray-tracing-events)
* [Traces](#concepts-xray-traces)
* [Service map](#concepts-xray-tracing-overview)
* [Segments and subsegments](#concepts-xray-tracing-segments)
* [Analytics](#concepts-xray-tracing-analytics)
* [Configuration](#concepts-xray-tracing-config)
* [What if there is no data in the trace map or
service map?](#concepts-xray-troubleshooting)
### Enable X-Ray tracing when creating a state
machine
You can enable X-Ray tracing when creating a new state machine by selecting
**Enable X-Ray tracing** on the **Specify details**
page.
1. Open the [Step Functions console](https://console.aws.amazon.com/states/home) and choose **Create state
machine**.
2. On the **Choose authoring method** page, choose an appropriate option to create your state
machine. If you choose **Run a sample project**, you cannot enable X-Ray tracing during the
state machine creation, and you will need to enable X-Ray tracing after your state machine has been created.
For more information about enabling X-Ray in an existing state machine, see
[Enable X-Ray in an existing state
machine](#xray-concept-enable-existing).
Choose **Next**.
3. On the **Specify details** page, configure your state
machine.
4. Choose **Enable X-Ray tracing**.
Your Step Functions state machine will now send traces to X-Ray for state machine
executions.
###### Note
If you choose to use an existing IAM role, you should ensure that X-Ray writes are allowed. For more information about the permissions that you need, see the following topic.
### IAM policies using AWS X-Ray in Step Functions
To enable X-Ray tracing, you will need an IAM policy with suitable permissions to
allow tracing. If your state machine uses other integrated services, you may need
additional IAM policies. See the IAM policies for your specific service
integrations.
If you enable X-Ray tracing for an existing state machine you must ensure that
you add a policy with sufficient permissions to enable X-Ray traces.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"xray:PutTraceSegments",
"xray:PutTelemetryRecords",
"xray:GetSamplingRules",
"xray:GetSamplingTargets"
],
"Resource": [
"\*"
]
}
]
}`
`
```
### Enable X-Ray in an existing state
machine
To enable X-Ray in an existing state machine:
1. In the [Step Functions console](https://console.aws.amazon.com/states/home), select the state machine for which
you want to enable tracing.
2. Choose **Edit**.
3. Choose **Enable X-Ray tracing**.
You will see a notification telling you that you that you may need to make
additional changes.
###### Note
When you enable X-Ray for an existing state machine, you must ensure that you
have an IAM policy that grants sufficient permissions for X-Ray to perform traces.
You can either add one manually, or generate one. For more information, see the IAM
policy section for [IAM policies using AWS X-Ray in Step Functions](#xray-iam).
4. (Optional) Auto-generate a new role for your state machine to include X-Ray
permissions.
5. Choose **Save**.
### Configure X-Ray tracing for Step Functions
When you first run a state machine with X-Ray tracing enabled, it will use the default
configuration values for X-Ray tracing. AWS X-Ray does not collect data for every request
that is sent to an application. Instead, it collects data for a statistically significant
number of requests. The default is to record the first request each second, and five percent
of any additional requests. One request per second is the *reservoir*.
This ensures that at least one trace is recorded each second as long as the service is
serving requests. Five percent is the *rate* at which additional requests
beyond the reservoir size are sampled.
To avoid incurring service charges when you are getting started, the default sampling
rate is conservative. You can configure X-Ray to modify the default sampling rule and
configure additional rules that apply sampling based on properties of the service or
request.
For example, you might want to disable sampling and trace all requests for calls that
modify state or handle AWS accounts or transactions. For high-volume read-only calls, like
background polling, health checks, or connection maintenance, you can sample at a low rate
and still get enough data to observe issues that occur.
To configure a sampling rule for your state machine:
1. Go to the [X-Ray console](https://console.aws.amazon.com/xray/home).
2. Choose **Sampling**.
3. To create a rule, choose **Create sampling rule**.
To edit a rule, choose a rule's name.
To delete a rule, choose a rule and use the **Actions** menu to
delete it.
Some parts of existing sampling rules, such as the name and priority, cannot be changed.
Instead, add or clone an existing rule, make the changes you want, then use the new
rule.
For detailed information on X-Ray sampling rules and how to configure the various
parameters, see [Configuring sampling
rules in the X-Ray console](https://docs.aws.amazon.com/xray/latest/devguide/xray-console-sampling.html).
### Integrate upstream services
To integrate the execution of Step Functions workflows, such as Express, Synchronous, and Standard workflows, with an upstream service you need to set the `traceHeader`. This is
automatically done for you if you are using a HTTP API in API Gateway. However, if you're using a Lambda function and/or an SDK, you need to set the `traceHeader` on the
[`StartExecution`](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html) or
[`StartSyncExecution`](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartSyncExecution.html) API calls yourself.
You must specify the `traceHeader` format as `\\p{ASCII}∗`. Additionally, to let Step Functions use the same trace ID, you must specify the format as
`Root={TRACE\_ID};Sampled={1 or 0}`. If you're using a Lambda function, replace the `TRACE\_ID` with the trace ID in your current segment and set the Sampled
field as `1` if your sampling mode is true and `0` if your sampling mode is false. Providing the trace ID in this format ensures that you'll get a complete
trace.
The following is an example written in Python to showcase how to specify the `traceHeader`.
```
`state\_machine = config.get\_string\_paramter("STATE\_MACHINE\_ARN")
if (xray\_recorder.current\_subsegment() is not None and
xray\_recorder.current\_subsegment().sampled) :
trace\_id = "Root={};Sampled=1".format(
xray\_recorder.current\_subsegment().trace\_id
)
else:
trace\_id = "Root=not enabled;Sampled=0"
LOGGER.info("trace %s", trace\_id)
# execute it
response = states.start\_sync\_execution(
stateMachineArn=state\_machine,
input=event['body'],
name=context.aws\_request\_id,
traceHeader=trace\_id
)
LOGGER.info(response)`
```
### The X-Ray console
In the AWS X-Ray console, you can view service maps and traces for requests that
your applications serve when X-Ray is enabled for your state machine.
See [Viewing the X-Ray console](#xray-concept-tracing-details) for information on how to access the X-Ray
console for your state machine executions.
For detailed information about the X-Ray console, see the [X-Ray console documentation](https://docs.aws.amazon.com/xray/latest/devguide/xray-console.html).
### Segments, subsegments, and traces
A **segment** records information about a request to your state
machine. It contains information such as the work that your state machine performs, and may
also contain **subsegments** with information about downstream
calls.
A **trace** collects all the segments generated by a single
request.
### Sampling
To ensure efficient tracing and provide a representative sample of the requests that
your application serves, X-Ray applies a **sampling** algorithm to
determine which requests get traced. This can be changed by editing the sampling
rules.
### Metrics
For your state machine, X-Ray will meter invocation time, state transition time, the
overall execution time of Step Functions, and variances in this execution time. This information can
be accessed through the X-Ray console.
### Analytics
The AWS X-Ray Analytics console is an interactive tool for interpreting trace data. You
can refine the active dataset with increasingly granular filters by clicking the graphs and
the panels of metrics and fields that are associated with the current trace set.
You can analyze how your state machine is performing to locate and identify
performance issues.
For detailed information about X-Ray analytics, see [Interacting with the AWS X-Ray Analytics
console](https://docs.aws.amazon.com/xray/latest/devguide/xray-console-analytics.html)
## Step Functions service integrations and X-Ray
Some of the AWS services that integrate with Step Functions provide integration with AWS X-Ray by
adding a tracing header to requests, running the X-Ray daemon, or making sampling decisions
and uploading trace data to X-Ray. Others must be instrumented using the AWS X-Ray SDK. A
few do not yet support X-Ray integration. X-Ray integration is necessary to provide complete
trace data when using a service integration with Step Functions
### Native X-Ray support
Service integrations with native X-Ray support include:
* [Amazon Simple Notification Service](https://docs.aws.amazon.com/xray/latest/devguide/xray-services-sns.html)
* [Amazon Simple Queue Service](https://docs.aws.amazon.com/xray/latest/devguide/xray-services-sqs.html)
* [AWS Lambda](https://docs.aws.amazon.com/xray/latest/devguide/xray-services-lambda.html)
* AWS Step Functions
### Instrumentation required
Service integrations that require [X-Ray
instrumentation](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray.html):
* Amazon Elastic Container Service
* AWS Batch
* AWS Fargate
### Client-side trace only
Other service integrations do not support X-Ray traces. However, client side traces can
still be collected:
* Amazon DynamoDB
* Amazon EMR
* Amazon SageMaker AI
* AWS CodeBuild
* AWS Glue
## Viewing the X-Ray console
X-Ray receives data from services as segments. X-Ray groups segments that have a common
request into traces. X-Ray processes the traces to generate a service graph that provides a
visual representation of your application.
After you start your state machine's execution, you can view its X-Ray traces by choosing
the **X-Ray trace map** link in the **Execution details**
section.
After you have enabled X-Ray for your state machine, you can view tracing information for
its executions in the X-Ray console.
## Viewing X-Ray tracing information for
Step Functions
The following steps illustrate what kind of information you can see in the console after
you enable X-Ray and run an execution. X-Ray traces for the [Create a callback pattern example with Amazon SQS, Amazon SNS, and Lambda](./callback-task-sample-sqs.html) sample project
are shown.
## Traces
After an execution has finished, you can navigate to the X-Ray console, where you
will see the X-Ray **Traces** page. This displays an overview of the service
map as well as trace and segment information for your state machine.
![Illustrative screenshot of X-Ray traces for a state machine.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/xray-tracing-overview.png)
## Service map
The service map in the X-Ray console helps you to identify services where errors are
occurring, where there are connections with high latency, or see traces for requests that were
unsuccessful.
![Illustrative screenshot focused on the service map in X-Ray traces.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/xray-tracing-servicemap.png)
On the trace map, you can choose a service node to view requests for that node, or an edge
between two nodes to view requests that traveled that connection. Here, the
`WaitForCallBack` node has been selected, and you can view additional information
about its execution and response status.
![Example details for an X-Ray trace node.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/xray-tracing-servicemap-detail.png)
You can see how the X-Ray service map correlates to the state machine. There is a service
map node for each service integration that is called by Step Functions, provided it supports X-Ray.
![Example graphical representation of the state machine being traced.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/sample-callback-example.png)
## Segments and subsegments
A **trace** is a collection of **segments** generated by
a single request. Each segment provides the resource's name, details about the request, and
details about the work done. On the **Traces** page, you can see the segments
and, if expanded, its corresponding subsegments. You can choose a segment or subsegment to
view detailed information about it.
You will be a different segment for each node on the service map.
![Example screenshot of segments and subsegments for a state machine.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/xray-tracing-segments.png)
Choosing a segment provides the resource's name, details about the request, and
details about the work done.
A segment can break down the data about the work done into subsegments. Choosing a
subsegment shows granular timing information and details. A
subsegment can contain additional details about a call to an AWS service,
an external HTTP API, or an SQL database.
## Analytics
The AWS X-Ray **Analytics** console is an interactive tool for
interpreting trace data. You can use this to more easily understand how your state machine is
performing. You can explore, analyze, and visualize traces through
interactive response time and time-series graphs to help locate performance
and latency issues.
You can refine the active dataset with increasingly granular filters by clicking the
graphs and the panels of metrics and fields that are associated with the current trace
set.
## Configuration
You can configure sampling and encryption options from the X-Ray console.
*
Choose **Sampling** to view details about the
sampling rate and configuration.
You can change the sampling rules to control the amount of data
that you record, and modify sampling behavior to suit your specific
requirements.
* Choose **Encryption** to modify the encryption
settings.
You can use the default setting, where X-Ray encrypts traces and
data at rest, or, if needed, you can choose a KMS key. Standard [AWS KMS](https://docs.aws.amazon.com/kms/latest/developerguide/) charges
apply in the latter case.
## What if there is no data in the trace map or
service map?
If you have enabled X-Ray, but can't see any data in the X-Ray console, check
that:
* Your IAM roles are set up correctly to allow writing to X-Ray.
* Sampling rules allow sampling of data.
* Since there can be a short delay before newly created or modified IAM roles are
applied, check the trace or service maps again after a few minutes.
* If you see **Data Not Found** in the X-Ray Traces panel, check your
[ IAM account
settings](https://console.aws.amazon.com/iam/home?#/account_settings) and ensure that AWS Security Token Service is enabled for the intended region. For more
information, see [Activating and deactivating AWS STS in an AWS Region](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html#sts-regions-activate-deactivate) in the *IAM User Guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Logging in CloudWatch Logs
Events using User Notifications
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.