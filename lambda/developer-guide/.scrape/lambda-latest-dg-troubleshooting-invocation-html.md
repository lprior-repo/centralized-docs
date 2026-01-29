---
url: https://docs.aws.amazon.com/lambda/latest/dg/troubleshooting-invocation.html
title: Troubleshoot invocation issues in Lambda
word_count: 2216
filtered: true
elements_removed: 0
density_score: 0.87
---

Troubleshoot invocation issues in Lambda - AWS Lambda
Troubleshoot invocation issues in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#troubleshooting-invocation)
[Lambda: Function times out during Init phase (Sandbox.Timedout)](#troubleshooting-timeouts)[IAM: lambda:InvokeFunction not authorized](#troubleshooting-invocation-noauth)[Lambda: Couldn't find valid bootstrap (Runtime.InvalidEntrypoint)](#troubleshooting-invocation-bootstrap)[Lambda: Operation cannot be performed ResourceConflictException](#troubleshooting-invocation-ResourceConflictException)[Lambda: Function is stuck in Pending](#troubleshooting-invocation-pending)[Lambda: One function is using all concurrency](#troubleshooting-invocation-allconcurrency)[General: Cannot invoke function with other accounts or services](#troubleshooting-invocation-cannotinvoke)[General: Function invocation is looping](#troubleshooting-invocation-loop)[Lambda: Alias routing with provisioned concurrency](#troubleshooting-invocation-alias)[Lambda: Cold starts with provisioned concurrency](#troubleshooting-invocation-coldstart)[Lambda: Cold starts with new versions](#troubleshooting-invocation-newversion)[Lambda: Unexpected Node.js exit in runtime (Runtime.NodejsExit)](#troubleshooting-invocation-nodejs-exit)[EFS: Function could not mount the EFS file system](#troubleshooting-invocation-efsmount)[EFS: Function could not connect to the EFS file system](#troubleshooting-invocation-efsconnect)[EFS: Function could not mount the EFS file system due to timeout](#troubleshooting-invocation-efstimeout)[Lambda: Lambda detected an IO process that was taking too long](#troubleshooting-invocation-ioprocess)[Container:
CodeArtifactUserException errors](#troubleshooting-deployment-container-artifact)[Container:
InvalidEntrypoint errors](#troubleshooting-deployment-container-entrypoint)
# Troubleshoot invocation issues in Lambda
When you invoke a Lambda function, Lambda validates the request and checks for scaling capacity before sending the
event to your function or, for asynchronous invocation, to the event queue. Invocation errors can be caused by
issues with request parameters, event structure, function settings, user permissions, resource permissions, or
limits.
If you invoke your function directly, you see any invocation errors in the response from Lambda. If you invoke
your function asynchronously with an event source mapping or through another service, you might find errors in logs,
a dead-letter queue, or a failed-event destination. Error handling options and retry behavior vary depending on how
you invoke your function and on the type of error.
For a list of error types that the `Invoke` operation can return, see [Invoke](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html).
###### Topics
* [Lambda: Function times out during Init phase (Sandbox.Timedout)](#troubleshooting-timeouts)
* [IAM: lambda:InvokeFunction not authorized](#troubleshooting-invocation-noauth)
* [Lambda: Couldn't find valid bootstrap (Runtime.InvalidEntrypoint)](#troubleshooting-invocation-bootstrap)
* [Lambda: Operation cannot be performed ResourceConflictException](#troubleshooting-invocation-ResourceConflictException)
* [Lambda: Function is stuck in Pending](#troubleshooting-invocation-pending)
* [Lambda: One function is using all concurrency](#troubleshooting-invocation-allconcurrency)
* [General: Cannot invoke function with other accounts or services](#troubleshooting-invocation-cannotinvoke)
* [General: Function invocation is looping](#troubleshooting-invocation-loop)
* [Lambda: Alias routing with provisioned concurrency](#troubleshooting-invocation-alias)
* [Lambda: Cold starts with provisioned concurrency](#troubleshooting-invocation-coldstart)
* [Lambda: Cold starts with new versions](#troubleshooting-invocation-newversion)
* [Lambda: Unexpected Node.js exit in runtime (Runtime.NodejsExit)](#troubleshooting-invocation-nodejs-exit)
* [EFS: Function could not mount the EFS file system](#troubleshooting-invocation-efsmount)
* [EFS: Function could not connect to the EFS file system](#troubleshooting-invocation-efsconnect)
* [EFS: Function could not mount the EFS file system due to timeout](#troubleshooting-invocation-efstimeout)
* [Lambda: Lambda detected an IO process that was taking too long](#troubleshooting-invocation-ioprocess)
* [Container:
CodeArtifactUserException errors](#troubleshooting-deployment-container-artifact)
* [Container:
InvalidEntrypoint errors](#troubleshooting-deployment-container-entrypoint)
## Lambda: Function times out during Init phase (Sandbox.Timedout)
**Error:**
*Task timed out after 3.00 seconds*
When the [Init](./lambda-runtime-environment.html#runtimes-lifecycle-ib) phase times out, Lambda initializes the execution environment again by re-running the `Init` phase when the next invoke request arrives. This is called a [suppressed init](./lambda-runtime-environment.html#suppressed-init). However, if your function is configured with a short [timeout duration](./configuration-timeout.html) (generally around 3 seconds), the suppressed init might not complete during the allocated timeout duration, causing the `Init` phase to time out again. Alternatively, the suppressed init completes but does not leave enough time for the [Invoke](./lambda-runtime-environment.html#runtimes-lifecycle-invoke) phase to complete, causing the `Invoke` phase to time out.
To reduce timeout errors, use one or more of the following strategies:
* **Increase the function timeout duration**: Extend the [timeout](./configuration-timeout.html) to give the `Init` and `Invoke` phases time to complete successfully.
* **Increase the function memory allocation**: More [memory](./configuration-memory.html) also means more proportional CPU allocation, which can speed up both the `Init` and `Invoke` phases.
* **Optimize the function initialization code**: Reduce the time needed for initialization to ensure that the the `Init` and `Invoke` phase can complete within the configured timeout.
## IAM: lambda:InvokeFunction not authorized
**Error:**
*User: arn:aws:iam::123456789012:user/developer is not authorized to perform: lambda:InvokeFunction on
resource: my-function*
Your user, or the role that you assume, must have permission to invoke a function. This
requirement also applies to Lambda functions and other compute resources that invoke functions. Add the AWS managed
policy **AWSLambdaRole** to your user, or add a custom policy that allows the
`lambda:InvokeFunction` action on the target function.
###### Note
The name of the IAM action (`lambda:InvokeFunction`) refers to the `Invoke`
Lambda API operation.
For more information, see [Managing permissions in AWS Lambda](./lambda-permissions.html)
.
## Lambda: Couldn't find valid bootstrap (Runtime.InvalidEntrypoint)
**Error:**
*Couldn't find valid bootstrap(s): [/var/task/bootstrap /opt/bootstrap]*
This error typically occurs when the root of your deployment package doesn't contain an executable file named `bootstrap`. For example, if you're deploying a `provided.al2023` function with a .zip file, the `bootstrap` file must be at the root of the .zip file, not in a directory.
## Lambda: Operation cannot be performed ResourceConflictException
**Error:**
*ResourceConflictException: The operation cannot be performed at this time. The function is currently in
the following state: Pending*
When you connect a function to a virtual private cloud (VPC) at the time of creation, the function enters a
`Pending` state while Lambda creates elastic network interfaces. During this time, you can't invoke or
modify your function. If you connect your function to a VPC after creation, you can invoke it while the update is
pending, but you can't modify its code or configuration.
For more information, see [Lambda function states](./functions-states.html)
.
## Lambda: Function is stuck in Pending
**Error:**
*A function is stuck in the `Pending` state for several minutes.*
If a function is stuck in the `Pending` state for more than six minutes, call one of the following
API operations to unblock it:
* [UpdateFunctionCode](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionCode.html)
* [UpdateFunctionConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionConfiguration.html)
* [PublishVersion](https://docs.aws.amazon.com/lambda/latest/api/API_PublishVersion.html)
Lambda cancels the pending operation and puts the function into the `Failed` state. You can then
attempt another update.
## Lambda: One function is using all concurrency
**Issue:**
*One function is using all of the available concurrency, causing other functions to be
throttled.*
To divide your AWS account's available concurrency in an AWS Region into pools, use [reserved concurrency](./configuration-concurrency.html). Reserved concurrency ensures that a function can
always scale to its assigned concurrency, and that it doesn't scale beyond its assigned concurrency.
## General: Cannot invoke function with other accounts or services
**Issue:**
*You can invoke your function directly, but it doesn't run when another service or account invokes
it.*
You grant [other services](./lambda-services.html) and accounts permission to invoke a function in
the function's [resource-based policy](./access-control-resource-based.html). If the invoker is in
another account, that user must also have [permission to invoke
functions](./access-control-identity-based.html).
## General: Function invocation is looping
**Issue:**
*Function is invoked continuously in a loop.*
This typically occurs when your function manages resources in the same AWS service that triggers it. For
example, it's possible to create a function that stores an object in an Amazon Simple Storage Service (Amazon S3) bucket that's configured
with a [notification that invokes the function again](./with-s3.html). To stop the function from
running, reduce the available [concurrency](./lambda-concurrency.html) to zero, which throttles all future
invocations. Then, identify the code path or configuration error that caused the recursive
invocation. Lambda automatically detects and stops recursive loops for some AWS services and SDKs. For more information, see
[Use Lambda recursive loop detection to prevent infinite loops](./invocation-recursion.html).
## Lambda: Alias routing with provisioned concurrency
**Issue:**
*Provisioned concurrency spillover invocations during alias routing.*
Lambda uses a simple probabilistic model to distribute the traffic between the two function versions. At low
traffic levels, you might see a high variance between the configured and actual percentage of traffic on each
version. If your function uses provisioned concurrency, you can avoid [spillover invocations](./monitoring-metrics-types.html#invocation-metrics) by configuring a higher number of
provisioned concurrency instances during the time that alias routing is active.
## Lambda: Cold starts with provisioned concurrency
**Issue:**
*You see cold starts after enabling provisioned concurrency.*
When the number of concurrent executions on a function is less than or equal to the [configured level of provisioned concurrency](./provisioned-concurrency.html), there
shouldn't be any cold starts. To help you confirm if provisioned concurrency is operating normally, do the
following:
* [Check that provisioned concurrency is enabled](./provisioned-concurrency.html) on the function version or alias.
###### Note
Provisioned concurrency is not configurable on the unpublished [version of the function](./configuration-versions.html) ($LATEST).
* Ensure that your triggers invoke the correct function version or alias. For example, if you're using
Amazon API Gateway, check that API Gateway invokes the function version or alias with provisioned concurrency, not $LATEST. To
confirm that provisioned concurrency is being used, you can check the [ProvisionedConcurrencyInvocations Amazon CloudWatch metric](./monitoring-concurrency.html#provisioned-concurrency-metrics). A non-zero
value indicates that the function is processing invocations on initialized execution environments.
* Determine whether your function concurrency exceeds the configured level of provisioned concurrency by
checking the [ProvisionedConcurrencySpilloverInvocations CloudWatch
metric](./monitoring-concurrency.html#provisioned-concurrency-metrics). A non-zero value indicates that all provisioned concurrency is in use and some invocation
occurred with a cold start.
* Check your [invocation frequency](./gettingstarted-limits.html#api-requests) (requests per second).
Functions with provisioned concurrency have a maximum rate of 10 requests per second per provisioned
concurrency. For example, a function configured with 100 provisioned concurrency can handle 1,000 requests per
second. If the invocation rate exceeds 1,000 requests per second, some cold starts can occur.
## Lambda: Cold starts with new versions
**Issue:**
*You see cold starts while deploying new versions of your function.*
When you update a function alias, Lambda automatically shifts provisioned concurrency to the new version based
on the weights configured on the alias.
**Error:**
*KMSDisabledException: Lambda was unable to decrypt the environment variables because the KMS key used is
disabled. Please check the function's KMS key settings.*
This error can occur if your AWS Key Management Service (AWS KMS) key is disabled, or if the grant that allows Lambda to use the key
is revoked. If the grant is missing, configure the function to use a different key. Then, reassign the custom key to
recreate the grant.
## Lambda: Unexpected Node.js exit in runtime (Runtime.NodejsExit)
**Issue:**
*Lambda runtime client detected an unexpected Node.js exit
code.*
This error occurs when your function exits before all Promises are settled, for example
due to a code bug. It can also occur when Node.js detects a deadlock that prevents Promises
from being settled. This error affects only async style handlers, not callback-style
handlers.
**Affected runtimes:** Node.js 18 and later.
**To resolve this issue:**
1. Check your function code for unsettled promises in async handlers.
2. Ensure all promises are properly settled (resolved or rejected) before the function
completes.
3. Review your code for potential race conditions in asynchronous operations.
For more information about Node.js exit codes and process termination, see the [Node.js documentation](https://nodejs.org/docs/latest/api/process.html#exit-codes).
## EFS: Function could not mount the EFS file system
**Error:**
*EFSMountFailureException: The function could not mount the EFS file system with access point
arn:aws:elasticfilesystem:us-east-2:123456789012:access-point/fsap-015cxmplb72b405fd.*
The mount request to the function's [file system](./configuration-filesystem.html) was rejected.
Check the function's permissions, and confirm that its file system and access point exist and are ready for
use.
## EFS: Function could not connect to the EFS file system
**Error:**
*EFSMountConnectivityException: The function couldn't connect to the Amazon EFS file system with access point
arn:aws:elasticfilesystem:us-east-2:123456789012:access-point/fsap-015cxmplb72b405fd. Check your network
configuration and try again.*
The function couldn't establish a connection to the function's [file
system](./configuration-filesystem.html) with the NFS protocol (TCP port 2049). Check the [security group and routing configuration](https://docs.aws.amazon.com/efs/latest/ug/network-access.html) for the VPC's subnets.
If you get these errors after updating your function's VPC configuration settings, try unmounting and
remounting the file system.
## EFS: Function could not mount the EFS file system due to timeout
**Error:**
*EFSMountTimeoutException: The function could not mount the EFS file system with access point
{arn:aws:elasticfilesystem:us-east-2:123456789012:access-point/fsap-015cxmplb72b405fd} due to mount time
out.*
The function could connect to the function's [file system](./configuration-filesystem.html), but
the mount operation timed out. Try again after a short time and consider limiting the function's [concurrency](./configuration-concurrency.html) to reduce load on the file system.
## Lambda: Lambda detected an IO process that was taking too long
*EFSIOException: This function instance was stopped because Lambda detected an IO process that was taking
too long.*
A previous invocation timed out and Lambda couldn't terminate the function handler. This issue can occur when an
attached file system runs out of burst credits and the baseline throughput is insufficient. To increase throughput,
you can increase the size of the file system or use provisioned throughput.
## Container:
CodeArtifactUserException errors
**Error:**
*CodeArtifactUserPendingException error message*
The CodeArtifact is pending optimization. The function transitions to the [Active state](./functions-states.html) when
Lambda completes the optimization. HTTP response code 409.
**Error:**
*CodeArtifactUserDeletedException error message*
The CodeArtifact is scheduled to be deleted. HTTP response code 409.
**Error:**
*CodeArtifactUserFailedException error message*
Lambda failed to optimize the code. You need to correct the code and upload it again.
HTTP response code 409.
## Container:
InvalidEntrypoint errors
**Error:**
*Runtime.ExitError or "errorType": "Runtime.InvalidEntrypoint"*
Verify that the ENTRYPOINT to your container image includes the absolute path as the
location. Also verify that the image does not contain a symlink as the ENTRYPOINT.
**Error:**
*You are using an CloudFormation template, and your container ENTRYPOINT is being overridden
with a null or empty value.*
Review the [ImageConfig](https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-properties-lambda-function-imageconfig.html) resource in the CloudFormation template. If you declare an
`ImageConfig` resource in your template, you must provide non-empty values for
all three of the properties.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Deployment
Execution
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.