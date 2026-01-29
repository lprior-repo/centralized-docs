---
url: https://docs.aws.amazon.com/lambda/latest/dg/configuration-memory.html
title: Configure Lambda function memory
word_count: 764
filtered: true
elements_removed: 0
density_score: 0.88
---

Configure Lambda function memory - AWS Lambda
Configure Lambda function memory - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#configuration-memory)
[When to increase memory](#configuration-memory-use-cases)[Using the console](#configuration-memory-console)[Using the AWS CLI](#configuration-memory-cli)[Using AWS SAM](#configuration-memory-sam)[Accepting function memory recommendations (console)](#configuration-memory-optimization-accept)
# Configure Lambda function memory
Lambda allocates CPU power in proportion to the amount of memory configured. *Memory* is the amount of memory available to your Lambda function at runtime. You can increase or decrease the memory and CPU power allocated to your function using the **Memory** setting. You can configure memory between 128 MB and 10,240 MB in 1-MB increments. At 1,769 MB, a
function has the equivalent of one vCPU (one vCPU-second of credits per second).
This page describes how and when to update the memory setting for a Lambda function.
###### Sections
* [Determining the appropriate memory setting for a Lambda function](#configuration-memory-use-cases)
* [Configuring function memory (console)](#configuration-memory-console)
* [Configuring function memory (AWS CLI)](#configuration-memory-cli)
* [Configuring function memory (AWS SAM)](#configuration-memory-sam)
* [Accepting function memory recommendations (console)](#configuration-memory-optimization-accept)
## Determining the appropriate memory setting for a Lambda function
Memory is the principal lever for controlling the performance of a function. The default
setting, 128 MB, is the lowest possible setting. We recommend that you only use 128 MB for
simple Lambda functions, such as those that transform and route events to other AWS
services. A higher memory allocation can improve performance for functions that use imported
libraries, [Lambda layers](./chapter-layers.html), Amazon Simple Storage Service (Amazon S3) or Amazon Elastic File System
(Amazon EFS). Adding more memory proportionally increases the amount of CPU, increasing the overall
computational power available. If a function is CPU, network or memory-bound, then increasing
the memory setting can dramatically improve its performance.
To find the right memory configuration, monitor your functions with Amazon CloudWatch and set alarms
if memory consumption is approaching the configured maximums. This can help identify
memory-bound functions. For CPU-bound and IO-bound functions, monitoring the duration can
also provide insight. In these cases, increasing the memory can help resolve the compute or
network bottlenecks.
You can also consider using the open source
[AWS Lambda Power Tuning](https://github.com/alexcasalboni/aws-lambda-power-tuning) tool.
This tool uses AWS Step Functions to run multiple concurrent versions of a Lambda function at different
memory allocations and measure the performance. The input function runs in your AWS account,
performing live HTTP calls and SDK interaction, to measure likely performance in a live
production scenario. You can also implement a CI/CD process to use this tool to automatically
measure the performance of new functions that you deploy.
## Configuring function memory (console)
You can configure the memory of your function in the Lambda console.
###### To update the memory of a function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Choose the **Configuration** tab and then choose **General configuration**.
![The Configuration tab in the Lambda console.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/configuration-tab.png)
4. Under **General configuration**, choose **Edit**.
5. For **Memory**, set a value from 128 MB to 10,240 MB.
6. Choose **Save**.
## Configuring function memory (AWS CLI)
You can use the [update-function-configuration](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-function-configuration.html) command to configure the memory of your function.
```
`aws lambda update-function-configuration \\
--function-name `my-function` \\
--memory-size `1024``
```
## Configuring function memory (AWS SAM)
You can use the [AWS Serverless Application Model](<https://docs.aws.amazon.com//serverless-application-model/latest/developerguide/serverless-getting-started.html >) to configure memory for your function. Update the [MemorySize](https://docs.aws.amazon.com//serverless-application-model/latest/developerguide/sam-resource-function.html#sam-function-memorysize) property in your `template.yaml` file and then run [sam deploy](https://docs.aws.amazon.com//serverless-application-model/latest/developerguide/sam-cli-command-reference-sam-deploy.html).
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
MemorySize: `1024`
*# Other function properties...*`
```
## Accepting function memory recommendations (console)
If you have administrator permissions in AWS Identity and Access Management (IAM), you can opt in to receive Lambda function memory
setting recommendations from AWS Compute Optimizer. For instructions on opting in to memory recommendations for your account or
organization, see [Opting in your account](https://docs.aws.amazon.com/compute-optimizer/latest/ug/getting-started.html#account-opt-in) in the
*AWS Compute Optimizer User Guide*.
###### Note
Compute Optimizer supports only functions that use x86\_64 architecture.
When you've opted in and your [Lambda
function meets Compute Optimizer requirements](https://docs.aws.amazon.com/compute-optimizer/latest/ug/requirements.html#requirements-lambda-functions), you can view and accept function memory
recommendations from Compute Optimizer in the Lambda console in **General configuration**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Container images
Ephemeral storage
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.