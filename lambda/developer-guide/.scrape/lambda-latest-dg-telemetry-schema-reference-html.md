---
url: https://docs.aws.amazon.com/lambda/latest/dg/telemetry-schema-reference.html
title: Lambda Telemetry API `Event` schema reference
word_count: 2913
filtered: true
elements_removed: 0
density_score: 0.86
---

Lambda Telemetry API Event schema reference - AWS Lambda
Lambda Telemetry API Event schema reference - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#telemetry-schema-reference)
[Telemetry API Event object types](#telemetry-api-events)[Shared object types](#telemetry-api-objects)
# Lambda Telemetry API `Event` schema reference
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
The following table is a summary of all the types of `Event` objects that the Telemetry API
supports.
|Category|Event type|Description|Event record schema|
|
Platform event
|
`platform.initStart`
|
Function initialization started.
|
[platform.initStart](#platform-initStart) schema
|
|
Platform event
|
`platform.initRuntimeDone`
|
Function initialization completed.
|
[platform.initRuntimeDone](#platform-initRuntimeDone) schema
|
|
Platform event
|
`platform.initReport`
|
A report of function initialization.
|
[platform.initReport](#platform-initReport) schema
|
|
Platform event
|
`platform.start`
|
Function invocation started.
|
[platform.start](#platform-start) schema
|
|
Platform event
|
`platform.runtimeDone`
|
The runtime finished processing an event with either success or failure.
|
[platform.runtimeDone](#platform-runtimeDone) schema
|
|
Platform event
|
`platform.report`
|
A report of function invocation.
|
[platform.report](#platform-report) schema
|
|
Platform event
|
`platform.restoreStart`
|
Runtime restore started.
|
[platform.restoreStart](#platform-restoreStart) schema
|
|
Platform event
|
`platform.restoreRuntimeDone`
|
Runtime restore completed.
|
[platform.restoreRuntimeDone](#platform-restoreRuntimeDone) schema
|
|
Platform event
|
`platform.restoreReport`
|
Report of runtime restore.
|
[platform.restoreReport](#platform-restoreReport) schema
|
|
Platform event
|
`platform.telemetrySubscription`
|
The extension subscribed to the Telemetry API.
|
[platform.telemetrySubscription](#platform-telemetrySubscription) schema
|
|
Platform event
|
`platform.logsDropped`
|
Lambda dropped log entries.
|
[platform.logsDropped](#platform-logsDropped) schema
|
|
Function logs
|
`function`
|
A log line from function code.
|
[function](#telemetry-api-function) schema
|
|
Extension logs
|
`extension`
|
A log line from extension code.
|
[extension](#telemetry-api-extension) schema
|
###### Contents
* [Telemetry API Event object types](./telemetry-schema-reference.html#telemetry-api-events)
* [platform.initStart](./telemetry-schema-reference.html#platform-initStart)
* [platform.initRuntimeDone](./telemetry-schema-reference.html#platform-initRuntimeDone)
* [platform.initReport](./telemetry-schema-reference.html#platform-initReport)
* [platform.start](./telemetry-schema-reference.html#platform-start)
* [platform.runtimeDone](./telemetry-schema-reference.html#platform-runtimeDone)
* [platform.report](./telemetry-schema-reference.html#platform-report)
* [platform.restoreStart](./telemetry-schema-reference.html#platform-restoreStart)
* [platform.restoreRuntimeDone](./telemetry-schema-reference.html#platform-restoreRuntimeDone)
* [platform.restoreReport](./telemetry-schema-reference.html#platform-restoreReport)
* [platform.extension](./telemetry-schema-reference.html#platform-extension)
* [platform.telemetrySubscription](./telemetry-schema-reference.html#platform-telemetrySubscription)
* [platform.logsDropped](./telemetry-schema-reference.html#platform-logsDropped)
* [function](./telemetry-schema-reference.html#telemetry-api-function)
* [extension](./telemetry-schema-reference.html#telemetry-api-extension)
* [Shared object types](./telemetry-schema-reference.html#telemetry-api-objects)
* [InitPhase](./telemetry-schema-reference.html#InitPhase)
* [InitReportMetrics](./telemetry-schema-reference.html#InitReportMetrics)
* [InitType](./telemetry-schema-reference.html#InitType)
* [ReportMetrics](./telemetry-schema-reference.html#ReportMetrics)
* [RestoreReportMetrics](./telemetry-schema-reference.html#RestoreReportMetrics)
* [RuntimeDoneMetrics](./telemetry-schema-reference.html#RuntimeDoneMetrics)
* [Span](./telemetry-schema-reference.html#Span)
* [Status](./telemetry-schema-reference.html#Status)
* [TraceContext](./telemetry-schema-reference.html#TraceContext)
* [TracingType](./telemetry-schema-reference.html#TracingType)
## Telemetry API `Event` object types
This section details the types of `Event` objects that the Lambda Telemetry API supports. In the
event descriptions, a question mark (`?`) indicates that the attribute may not be present in the
object.
### `platform.initStart`
A `platform.initStart` event indicates that the function initialization phase has started. A
`platform.initStart`
`Event` object has the following shape:
```
`Event: Object
- time: String
- type: String = platform.initStart
- record: PlatformInitStart
`
```
The `PlatformInitStart` object has the following attributes:
* **functionName** – `String`
* **functionVersion** – `String`
* **initializationType** – `[InitType](#InitType)`
object
* **instanceId?** – `String`
* **instanceMaxMemory?** – `Integer`
* **phase** – `[InitPhase](#InitPhase)` object
* **runtimeVersion?** – `String`
* **runtimeVersionArn?** – `String`
The following is an example `Event` of type `platform.initStart`:
```
`{
"time": "2022-10-12T00:00:15.064Z",
"type": "platform.initStart",
"record": {
"initializationType": "on-demand",
"phase": "init",
"runtimeVersion": "nodejs-14.v3",
"runtimeVersionArn": "arn",
"functionName": "myFunction",
"functionVersion": "$LATEST",
"instanceId": "82561ce0-53dd-47d1-90e0-c8f5e063e62e",
"instanceMaxMemory": 256
}
}`
```
### `platform.initRuntimeDone`
A `platform.initRuntimeDone` event indicates that the function initialization phase has
completed. A `platform.initRuntimeDone`
`Event` object has the following shape:
```
`Event: Object
- time: String
- type: String = platform.initRuntimeDone
- record: PlatformInitRuntimeDone`
```
The `PlatformInitRuntimeDone` object has the following attributes:
* **initializationType** – `[InitType](#InitType)`
object
* **phase** – `[InitPhase](#InitPhase)` object
* **status** – `[Status](#Status)` object
* **spans?** – List of `[Span](#Span)`
objects
The following is an example `Event` of type `platform.initRuntimeDone`:
```
`{
"time": "2022-10-12T00:01:15.000Z",
"type": "platform.initRuntimeDone",
"record": {
"initializationType": "on-demand"
"status": "success",
"spans": [
{
"name": "someTimeSpan",
"start": "2022-06-02T12:02:33.913Z",
"durationMs": 70.5
}
]
}
}`
```
### `platform.initReport`
A `platform.initReport` event contains an overall report of the function initialization phase. A
`platform.initReport`
`Event` object has the following shape:
```
`Event: Object
- time: String
- type: String = platform.initReport
- record: PlatformInitReport`
```
The `PlatformInitReport` object has the following attributes:
* **errorType?** – string
* **initializationType** – `[InitType](#InitType)`
object
* **phase** – `[InitPhase](#InitPhase)` object
* **metrics** – `[InitReportMetrics](#InitReportMetrics)`
object
* **spans?** – List of `[Span](#Span)`
objects
* **status** – `[Status](#Status)` object
The following is an example `Event` of type `platform.initReport`:
```
`{
"time": "2022-10-12T00:01:15.000Z",
"type": "platform.initReport",
"record": {
"initializationType": "on-demand",
"status": "success",
"phase": "init",
"metrics": {
"durationMs": 125.33
},
"spans": [
{
"name": "someTimeSpan",
"start": "2022-06-02T12:02:33.913Z",
"durationMs": 90.1
}
]
}
}`
```
### `platform.start`
A `platform.start` event indicates that the function invocation phase has started. A
`platform.start`
`Event` object has the following shape:
```
`Event: Object
- time: String
- type: String = platform.start
- record: PlatformStart`
```
The `PlatformStart` object has the following attributes:
* **requestId** – `String`
* **version?** – `String`
* **tracing?** – `[TraceContext](#TraceContext)`
The following is an example `Event` of type `platform.start`:
```
`{
"time": "2022-10-12T00:00:15.064Z",
"type": "platform.start",
"record": {
"requestId": "6d68ca91-49c9-448d-89b8-7ca3e6dc66aa",
"version": "$LATEST",
"tracing": {
"spanId": "54565fb41ac79632",
"type": "X-Amzn-Trace-Id",
"value": "Root=1-62e900b2-710d76f009d6e7785905449a;Parent=0efbd19962d95b05;Sampled=1"
}
}
}`
```
### `platform.runtimeDone`
A `platform.runtimeDone` event indicates that the function invocation phase has completed. A
`platform.runtimeDone`
`Event` object has the following shape:
###### Lambda Managed Instances
The `platform.runtimeDone` event is not supported for Lambda Managed Instances. Extensions
running on Managed Instances will not receive this event because extensions cannot subscribe to the
`INVOKE` event on Managed Instances. Due to the concurrent execution model where multiple
invocations can be processed simultaneously, extensions cannot perform post-invoke processing for individual
invocations as they traditionally do on Lambda (default) functions.
For Managed Instances, the `responseLatency` and `responseDuration` spans that
are normally included in `platform.runtimeDone` are instead available in the
`platform.report` event. See [platform.report](#platform-report) for details.
```
`Event: Object
- time: String
- type: String = platform.runtimeDone
- record: PlatformRuntimeDone`
```
The `PlatformRuntimeDone` object has the following attributes:
* **errorType?** – `String`
* **metrics?** – `[RuntimeDoneMetrics](#RuntimeDoneMetrics)`
object
* **requestId** – `String`
* **status** – `[Status](#Status)` object
* **spans?** – List of `[Span](#Span)`
objects
* **tracing?** – `[TraceContext](#TraceContext)`
object
The following is an example `Event` of type `platform.runtimeDone`:
```
`{
"time": "2022-10-12T00:01:15.000Z",
"type": "platform.runtimeDone",
"record": {
"requestId": "6d68ca91-49c9-448d-89b8-7ca3e6dc66aa",
"status": "success",
"tracing": {
"spanId": "54565fb41ac79632",
"type": "X-Amzn-Trace-Id",
"value": "Root=1-62e900b2-710d76f009d6e7785905449a;Parent=0efbd19962d95b05;Sampled=1"
},
"spans": [
{
"name": "someTimeSpan",
"start": "2022-08-02T12:01:23:521Z",
"durationMs": 80.0
}
],
"metrics": {
"durationMs": 140.0,
"producedBytes": 16
}
}
}`
```
### `platform.report`
A `platform.report` event contains an overall report of the function invoke phase. A
`platform.report`
`Event` object has the following shape:
###### Lambda Managed Instances
The `platform.report` event for Lambda Managed Instances has different metrics and spans compared
to Lambda (default) functions. For Managed Instances:
* **Spans**: Contains `responseLatency` and `responseDuration`
instead of `extensionOverhead`. The `extensionOverhead` span is not available because
extensions cannot subscribe to the `INVOKE` event on Managed Instances due to the concurrent
execution model.
* **Metrics**: Only includes `durationMs`. The following metrics are
not included: `billedDurationMs`, `initDurationMs`, `maxMemoryUsedMB`,
and `memorySizeMB`. These per-invoke metrics are not applicable in the concurrent execution
environment. For resource utilization metrics, use [Monitoring Lambda Managed Instances](./lambda-managed-instances-monitoring.html) or
[Lambda Insights](https://docs.aws.amazon.com/lambda/latest/dg/monitoring-insights.html).
```
`Event: Object
- time: String
- type: String = platform.report
- record: PlatformReport`
```
The `PlatformReport` object has the following attributes:
* **metrics** – `[ReportMetrics](#ReportMetrics)`
object
* **requestId** – `String`
* **spans?** – List of `[Span](#Span)`
objects
* **status** – `[Status](#Status)` object
* **tracing?** – `[TraceContext](#TraceContext)`
object
The following is an example `Event` of type `platform.report`:
```
`{
"time": "2022-10-12T00:01:15.000Z",
"type": "platform.report",
"record": {
"metrics": {
"billedDurationMs": 694,
"durationMs": 693.92,
"initDurationMs": 397.68,
"maxMemoryUsedMB": 84,
"memorySizeMB": 128
},
"requestId": "6d68ca91-49c9-448d-89b8-7ca3e6dc66aa",
}
}`
```
### `platform.restoreStart`
A `platform.restoreStart` event indicates that a function environment restoration event started.
In an environment restoration event, Lambda creates the environment from a cached snapshot rather than initializing it
from scratch. For more information, see [Lambda SnapStart](./snapstart.html). A
`platform.restoreStart`
`Event` object has the following shape:
```
`Event: Object
- time: String
- type: String = platform.restoreStart
- record: PlatformRestoreStart`
```
The `PlatformRestoreStart` object has the following attributes:
* **functionName** – `String`
* **functionVersion** – `String`
* **instanceId?** – `String`
* **instanceMaxMemory?** – `String`
* **runtimeVersion?** – `String`
* **runtimeVersionArn?** – `String`
The following is an example `Event` of type `platform.restoreStart`:
```
`{
"time": "2022-10-12T00:00:15.064Z",
"type": "platform.restoreStart",
"record": {
"runtimeVersion": "nodejs-14.v3",
"runtimeVersionArn": "arn",
"functionName": "myFunction",
"functionVersion": "$LATEST",
"instanceId": "82561ce0-53dd-47d1-90e0-c8f5e063e62e",
"instanceMaxMemory": 256
}
}`
```
### `platform.restoreRuntimeDone`
A `platform.restoreRuntimeDone` event indicates that a function environment restoration event completed.
In an environment restoration event, Lambda creates the environment from a cached snapshot rather than initializing it
from scratch. For more information, see [Lambda SnapStart](./snapstart.html). A
`platform.restoreRuntimeDone`
`Event` object has the following shape:
```
`Event: Object
- time: String
- type: String = platform.restoreRuntimeDone
- record: PlatformRestoreRuntimeDone`
```
The `PlatformRestoreRuntimeDone` object has the following attributes:
* **errorType?** – `String`
* **spans?** – List of `[Span](#Span)`
objects
* **status** – `[Status](#Status)` object
The following is an example `Event` of type `platform.restoreRuntimeDone`:
```
`{
"time": "2022-10-12T00:00:15.064Z",
"type": "platform.restoreRuntimeDone",
"record": {
"status": "success",
"spans": [
{
"name": "someTimeSpan",
"start": "2022-08-02T12:01:23:521Z",
"durationMs": 80.0
}
]
}
}`
```
### `platform.restoreReport`
A `platform.restoreReport` event contains an overall report of a function restoration event. A
`platform.restoreReport`
`Event` object has the following shape:
```
`Event: Object
- time: String
- type: String = platform.restoreReport
- record: PlatformRestoreReport`
```
The `PlatformRestoreReport` object has the following attributes:
* **errorType?** – string
* **metrics?** – `[RestoreReportMetrics](#RestoreReportMetrics)`
object
* **spans?** – List of `[Span](#Span)`
objects
* **status** – `[Status](#Status)` object
The following is an example `Event` of type `platform.restoreReport`:
```
`{
"time": "2022-10-12T00:00:15.064Z",
"type": "platform.restoreReport",
"record": {
"status": "success",
"metrics": {
"durationMs": 15.19
},
"spans": [
{
"name": "someTimeSpan",
"start": "2022-08-02T12:01:23:521Z",
"durationMs": 30.0
}
]
}
}`
```
### `platform.extension`
An `extension` event contains logs from the extension code. An `extension`
`Event` object has the following shape:
```
`Event: Object
- time: String
- type: String = extension
- record: {}`
```
The `PlatformExtension` object has the following attributes:
* **events** – List of `String`
* **name** – `String`
* **state** – `String`
The following is an example `Event` of type `platform.extension`:
```
`{
"time": "2022-10-12T00:02:15.000Z",
"type": "platform.extension",
"record": {
"events": [ "INVOKE", "SHUTDOWN" ],
"name": "my-telemetry-extension",
"state": "Ready"
}
}`
```
### `platform.telemetrySubscription`
A `platform.telemetrySubscription` event contains information about an extension subscription. A
`platform.telemetrySubscription`
`Event` object has the following shape:
```
`Event: Object
- time: String
- type: String = platform.telemetrySubscription
- record: PlatformTelemetrySubscription`
```
The `PlatformTelemetrySubscription` object has the following attributes:
* **name** – `String`
* **state** – `String`
* **types** – List of `String`
The following is an example `Event` of type `platform.telemetrySubscription`:
```
`{
"time": "2022-10-12T00:02:35.000Z",
"type": "platform.telemetrySubscription",
"record": {
"name": "my-telemetry-extension",
"state": "Subscribed",
"types": [ "platform", "function" ]
}
}`
```
### `platform.logsDropped`
A `platform.logsDropped` event contains information about dropped events. Lambda emits the
`platform.logsDropped` event when a function outputs logs at too high a rate for Lambda to process
them. When Lambda can't send logs to CloudWatch or to the extension subscribed to Telemetry API at the rate the
function produces them, it drops logs to prevent the function's execution from slowing down. A
`platform.logsDropped`
`Event` object has the following shape:
```
`Event: Object
- time: String
- type: String = platform.logsDropped
- record: PlatformLogsDropped`
```
The `PlatformLogsDropped` object has the following attributes:
* **droppedBytes** – `Integer`
* **droppedRecords** – `Integer`
* **reason** – `String`
The following is an example `Event` of type `platform.logsDropped`:
```
`{
"time": "2022-10-12T00:02:35.000Z",
"type": "platform.logsDropped",
"record": {
"droppedBytes": 12345,
"droppedRecords": 123,
"reason": "Some logs were dropped because the downstream consumer is slower than the logs production rate"
}
}`
```
### `function`
A `function` event contains logs from the function code. A `function`
`Event` object has the following shape:
```
`Event: Object
- time: String
- type: String = function
- record: {}`
```
The format of the `record` field depends on whether your function's logs are formatted in plain text or JSON format.
to learn more about log format configuration options, see [Configuring JSON and plain text log formats](./monitoring-cloudwatchlogs-logformat.html)
The following is an example `Event` of type `function` where the log format is plain text:
```
`{
"time": "2022-10-12T00:03:50.000Z",
"type": "function",
"record": "[INFO] Hello world, I am a function!"
}`
```
The following is an example `Event` of type `function` where the log format is JSON:
```
`{
"time": "2022-10-12T00:03:50.000Z",
"type": "function",
"record": {
"timestamp": "2022-10-12T00:03:50.000Z",
"level": "INFO",
"requestId": "79b4f56e-95b1-4643-9700-2807f4e68189",
"message": "Hello world, I am a function!"
}
}`
```
###### Note
If the schema version you're using is older than the `2022-12-13` version, then the `"record"` is always rendered
as a string even when your function's logging format is configured as JSON. For Lambda Managed Instances, you must use schema version
`2025-01-29`.
### `extension`
A `extension` event contains logs from the extension code. A `extension`
`Event` object has the following shape:
```
`Event: Object
- time: String
- type: String = extension
- record: {}`
```
The format of the `record` field depends on whether your function's logs are formatted in plain text or JSON format.
to learn more about log format configuration options, see [Configuring JSON and plain text log formats](./monitoring-cloudwatchlogs-logformat.html)
The following is an example `Event` of type `extension` where the log format is plain text:
```
`{
"time": "2022-10-12T00:03:50.000Z",
"type": "extension",
"record": "[INFO] Hello world, I am an extension!"
}`
```
The following is an example `Event` of type `extension` where the log format is JSON:
```
`{
"time": "2022-10-12T00:03:50.000Z",
"type": "extension",
"record": {
"timestamp": "2022-10-12T00:03:50.000Z",
"level": "INFO",
"requestId": "79b4f56e-95b1-4643-9700-2807f4e68189",
"message": "Hello world, I am an extension!"
}
}`
```
###### Note
If the schema version you're using is older than the `2022-12-13` version, then the `"record"` is always rendered
as a string even when your function's logging format is configured as JSON. For Lambda Managed Instances, you must use schema version
`2025-01-29`.
## Shared object types
This section details the types of shared objects that the Lambda Telemetry API supports.
### `InitPhase`
A string enum that describes the phase when the initialization step occurs. In most cases, Lambda runs the
function initialization code during the `init` phase. However, in some error cases, Lambda may re-run
the function initialization code during the `invoke` phase. (This is called a *suppressed
init*.)
* **Type** – `String`
* **Valid values** – `init`|`invoke`|`snap-start`
### `InitReportMetrics`
An object that contains metrics about an initialization phase.
* **Type** – `Object`
An `InitReportMetrics` object has the following shape:
```
`InitReportMetrics: Object
- durationMs: Double`
```
The following is an example `InitReportMetrics` object:
```
`{
"durationMs": 247.88
}`
```
### `InitType`
A string enum that describes how Lambda initialized the environment.
* **Type** – `String`
* **Valid values** –
`on-demand`|`provisioned-concurrency`
### `ReportMetrics`
An object that contains metrics about a completed phase.
* **Type** – `Object`
A `ReportMetrics` object has the following shape:
```
`ReportMetrics: Object
- billedDurationMs: Integer
- durationMs: Double
- initDurationMs?: Double
- maxMemoryUsedMB: Integer
- memorySizeMB: Integer
- restoreDurationMs?: Double`
```
The following is an example `ReportMetrics` object:
```
`{
"billedDurationMs": 694,
"durationMs": 693.92,
"initDurationMs": 397.68,
"maxMemoryUsedMB": 84,
"memorySizeMB": 128
}`
```
### `RestoreReportMetrics`
An object that contains metrics about a completed restoration phase.
* **Type** – `Object`
A `RestoreReportMetrics` object has the following shape:
```
`RestoreReportMetrics: Object
- durationMs: Double`
```
The following is an example `RestoreReportMetrics` object:
```
`{
"durationMs": 15.19
}`
```
### `RuntimeDoneMetrics`
An object that contains metrics about an invocation phase.
* **Type** – `Object`
A `RuntimeDoneMetrics` object has the following shape:
```
`RuntimeDoneMetrics: Object
- durationMs: Double
- producedBytes?: Integer`
```
The following is an example `RuntimeDoneMetrics` object:
```
`{
"durationMs": 200.0,
"producedBytes": 15
}`
```
### `Span`
An object that contains details about a span. A span represents a unit of work or operation in a trace. For
more information about spans, see [Span](https://opentelemetry.io/docs/reference/specification/trace/api/#span) on the
**Tracing API** page of the OpenTelemetry Docs website.
Lambda supports the following spans for the `platform.RuntimeDone` event:
* The `responseLatency` span describes how long it took your Lambda function to start
sending the response.
* The `responseDuration` span describes how long it took your Lambda function to finish
sending the entire response.
* The `runtimeOverhead` span describes how long it took the Lambda runtime to signal that it is ready to process the next function invoke. This is how long the runtime took to call the [next invocation](./runtimes-api.html#runtimes-api-next) API to get the next event after returning your function response.
The following is an example `responseLatency` span object:
```
`{
"name": "responseLatency",
"start": "2022-08-02T12:01:23.521Z",
"durationMs": 23.02
}`
```
### `Status`
An object that describes the status of an initialization or invocation phase. If the status is either
`failure` or `error`, then the `Status` object also contains an
`errorType` field describing the error.
* **Type** – `Object`
* **Valid status values** –
`success`|`failure`|`error`|`timeout`
### `TraceContext`
An object that describes the properties of a trace.
* **Type** – `Object`
A `TraceContext` object has the following shape:
```
`TraceContext: Object
- spanId?: String
- type: TracingType enum
- value: String`
```
The following is an example `TraceContext` object:
```
`{
"spanId": "073a49012f3c312e",
"type": "X-Amzn-Trace-Id",
"value": "Root=1-62e900b2-710d76f009d6e7785905449a;Parent=0efbd19962d95b05;Sampled=1"
}`
```
### `TracingType`
A string enum that describes the type of tracing in a `[TraceContext](#TraceContext)`
object.
* **Type** – `String`
* **Valid values** – `X-Amzn-Trace-Id`
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API reference
Converting events to OTel Spans
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.