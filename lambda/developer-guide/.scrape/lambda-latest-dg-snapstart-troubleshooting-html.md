---
url: https://docs.aws.amazon.com/lambda/latest/dg/snapstart-troubleshooting.html
title: Troubleshooting SnapStart errors for Lambda functions
word_count: 751
filtered: true
elements_removed: 0
density_score: 0.90
---

Troubleshooting SnapStart errors for Lambda functions - AWS Lambda
Troubleshooting SnapStart errors for Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#snapstart-troubleshooting)
[SnapStartNotReadyException](#snapstartnotreadyexception)[SnapStartTimeoutException](#snapstart-invocation-failure)[500 Internal Service Error](#snapstart-500-error)[401 Unauthorized](#snapstart-401-unauthorized)[UnknownHostException (Java)](#snapstart-dns-caching)[Snapshot creation failures](#snapstart-creation-failure)[Snapshot creation latency](#snapstart-creation-latency)
# Troubleshooting SnapStart errors for Lambda functions
This page addresses common issues that occur when using Lambda SnapStart, including snapshot creation errors, timeout errors, and internal service errors.
## SnapStartNotReadyException
**Error:** An error occurred (SnapStartNotReadyException) when calling the Invoke20150331 operation: Lambda is initializing your function. It will be ready to invoke once your function state becomes ACTIVE.
### Common causes
This error occurs when you try to invoke a function version that is in the `Inactive` [state](./snapstart-activate.html#snapstart-function-states). Your function version becomes `Inactive` when it hasn't been invoked for 14 days or when Lambda periodically recycles the execution environment
### Resolution
Wait until the function version reaches the `Active` state, and then invoke it again.
## SnapStartTimeoutException
**Issue:** You receive a `SnapStartTimeoutException` when you try to invoke a SnapStart function version.
### Common cause
During the [Restore](./lambda-runtime-environment.html#runtimes-lifecycle-restore) phase, Lambda restores the Java runtime and runs any after-restore
[runtime hooks](./snapstart-runtime-hooks.html). If an after-restore runtime hook runs for longer than 10 seconds, the `Restore` phase times out and you get an error when you try to invoke the function. Network connection and credentials issues can also cause `Restore` phase timeouts.
### Resolution
Check the function's CloudWatch logs for timeout errors that happened during the [Restore](./lambda-runtime-environment.html#runtimes-lifecycle-restore) phase. Make sure that all after-restore hooks complete in less than 10 seconds.
###### Example CloudWatch log
```
{ "cause": "Lambda couldn't restore the snapshot within the timeout limit. (Service: Lambda, Status Code: 408, Request ID: 11a222c3-410f-427c-ab22-931d6bcbf4f2)", "error": "Lambda.SnapStartTimeoutException"}
```
## 500 Internal Service Error
**Error:** Lambda was unable to create a new snapshot because you have reached your concurrent snapshot creation limit.
### Common cause
A 500 error is an internal error within the Lambda service itself, rather than an issue with your function or code. These errors are often intermittent.
## 401 Unauthorized
**Error:** Bad session token or header key
### Common cause
This error occurs when using the [AWS Systems Manager Parameter Store and AWS Secrets Manager extension](./with-secrets-manager.html) with Lambda SnapStart.
### Resolution
The AWS Systems Manager Parameter Store and AWS Secrets Manager extension isn't compatible with SnapStart. The extension generates credentials for communicating with AWS Secrets Manager during function initialization, which causes expired credential errors when used with SnapStart.
## UnknownHostException (Java)
**Error:** Unable to execute HTTP request: Certificate for `abc.us-east-1.amazonaws.com` doesn't match any of the subject alternative names.
### Common cause
Lambda functions already cache DNS responses. If you use another DNS cache with SnapStart, then you might experience connection timeouts when the function resumes from a snapshot.
### Resolution
To prevent `UnknownHostException` failures in the Java 11 runtime, we recommend setting `networkaddress.cache.negative.ttl` to 0. In Java 17 and later runtimes, this step isn't necessary. You can set this
property for a Lambda function with the `AWS\_LAMBDA\_JAVA\_NETWORKADDRESS\_CACHE\_NEGATIVE\_TTL=0` environment variable.
## Snapshot creation failures
**Error:** AWS Lambda could not invoke your SnapStart function. If this error persists, check your function's CloudWatch logs for initialization errors.
### Resolution
Review your function's Amazon CloudWatch logs for before-checkpoint [runtime hook](./snapstart-runtime-hooks.html) timeouts. You can also try publishing a new function version, which can sometimes resolve the issue.
## Snapshot creation latency
**Issue:** When you publish a new function version, the function stays in the `Pending` [state](./snapstart-activate.html#snapstart-function-states) for a long time.
### Common cause
When Lambda creates a snapshot, your initialization code can run for up to 15 minutes. The time limit is 130 seconds or the [configured function timeout](./configuration-timeout.html) (maximum 900 seconds), whichever is higher.
If your function is [attached to a VPC](./configuration-vpc.html#configuration-vpc-attaching), Lambda might also need to create network interfaces before the function becomes `Active`. If you try to invoke the function version while the function is `Pending`, you might get a 409 `ResourceConflictException`. If the function is invoked using an Amazon API Gateway endpoint, you might get a 500 error in API Gateway.
### Resolution
Wait at least 15 minutes for the function version to initialize before invoking it.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Best practices
Tenant isolation
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.