---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-samples.html
title: Lambda sample applications
word_count: 746
filtered: true
elements_removed: 0
density_score: 0.92
---

Lambda sample applications - AWS Lambda
Lambda sample applications - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-samples)
# Lambda sample applications
The GitHub repository for this guide includes sample applications that demonstrate the use
of various languages and AWS services. Each sample application includes scripts for easy
deployment and cleanup and supporting resources.
Node.js
###### Sample Lambda applications in Node.js
* [blank-nodejs](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/blank-nodejs) – A Node.js function
that shows the use of logging, environment variables, AWS X-Ray tracing, layers, unit tests and the AWS
SDK.
* [nodejs-apig](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/nodejs-apig) – A function with a
public API endpoint that processes an event from API Gateway and returns an HTTP response.
Python
###### Sample Lambda applications in Python
* [blank-python](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/blank-python) – A Python function
that shows the use of logging, environment variables, AWS X-Ray tracing, layers, unit tests and the AWS SDK.
Ruby
###### Sample Lambda applications in Ruby
* [blank-ruby](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/blank-ruby) – A Ruby function that
shows the use of logging, environment variables, AWS X-Ray tracing, layers, unit tests and the AWS SDK.
* [Ruby Code Samples for AWS Lambda](https://docs.aws.amazon.com/code-samples/latest/catalog/code-catalog-ruby-example_code-lambda.html) – Code samples written in Ruby that demonstrate how to interact with AWS Lambda.
Java
###### Sample Lambda applications in Java
* [example-java](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/example-java) – A Java function that
demonstrates how you can use Lambda to process orders. This function illustrates how to define and
deserialize a custom input event object, use the AWS SDK, and output logging.
* [java-basic](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/java-basic) – A collection of minimal Java functions
with unit tests and variable logging configuration.
* [java-events](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/java-events) – A collection of Java functions that
contain skeleton code for how to handle events from various services such as Amazon API Gateway, Amazon SQS, and Amazon Kinesis.
These functions use the latest version of the [aws-lambda-java-events](./java-package.html)
library (3.0.0 and newer). These examples do not require the AWS SDK as a dependency.
* [s3-java](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/s3-java) – A Java function that processes
notification events from Amazon S3 and uses the Java Class Library (JCL) to create thumbnails from uploaded image
files.
* [layer-java](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/layer-java) – A Java function that illustrates
how to use a Lambda layer to package dependencies separate from your core function code.
###### Running popular Java frameworks on Lambda
* [spring-cloud-function-samples](https://github.com/spring-cloud/spring-cloud-function/tree/3.2.x/spring-cloud-function-samples/function-sample-aws) – An example from Spring that shows how to use the [Spring Cloud Function](https://spring.io/projects/spring-cloud-function) framework to create AWS Lambda functions.
* [Serverless Spring Boot Application Demo](https://github.com/aws-samples/serverless-java-frameworks-samples/tree/main/springboot) – An example that shows how to set up a typical Spring Boot application in a managed Java runtime with and without SnapStart, or as a GraalVM native image with a custom runtime.
* [Serverless Micronaut Application Demo](https://github.com/aws-samples/serverless-java-frameworks-samples/tree/main/micronaut) – An example that shows how to use Micronaut in a managed Java runtime with and without SnapStart, or as a GraalVM native image with a custom runtime. Learn more in the [Micronaut/Lambda guides](https://guides.micronaut.io/latest/tag-lambda.html).
* [Serverless Quarkus Application Demo](https://github.com/aws-samples/serverless-java-frameworks-samples/tree/main/quarkus) – An example that shows how to use Quarkus in a managed Java runtime with and without SnapStart, or as a GraalVM native image with a custom runtime. Learn more in the [Quarkus/Lambda guide](https://quarkus.io/guides/aws-lambda) and [Quarkus/SnapStart guide](https://quarkus.io/guides/aws-lambda-snapstart).
Go
Lambda provides the following sample applications for the Go runtime:
###### Sample Lambda applications in Go
* [go-al2](https://github.com/aws-samples/sessions-with-aws-sam/tree/master/go-al2) – A hello world function that returns the public IP address. This app uses the `provided.al2` custom runtime.
* [blank-go](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/blank-go) – A Go function that shows the use of Lambda's Go libraries, logging, environment variables, and the AWS SDK. This app uses the `go1.x` runtime.
C#
###### Sample Lambda applications in C#
* [blank-csharp](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/blank-csharp) – A C# function that
shows the use of Lambda's .NET libraries, logging, environment variables, AWS X-Ray tracing, unit tests, and the
AWS SDK.
* [blank-csharp-with-layer](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/blank-csharp-with-layer) – A C# function that
uses the .NET CLI to create a layer that packages the function's dependencies.
* [ec2-spot](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/ec2-spot) – A function that manages spot
instance requests in Amazon EC2.
PowerShell
Lambda provides the following sample applications for PowerShell:
* [blank-powershell](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/blank-powershell) – A PowerShell function that shows the use of logging, environment variables, and the AWS SDK.
To deploy a sample application, follow the instructions in its README file.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Networking
Working with AWS SDKs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.