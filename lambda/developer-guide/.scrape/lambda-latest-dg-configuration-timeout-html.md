---
url: https://docs.aws.amazon.com/lambda/latest/dg/configuration-timeout.html
title: Configure Lambda function timeout
word_count: 558
filtered: true
elements_removed: 0
density_score: 0.88
---

Configure Lambda function timeout - AWS Lambda
Configure Lambda function timeout - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#configuration-timeout)
[When to increase timeout](#configuration-timeout-use-cases)[Using the console](#configuration-timeout-console)[Using the AWS CLI](#configuration-timeout-cli)[Using AWS SAM](#configuration-timeout-sam)
# Configure Lambda function timeout
Lambda runs your code for a set amount of time before timing out.
*Timeout* is the maximum amount of time in seconds that a Lambda function can run. The
default value for this setting is 3 seconds, but you can adjust this in increments of 1 second up to a maximum value
of 900 seconds (15 minutes).
This page describes how and when to update the timeout setting for a Lambda function.
###### Sections
* [Determining the appropriate timeout value for a Lambda function](#configuration-timeout-use-cases)
* [Configuring timeout (console)](#configuration-timeout-console)
* [Configuring timeout (AWS CLI)](#configuration-timeout-cli)
* [Configuring timeout (AWS SAM)](#configuration-timeout-sam)
## Determining the appropriate timeout value for a Lambda function
If the timeout value is close to the average duration of a function, there is a higher
risk that the function will time out unexpectedly. The duration of a function can vary
based on the amount of data transfer and processing, and the latency of any services the
function interacts with. Some common causes of timeout include:
* Downloads from Amazon Simple Storage Service (Amazon S3) are larger or take longer than average.
* A function makes a request to another service, which takes longer to respond.
* The parameters provided to a function require more computational complexity in the function, which causes the invocation to take longer.
When testing your application, ensure that your tests accurately reflect the size and quantity of data and realistic parameter values. Tests often use small samples for convenience, but you should use datasets at the upper bounds of what is reasonably expected for your workload.
## Configuring timeout (console)
You can configure function timeout in the Lambda console.
###### To modify the timeout for a function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Choose the **Configuration** tab and then choose **General configuration**.
![The Configuration tab in the Lambda console.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/configuration-tab.png)
4. Under **General configuration**, choose **Edit**.
5. For **Timeout**, set a value between 1 and
900 seconds (15 minutes).
6. Choose **Save**.
## Configuring timeout (AWS CLI)
You can use the [update-function-configuration](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-function-configuration.html) command to configure the timeout value, in seconds. The following example command increases the function timeout to 120 seconds (2 minutes).
```
`aws lambda update-function-configuration \\
--function-name `my-function` \\
--timeout `120``
```
## Configuring timeout (AWS SAM)
You can use the [AWS Serverless Application Model](<https://docs.aws.amazon.com//serverless-application-model/latest/developerguide/serverless-getting-started.html >) to configure the timeout value for your function. Update the [Timeout](https://docs.aws.amazon.com//serverless-application-model/latest/developerguide/sam-resource-function.html#sam-function-timeout) property in your `template.yaml` file and then run [sam deploy](https://docs.aws.amazon.com//serverless-application-model/latest/developerguide/sam-cli-command-reference-sam-deploy.html).
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
Timeout: `120`
*# Other function properties...*`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Instruction sets (ARM/x86)
Configure durable functions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.