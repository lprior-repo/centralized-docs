---
url: https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html
title: Working with Lambda environment variables
word_count: 1961
filtered: true
elements_removed: 0
density_score: 0.88
---

Working with Lambda environment variables - AWS Lambda
Working with Lambda environment variables - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#configuration-envvars)
[Create environment variables](#create-environment-variables)[Example scenario for environment variables](#configuration-envvars-example)[Retrieve environment variables](#retrieve-environment-variables)[Defined runtime environment variables](#configuration-envvars-runtime)
# Working with Lambda environment variables
You can use environment variables to adjust your function's behavior without updating code. An environment
variable is a pair of strings that is stored in a function's version-specific configuration. The Lambda runtime makes
environment variables available to your code and sets additional environment variables that contain information
about the function and invocation request.
###### Note
To increase security, we recommend that you use AWS Secrets Manager instead of environment variables to store
database credentials and other sensitive information like API keys or authorization tokens. For more information, see [Use Secrets Manager secrets in Lambda functions](./with-secrets-manager.html).
Environment variables are not evaluated before the function invocation. Any value you define is considered a
literal string and not expanded. Perform the variable evaluation in your function code.
## Creating Lambda environment variables
You can configure environment variables in Lambda using the Lambda console, the AWS Command Line Interface (AWS CLI), AWS Serverless Application Model (AWS SAM), or using an AWS SDK.
Console
You define environment variables on the unpublished version of your function. When you publish a version, the
environment variables are locked for that version along with other [version-specific configuration settings](./configuration-versions.html).
You create an environment variable for your function by defining a key and a value. Your function uses the
name of the key to retrieve the value of the environment variable.
###### To set environment variables in the Lambda console
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Choose the **Configuration** tab, then choose **Environment variables**.
4. Under **Environment variables**, choose **Edit**.
5. Choose **Add environment variable**.
6. Enter a key and value.
###### Requirements
* Keys start with a letter and are at least two characters.
* Keys only contain letters, numbers, and the underscore character (`\_`).
* Keys aren't [reserved by Lambda](#configuration-envvars-runtime).
* The total size of all environment variables doesn't exceed 4 KB.
* Choose **Save**.
###### To generate a list of environment variables in the console code editor
You can generate a list of environment variables in the Lambda code editor. This is a quick way to reference
your environment variables while you code.
1. Choose the **Code** tab.
2. Scroll down to the **ENVIRONMENT VARIABLES** section of the code editor. Existing environment variables are listed here:
![ENVIRONMENT VARIABLES section of the Lambda console code editor](https://docs.aws.amazon.com/images/lambda/latest/dg/images/env-var.png)
3. To create new environment variables, choose the choose the plus sign (
![plus sign](https://docs.aws.amazon.com/images/lambda/latest/dg/images/add-plus.png)
):
![Add environment variables in the Lambda console code editor](https://docs.aws.amazon.com/images/lambda/latest/dg/images/create-env-var.png)
Environment variables remain encrypted when listed in the console code editor. If you enabled encryption helpers for encryption in transit, then those settings remain unchanged. For more information, see [Securing Lambda environment variables](./configuration-envvars-encryption.html).
The environment variables list is read-only and is available only on the Lambda console. This file is not included when you download the function's .zip file archive, and you can't add environment variables by uploading this file.
AWS CLI
The following example sets two environment variables on a function named `my-function`.
```
`aws lambda update-function-configuration \\
--function-name `my-function` \\
--environment `"Variables={BUCKET=amzn-s3-demo-bucket,KEY=file.txt}"``
```
When you apply environment variables with the `update-function-configuration` command, the entire
contents of the `Variables` structure is replaced. To retain existing environment variables when you
add a new one, include all existing values in your request.
To get the current configuration, use the `get-function-configuration` command.
```
`aws lambda get-function-configuration \\
--function-name `my-function``
```
You should see the following output:
```
{
"FunctionName": "my-function",
"FunctionArn": "arn:aws:lambda:us-east-2:111122223333:function:my-function",
"Runtime": "nodejs24.x",
"Role": "arn:aws:iam::111122223333:role/lambda-role",
"Environment": {
"Variables": {
"BUCKET": "amzn-s3-demo-bucket",
"KEY": "file.txt"
}
},
"RevisionId": "0894d3c1-2a3d-4d48-bf7f-abade99f3c15",
...
}
```
You can pass the revision ID from the output of `get-function-configuration` as a parameter to
`update-function-configuration`. This ensures that the values don't change between when you read the
configuration and when you update it.
To configure a function's encryption key, set the `KMSKeyARN` option.
```
`aws lambda update-function-configuration \\
--function-name `my-function` \\
--kms-key-arn `arn:aws:kms:us-east-2:111122223333:key/055efbb4-xmpl-4336-ba9c-538c7d31f599``
```
AWS SAM
You can use the [AWS Serverless Application Model](<https://docs.aws.amazon.com//serverless-application-model/latest/developerguide/serverless-getting-started.html >) to configure environment variables for your function. Update the [Environment](https://docs.aws.amazon.com//serverless-application-model/latest/developerguide/sam-resource-function.html#sam-function-environment) and [Variables](https://docs.aws.amazon.com//AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-environment.html#cfn-lambda-function-environment-variables) properties in your `template.yaml` file and then run [sam deploy](https://docs.aws.amazon.com//serverless-application-model/latest/developerguide/sam-cli-command-reference-sam-deploy.html).
###### Example template.yaml
```
`AWSTemplateFormatVersion: '2010-09-09'
Transform: AWS::Serverless-2016-10-31
Description: An AWS Serverless Application Model template describing your function.
Resources:
`my-function`:
Type: AWS::Serverless::Function
Properties:
CodeUri: .
Description: ''
MemorySize: 128
Timeout: 120
Handler: index.handler
Runtime: nodejs24.x
Architectures:
- x86\_64
EphemeralStorage:
Size: 10240
Environment:
Variables:
`BUCKET: amzn-s3-demo-bucket`
`KEY: file.txt`
*# Other function properties...*`
```
AWS SDKs
To manage environment variables using an AWS SDK, use the following API operations.
* [UpdateFunctionConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionConfiguration.html)
* [GetFunctionConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_GetFunctionConfiguration.html)
* [CreateFunction](https://docs.aws.amazon.com/lambda/latest/api/API_CreateFunction.html)
To learn more, refer to the [AWS SDK documentation](https://aws.amazon.com/developer/tools/) for your preferred programming language.
## Example scenario for environment variables
You can use environment variables to customize function behavior in your test environment and production
environment. For example, you can create two functions with the same code but different configurations. One
function connects to a test database, and the other connects to a production database. In this situation, you use
environment variables to pass the hostname and other connection details for the database to the function.
The following example shows how to define the database host and database name as environment variables.
![Environment variables in the Lambda console.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/console-env.png)
If you want your test environment to generate more debug information than the production environment, you
could set an environment variable to configure your test environment to use more verbose logging or more detailed
tracing.
For example, in your test environment, you could set an environment variable with the key `LOG\_LEVEL` and a value indicating a log level of
debug or trace. In your Lambda function's code, you can then use this environment variable to set the log level.
The following code examples in Python and Node.js illustrate how you can achieve this. These examples assume your environment variable has a
value of `DEBUG` in Python or `debug` in Node.js.
Python
###### Example Python code to set log level
```
`import os
import logging
# Get the log level from the environment variable and default to INFO if not set
log\_level = os.environ.get('LOG\_LEVEL', 'INFO')
# Produce some example log outputs
logger.debug('This is a log with detailed debug information - shown only in test environment')
logger.info('This is a log with standard information - shown in production and test environments')
`
```
Node.js (ES module format)
###### Example Node.js code to set log level
This example uses the `winston` logging library. Use npm to add this library to your function's deployment package. For more information, see
[Creating a .zip deployment package with dependencies](./nodejs-package.html#nodejs-package-create-dependencies).
```
`import winston from 'winston';
// Initialize the logger using the log level from environment variables, defaulting to INFO if not set
const logger = winston.createLogger({
level: process.env.LOG\_LEVEL || 'info',
format: winston.format.json(),
transports: [new winston.transports.Console()]
});
export const handler = async (event) =&gt;&gt; {
// Produce some example log outputs
logger.debug('This is a log with detailed debug information - shown only in test environment');
logger.info('This is a log with standard information - shown in production and test environment');
};`
```
## Retrieving Lambda environment variables
To retrieve environment variables in your function code, use the standard method for your programming
language.
Node.js
```
`let region = process.env.AWS\_REGION`
```
Python
```
`import os
region = os.environ['AWS\_REGION']`
```
###### Note
In some cases, you may need to use the following format:
```
`region = os.environ.get('AWS\_REGION')`
```
Ruby
```
`region = ENV["AWS\_REGION"]`
```
Java
```
`String region = System.getenv("AWS\_REGION");`
```
Go
```
`var region = os.Getenv("AWS\_REGION")`
```
C#
```
`string region = Environment.GetEnvironmentVariable("AWS\_REGION");`
```
PowerShell
```
`$region = $env:AWS\_REGION`
```
Lambda stores environment variables securely by encrypting them at rest. You can [configure Lambda to use a different encryption key](./configuration-envvars-encryption.html), encrypt
environment variable values on the client side, or set environment variables in an CloudFormation template with
AWS Secrets Manager.
## Defined runtime environment variables
Lambda [runtimes](./lambda-runtimes.html) set several environment variables during initialization.
Most of the environment variables provide information about the function or runtime. The keys for these
environment variables are *reserved* and cannot be set in your function configuration.
###### Reserved environment variables
* `\_HANDLER` – The handler location configured on the function.
* `\_X\_AMZN\_TRACE\_ID` – The [X-Ray tracing
header](./services-xray.html). This environment variable changes with each invocation.
* This environment variable is not defined for OS-only runtimes (the `provided` runtime family).
You can set `\_X\_AMZN\_TRACE\_ID` for custom runtimes using the
`Lambda-Runtime-Trace-Id` response header from the
[Next invocation](./runtimes-api.html#runtimes-api-next).
* For Java runtime versions 17 and later, this environment variable is not used.
Instead, Lambda stores tracing information in the `com.amazonaws.xray.traceHeader`
system property.
* `AWS\_DEFAULT\_REGION` – The default AWS Region where the Lambda function is executed.
* `AWS\_REGION` – The AWS Region where the Lambda function is executed. If defined, this value overrides the `AWS\_DEFAULT\_REGION`.
* For more information about using the AWS Region environment variables with AWS SDKs, see [AWS Region](https://docs.aws.amazon.com/sdkref/latest/guide/feature-region.html#feature-region-sdk-compat)
in the *AWS SDKs and Tools Reference Guide*.
* `AWS\_EXECUTION\_ENV` – The [runtime identifier](./lambda-runtimes.html),
prefixed by `AWS\_Lambda\_` (for example, `AWS\_Lambda\_java8`). This environment variable is not defined for OS-only runtimes (the `provided` runtime family).
* `AWS\_LAMBDA\_FUNCTION\_NAME` – The name of the function.
* `AWS\_LAMBDA\_FUNCTION\_MEMORY\_SIZE` – The amount of memory available to the function in
MB.
* `AWS\_LAMBDA\_FUNCTION\_VERSION` – The version of the function being
executed.
* `AWS\_LAMBDA\_INITIALIZATION\_TYPE` – The initialization type of the function, which is `on-demand`, `provisioned-concurrency`, `snap-start`, or `lambda-managed-instances`. For information, see [Configuring provisioned concurrency](./provisioned-concurrency.html), [Improving startup performance with Lambda SnapStart](./snapstart.html), or [Lambda Managed Instances](./lambda-managed-instances.html).
* `AWS\_LAMBDA\_LOG\_GROUP\_NAME`, `AWS\_LAMBDA\_LOG\_STREAM\_NAME` – The name of the
Amazon CloudWatch Logs group and stream for the function. The `AWS\_LAMBDA\_LOG\_GROUP\_NAME` and `AWS\_LAMBDA\_LOG\_STREAM\_NAME` environment variables are not available in Lambda SnapStart functions.
* `AWS\_ACCESS\_KEY`, `AWS\_ACCESS\_KEY\_ID`, `AWS\_SECRET\_ACCESS\_KEY`, `AWS\_SESSION\_TOKEN`
– The access keys obtained from the function's [execution
role](./lambda-intro-execution-role.html).
* `AWS\_LAMBDA\_RUNTIME\_API` – ([Custom runtime](./runtimes-custom.html)) The
host and port of the [runtime API](./runtimes-api.html).
* `LAMBDA\_TASK\_ROOT` – The path to your Lambda function code.
* `LAMBDA\_RUNTIME\_DIR` – The path to runtime libraries.
* `AWS\_LAMBDA\_MAX\_CONCURRENCY` – (Lambda Managed Instances only) The maximum number of concurrent invocations Lambda will send to one execution environment.
The following additional environment variables aren't reserved and can be extended in your function
configuration.
###### Unreserved environment variables
* `LANG` – The locale of the runtime (`en\_US.UTF-8`).
* `PATH` – The execution path
(`/usr/local/bin:/usr/bin/:/bin:/opt/bin`).
* `LD\_LIBRARY\_PATH` – The system library path
(`/var/lang/lib:/lib64:/usr/lib64:$LAMBDA\_RUNTIME\_DIR:$LAMBDA\_RUNTIME\_DIR/lib:$LAMBDA\_TASK\_ROOT:$LAMBDA\_TASK\_ROOT/lib:/opt/lib`).
* `NODE\_PATH` – ([Node.js](./lambda-nodejs.html)) The Node.js library path
(`/opt/nodejs/node12/node\_modules/:/opt/nodejs/node\_modules:$LAMBDA\_RUNTIME\_DIR/node\_modules`).
* `NODE\_OPTIONS` – ([Node.js](./lambda-nodejs.html)) For
Node.js runtimes, you can use `NODE\_OPTIONS` to re-enable experimental features
that Lambda disables by default.
* `PYTHONPATH` – ([Python](./lambda-python.html)) The Python
library path (`$LAMBDA\_RUNTIME\_DIR`).
* `GEM\_PATH` – ([Ruby](./lambda-ruby.html)) The Ruby library path
(`$LAMBDA\_TASK\_ROOT/vendor/bundle/ruby/3.3.0:/opt/ruby/gems/3.3.0`).
* `AWS\_XRAY\_CONTEXT\_MISSING` – For X-Ray tracing, Lambda sets this to
`LOG\_ERROR` to avoid throwing runtime errors from the X-Ray SDK.
* `AWS\_XRAY\_DAEMON\_ADDRESS` – For X-Ray tracing, the IP address and port of the X-Ray
daemon.
* `AWS\_LAMBDA\_DOTNET\_PREJIT` – ([.NET](./lambda-csharp.html)) Set this variable to enable or
disable .NET specific runtime optimizations. Values include `always`, `never`, and
`provisioned-concurrency`. For more information, see [Configuring provisioned concurrency for a function](./provisioned-concurrency.html).
* `TZ` – The environment's time zone (`:UTC`). The execution environment uses
NTP to synchronize the system clock.
The sample values shown reflect the latest runtimes. The presence of specific variables or their values can
vary on earlier runtimes.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Configure durable functions
Securing environment variables
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.