---
url: https://docs.aws.amazon.com/lambda/latest/dg/runtime-management-identify.html
title: Identifying Lambda runtime version changes
word_count: 377
filtered: true
elements_removed: 0
density_score: 0.83
---

Identifying Lambda runtime version changes - AWS Lambda
Identifying Lambda runtime version changes - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#runtime-management-identify)
# Identifying Lambda runtime version changes
The [runtime version number](./runtimes-update.html) and ARN are logged in the `INIT\_START` log line,
which Lambda emits to CloudWatch Logs each time that it creates a new [execution environment](./concepts-basics.html#gettingstarted-concepts-runtime). Because the execution environment uses the same runtime
version for all function invocations, Lambda emits the `INIT\_START` log line only
when Lambda executes the init phase. Lambda doesn't emit this log line for each function
invocation. Lambda emits the log line to CloudWatch Logs, but it is not visible in the console.
###### Note
Runtime version numbers are not always sequential. For example, version 42 might be followed by version 45.
###### Example INIT\_START log line
```
INIT\_START Runtime Version: python:3.13.v14 Runtime Version ARN: arn:aws:lambda:eu-south-1::runtime:7b620fc2e66107a1046b140b9d320295811af3ad5d4c6a011fad1fa65127e9e6I
```
Rather than working directly with the logs, you can use [Amazon CloudWatch Contributor
Insights](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/ContributorInsights-CreateRule.html) to identify transitions between runtime versions. The following rule counts
the distinct runtime versions from each `INIT\_START` log line. To use the rule,
replace the example log group name `/aws/lambda/\*` with the appropriate prefix for
your function or group of functions.
```
`{
"Schema": {
"Name": "CloudWatchLogRule",
"Version": 1
},
"AggregateOn": "Count",
"Contribution": {
"Filters": [
{
"Match": "eventType",
"In": [
"INIT\_START"
]
}
],
"Keys": [
"runtimeVersion",
"runtimeVersionArn"
]
},
"LogFormat": "CLF",
"LogGroupNames": [
"`/aws/lambda/\*`"
],
"Fields": {
"1": "eventType",
"4": "runtimeVersion",
"8": "runtimeVersionArn"
}
}`
```
The following CloudWatch Contributor Insights report shows an example of a runtime version
transition as captured by the preceding rule. The orange line shows execution environment
initialization for the earlier runtime version (**python:3.13.v12**), and the
blue line shows execution environment initialization for the new runtime version
(**python:3.13.v14**).
![Graph showing the transition from one runtime version to another.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/runtime_version_graph.png)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Runtime version roll-back
Shared responsibility model
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.