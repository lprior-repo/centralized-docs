---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-supported-runtimes.html
title: Supported runtimes for durable functions
word_count: 639
filtered: true
elements_removed: 0
density_score: 0.90
---

Supported runtimes for durable functions - AWS Lambda
Supported runtimes for durable functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-supported-runtimes)
[Lambda managed runtimes](#durable-managed-runtimes)[Container images](#durable-container-images)[Runtime considerations](#durable-runtime-considerations)
# Supported runtimes for durable functions
Durable functions are available for Node.js and Python runtimes. You can create durable functions using managed runtimes in the Lambda console or deploy them using container images for additional runtime version flexibility.
## Lambda managed runtimes
The following managed runtimes support durable functions when you create functions in the Lambda console or using the AWS CLI with the `--durable-config '{"ExecutionTimeout": 3600, "RetentionPeriodInDays": 7}'` parameter. For complete information about Lambda runtimes, see [Lambda runtimes](./lambda-runtimes.html).
|Language|Runtime|
|Node.js|nodejs22.x|
|Node.js|nodejs24.x|
|Python|python3.13|
|Python|python3.14|
###### Note
Lambda runtimes include the durable execution SDK for testing and development. However, we recommend including the SDK in your deployment package for production. This ensures version consistency and avoids potential runtime updates that might affect your function behavior.
### Node.js
Install the SDK in your Node.js project:
```
`
npm install @aws/durable-execution-sdk-js
`
```
The SDK supports JavaScript and TypeScript. For TypeScript projects, the SDK includes type definitions.
### Python
Install the SDK in your Python project:
```
`
pip install aws-durable-execution-sdk-python
`
```
The Python SDK uses synchronous methods and doesn't require `async/await`.
## Container images
You can use durable functions with container images to support additional runtime versions or custom runtime configurations. Container images let you use runtime versions not available as managed runtimes or customize your runtime environment.
To create a durable function using a container image:
1. Create a Dockerfile based on an Lambda base image
2. Install the durable execution SDK in your container
3. Build and push the container image to Amazon Elastic Container Registry
4. Create the Lambda function from the container image with durable execution enabled
### Python container example
Create a Dockerfile for Python 3.11:
```
`
FROM public.ecr.aws/lambda/python:3.11
# Install dependencies including durable SDK
RUN pip install -r requirements.txt
# Set the handler
CMD [ "lambda\_function.handler" ]
`
```
Create a `requirements.txt` file:
```
`
aws-durable-execution-sdk-python
`
```
Build and push the image:
```
`
# Push to ECR
docker push 123456789012.dkr.ecr.us-east-1.amazonaws.com/my-durable-function:latest
`
```
Create the function with durable execution enabled:
```
`
aws lambda create-function \\
--function-name myDurableFunction \\
--package-type Image \\
--code ImageUri=123456789012.dkr.ecr.us-east-1.amazonaws.com/my-durable-function:latest \\
--role arn:aws:iam::123456789012:role/lambda-execution-role \\
--durable-config '{"ExecutionTimeout": 3600, "RetentionPeriodInDays": 7}'
`
```
For more information about using container images with Lambda, see [Creating Lambda container images](https://docs.aws.amazon.com/lambda/latest/dg/images-create.html) in the Lambda Developer Guide.
## Runtime considerations
**SDK version management:** Include the durable execution SDK in your deployment package or container image. This ensures your function uses a specific SDK version and isn't affected by runtime updates. Pin SDK versions in your `package.json` or `requirements.txt` to control when you upgrade.
**Runtime updates:** AWS updates managed runtimes to include security patches and bug fixes. These updates may include new SDK versions. To avoid unexpected behavior, include the SDK in your deployment package and test thoroughly before deploying to production.
**Container image size:** Container images have a maximum uncompressed size of 10 GB. The durable execution SDK adds minimal size to your image. Optimize your container by using multi-stage builds and removing unnecessary dependencies.
**Cold start performance:** Container images may have longer cold start times than managed runtimes. The durable execution SDK has minimal impact on cold start performance. Use provisioned concurrency if cold start latency is critical for your application.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Durable execution SDK
Invoking durable functions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.