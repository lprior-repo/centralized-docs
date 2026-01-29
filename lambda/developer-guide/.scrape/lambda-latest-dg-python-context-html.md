---
url: https://docs.aws.amazon.com/lambda/latest/dg/python-context.html
title: Using the Lambda context object to retrieve Python function information
word_count: 504
filtered: true
elements_removed: 0
density_score: 0.89
---

Using the Lambda context object to retrieve Python function information - AWS Lambda
Using the Lambda context object to retrieve Python function information - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#python-context)
# Using the Lambda context object to retrieve Python function information
When Lambda runs your function, it passes a context object to the [handler](./python-handler.html). This object provides methods and properties that provide information about the invocation, function, and execution environment.
For more information on how the context object is passed to the function handler, see [Define Lambda function handler in Python](./python-handler.html).
###### Context methods
* `get\_remaining\_time\_in\_millis` – Returns the number of milliseconds left before the execution times out.
###### Context properties
* `function\_name` – The name of the Lambda function.
* `function\_version` – The [version](./configuration-versions.html) of the function.
* `invoked\_function\_arn` – The Amazon Resource Name (ARN) that's used to invoke the function. Indicates if the invoker
specified a version number or alias.
* `memory\_limit\_in\_mb` – The amount of memory that's allocated for the function.
* `aws\_request\_id` – The identifier of the invocation request.
* `log\_group\_name` – The log group for the function.
* `log\_stream\_name` – The log stream for the function instance.
* `identity` – (mobile apps) Information about the Amazon Cognito identity that authorized the request.
* `cognito\_identity\_id` – The authenticated Amazon Cognito identity.
* `cognito\_identity\_pool\_id` – The Amazon Cognito identity pool that authorized the invocation.
* `client\_context` – (mobile apps) Client context that's provided to Lambda by the client application.
* `client.installation\_id`
* `client.app\_title`
* `client.app\_version\_name`
* `client.app\_version\_code`
* `client.app\_package\_name`
* `custom` – A `dict` of custom values set by the mobile client
application.
* `env` – A `dict` of environment information provided by the AWS SDK.
Powertools for Lambda (Python) provides an interface definition for the Lambda context object. You can use the interface
definition for type hints, or to further inspect the structure of the Lambda context object. For the interface
definition, see [lambda\_context.py](https://github.com/aws-powertools/powertools-lambda-python/blob/develop/aws_lambda_powertools/utilities/typing/lambda_context.py) in the *powertools-lambda-python* repository on GitHub.
The following example shows a handler function that logs context information.
###### Example handler.py
```
`import time
def lambda\_handler(event, context):
print("Lambda function ARN:", context.invoked\_function\_arn)
print("CloudWatch log stream name:", context.log\_stream\_name)
print("CloudWatch log group name:", context.log\_group\_name)
print("Lambda Request ID:", context.aws\_request\_id)
print("Lambda function memory limits in MB:", context.memory\_limit\_in\_mb)
# We have added a 1 second delay so you can see the time remaining in get\_remaining\_time\_in\_millis.
time.sleep(1)
print("Lambda time remaining in MS:", context.get\_remaining\_time\_in\_millis())`
```
In addition to the options listed above, you can also use the AWS X-Ray SDK for [Instrumenting Python code in AWS Lambda](./python-tracing.html) to identify critical code paths, trace their performance and capture the data for
analysis.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Layers
Logging
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.