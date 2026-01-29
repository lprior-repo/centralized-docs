---
url: https://docs.aws.amazon.com/lambda/latest/dg/ruby-context.html
title: Using the Lambda context object to retrieve Ruby function information
word_count: 287
filtered: true
elements_removed: 0
density_score: 0.90
---

Using the Lambda context object to retrieve Ruby function information - AWS Lambda
Using the Lambda context object to retrieve Ruby function information - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#ruby-context)
# Using the Lambda context object to retrieve Ruby function information
When Lambda runs your function, it passes a context object to the [handler](./ruby-handler.html).
This object provides methods and properties that provide information about the invocation, function, and execution
environment.
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
* `deadline\_ms`– The date that the execution times out, in Unix time milliseconds.
* `identity` – (mobile apps) Information about the Amazon Cognito identity that authorized the request.
* `client\_context`– (mobile apps) Client context that's provided to Lambda by the client application.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Layers
Logging
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.