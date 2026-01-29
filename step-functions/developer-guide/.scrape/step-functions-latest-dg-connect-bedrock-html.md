---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-bedrock.html
title: Invoke and customize Amazon Bedrock models with Step Functions
word_count: 1652
filtered: true
elements_removed: 0
density_score: 0.79
---

Invoke and customize Amazon Bedrock models with Step Functions - AWS Step Functions
Invoke and customize Amazon Bedrock models with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-bedrock)
[Service integration APIs](#connect-bedrock-custom-apis)[Task state definition](#connect-bedrock-task-definition)[IAM policies](#bedrock-iam)
# Invoke and customize Amazon Bedrock models with Step Functions
You can integrate Step Functions with Amazon Bedrock to invoke a specified Amazon Bedrock model and create a fine-tuning job to customize a model. This page lists the optimized Amazon Bedrock APIs and provides an example `Task` state to extract the result of a model invocation.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### Tip
To deploy an example workflow that integrates with Amazon Bedrock, see [Perform AI prompt-chaining with Amazon Bedrock](./sample-bedrock-prompt-chaining.html).
## Amazon Bedrock service integration APIs
To integrate AWS Step Functions with Amazon Bedrock, you can use the following APIs. These APIs are similar to the corresponding Amazon Bedrock APIs, except *InvokeModel* has additional request fields.
###### Amazon Bedrock API - [CreateModelCustomizationJob](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_CreateModelCustomizationJob.html)
Creates a fine-tuning job to customize a base model. You can invoke the Step Functions integration API with **CreateModelCustomizationJob** for *Request Response*, or **CreateModelCustomizationJob.sync** for *Run a Job (.sync)* integration patterns. There are no differences in the fields for the API calls.
###### Amazon Bedrock API - [InvokeModel](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModel.html)
Invokes the specified Amazon Bedrock model to run inference using the input you provide in the request body. You use `InvokeModel` to run inference for text models, image models, and embedding models.
The Amazon Bedrock service integration API request body for *InvokeModel* includes the following additional parameters.
* `Body` – Specifies input data in the format specified in the content-type request header. `Body` contains parameters specific to the target model.
If you use the `InvokeModel` API, you must specify the `Body` parameter. Step Functions doesn't validate the input you provide in `Body`.
When you specify `Body` using the Amazon Bedrock optimized integration, you can specify a payload of up to 256 KiB. If your payload exceeds 256 KiB, we recommend that you use `Input`.
* `Input` – Specifies the source to retrieve the input data from. This optional field is specific to Amazon Bedrock optimized integration with Step Functions. In this field, you can specify an `S3Uri`.
You can specify either `Body` in the Parameters or `Input`, but not both.
When you specify `Input` without specifying `ContentType`, the content type of the input data source becomes the value for `ContentType`.
* `Output` – Specifies the destination where the API response is written. This optional field is specific to Amazon Bedrock optimized integration with Step Functions. In this field, you can specify an `S3Uri`.
If you specify this field, the API response body is replaced with a reference to the Amazon S3 location of the original output.
The following example shows the syntax for InvokeModel API for Amazon Bedrock integration.
```
`{
"ModelId": String, // required
"Accept": String, // default: application/json
"ContentType": String, // default: application/json
"Input": { // not from Bedrock API
"S3Uri": String
},
"Output": { // not from Bedrock API
"S3Uri": String
}
}`
```
## Task state definition for Amazon Bedrock integration
The following Task state definition shows how you can integrate with Amazon Bedrock in your state machines. This example shows a Task state that extracts the full result of model invocation specified by the path, `result\_one`. This is based on [Inference parameters for foundation models](https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters.html). This example uses the Cohere Command large language model (LLM).
```
`{
"Type": "Task",
"Resource": "arn:aws:states:::bedrock:invokeModel",
"Arguments": {
"ModelId": "cohere.command-text-v14",
"Body": {
"prompt": "{% states.input.prompt\_one %}",
"max\_tokens": 20
},
"ContentType": "application/json",
"Accept": "\*/\*"
},
"End": true
}`
```
## IAM policies for calling Amazon Bedrock
When you create a state machine using the console, Step Functions automatically creates an execution role for your state machine with the least privileges required. These automatically generated IAM roles are valid for the AWS Region in which you create the state machine.
We recommend that when you create IAM policies, do not include wildcards in the policies. As a security best practice, you should scope your policies down as much as possible. You should use dynamic policies only when certain input parameters are not known during runtime.
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
### IAM policy examples for Amazon Bedrock integration
The following section describes the IAM permissions you need based on the Amazon Bedrock API that you use for a specific foundation or provisioned model. This section also contains examples of policies that grant full access.
Remember to replace the `italicized` text with your resource-specific information.
* [IAM policy example to access a specific foundation model using InvokeModel](#bedrock-policy-invoke-foundation-model)
* [IAM policy example to access a specific provisioned model using InvokeModel](#bedrock-policy-invoke-provisioned-model)
* [Full access IAM policy example to use InvokeModel](#bedrock-policy-invokemodel-full-access)
* [IAM policy example to access a specific foundation model as a base model](#bedrock-policy-foundation-model)
* [IAM policy example to access a specific custom model as a base model](#bedrock-policy-custom-model)
* [Full access IAM policy example to use CreateModelCustomizationJob.sync](#bedrock-policy-createmodel-full-access)
* [IAM policy example to access a specific foundation model using CreateModelCustomizationJob.sync](#bedrock-policy-createmodel-sync-foundation-model)
* [IAM policy example to access a custom model using CreateModelCustomizationJob.sync](#bedrock-policy-createmodel-sync-custom-model)
* [Full access IAM policy example to use CreateModelCustomizationJob.sync](#bedrock-policy-createmodel-sync-full-access)
#### IAM policy example to access a specific foundation model using InvokeModel
The following is an IAM policy example for a state machine that accesses a specific foundation model named `amazon.titan-text-express-v1` using the [InvokeModel](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModel.html) API action.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Sid": "InvokeModel1",
"Action": [
"bedrock:InvokeModel"
],
"Resource": [
"arn:aws:bedrock:`us-east-1`::foundation-model/`amazon.titan-text-express-v1`"
]
}
]
}`
`
```
#### IAM policy example to access a specific provisioned model using InvokeModel
The following is an IAM policy example for a state machine that accesses a specific provisioned model named `c2oi931ulksx` using the [InvokeModel](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModel.html) API action.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Sid": "InvokeModel1",
"Action": [
"bedrock:InvokeModel"
],
"Resource": [
"arn:aws:bedrock:`us-east-1`:`123456789012`:provisioned-model/`c2oi931ulksx`"
]
}
]
}`
`
```
#### Full access IAM policy example to use InvokeModel
The following is an IAM policy example for a state machine that provides full access when you use the [InvokeModel](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModel.html) API action.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Sid": "InvokeModel1",
"Action": [
"bedrock:InvokeModel"
],
"Resource": [
"arn:aws:bedrock:`us-east-1`::foundation-model/\*",
"arn:aws:bedrock:`us-east-1`:`123456789012`:provisioned-model/\*"
]
}
]
}`
`
```
#### IAM policy example to access a specific foundation model as a base model
The following is an IAM policy example for a state machine to access a specific foundation model named `amazon.titan-text-express-v1` as a base model using the [CreateModelCustomizationJob](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_CreateModelCustomizationJob.html) API action.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob1",
"Action": [
"bedrock:CreateModelCustomizationJob"
],
"Resource": [
"arn:aws:bedrock:`us-east-1`::foundation-model/`amazon.titan-text-express-v1`",
"arn:aws:bedrock:`us-east-1`:`123456789012`:custom-model/\*",
"arn:aws:bedrock:`us-east-1`:`123456789012`:model-customization-job/\*"
]
},
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob2",
"Action": [
"iam:PassRole"
],
"Resource": [
"arn:aws:iam::`123456789012`:role/`myRole`"
]
}
]
}`
`
```
#### IAM policy example to access a specific custom model as a base model
The following is an IAM policy example for a state machine to access a specific custom model as a base model using the [CreateModelCustomizationJob](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_CreateModelCustomizationJob.html) API action.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob1",
"Action": [
"bedrock:CreateModelCustomizationJob"
],
"Resource": [
"arn:aws:bedrock:`us-east-1`:`123456789012`:custom-model/\*",
"arn:aws:bedrock:`us-east-1`:`123456789012`:model-customization-job/\*"
]
},
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob2",
"Action": [
"iam:PassRole"
],
"Resource": [
"arn:aws:iam::`123456789012`:role/myRoleName"
]
}
]
}`
`
```
#### Full access IAM policy example to use CreateModelCustomizationJob.sync
The following is an IAM policy example for a state machine that provides full access when you use the [CreateModelCustomizationJob](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_CreateModelCustomizationJob.html) API action.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob1",
"Action": [
"bedrock:CreateModelCustomizationJob"
],
"Resource": [
"arn:aws:bedrock:`us-east-1`::foundation-model/\*",
"arn:aws:bedrock:`us-east-1`:`123456789012`:custom-model/\*",
"arn:aws:bedrock:`us-east-1`:`123456789012`:model-customization-job/\*"
]
},
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob2",
"Action": [
"iam:PassRole"
],
"Resource": [
"arn:aws:iam::`123456789012`:role/`myRole`"
]
}
]
}`
`
```
#### IAM policy example to access a specific foundation model using CreateModelCustomizationJob.sync
The following is an IAM policy example for a state machine to access a specific foundation model named `amazon.titan-text-express-v1` using the [CreateModelCustomizationJob.sync](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_CreateModelCustomizationJob.html) API action.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob1",
"Action": [
"bedrock:CreateModelCustomizationJob"
],
"Resource": [
"arn:aws:bedrock:`us-east-1`::foundation-model/amazon.titan-text-express-v1",
"arn:aws:bedrock:`us-east-1`:`123456789012`:custom-model/\*",
"arn:aws:bedrock:`us-east-1`:`123456789012`:model-customization-job/\*"
]
},
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob2",
"Action": [
"bedrock:GetModelCustomizationJob",
"bedrock:StopModelCustomizationJob"
],
"Resource": [
"arn:aws:bedrock:`us-east-1`:`123456789012`:model-customization-job/\*"
]
},
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob3",
"Action": [
"iam:PassRole"
],
"Resource": [
"arn:aws:iam::`123456789012`:role/`myRole`"
]
}
]
}`
`
```
#### IAM policy example to access a custom model using CreateModelCustomizationJob.sync
The following is an IAM policy example for a state machine to access a custom model using the [CreateModelCustomizationJob.sync](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_CreateModelCustomizationJob.html) API action.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob1",
"Action": [
"bedrock:CreateModelCustomizationJob"
],
"Resource": [
"arn:aws:bedrock:`us-east-1`:`123456789012`:custom-model/\*",
"arn:aws:bedrock:`us-east-1`:`123456789012`:model-customization-job/\*"
]
},
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob2",
"Action": [
"bedrock:GetModelCustomizationJob",
"bedrock:StopModelCustomizationJob"
],
"Resource": [
"arn:aws:bedrock:`us-east-1`:`123456789012`:model-customization-job/\*"
]
},
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob3",
"Action": [
"iam:PassRole"
],
"Resource": [
"arn:aws:iam::`123456789012`:role/`myRole`"
]
}
]
}`
`
```
#### Full access IAM policy example to use CreateModelCustomizationJob.sync
The following is an IAM policy example for a state machine that provides full access when you use the [CreateModelCustomizationJob.sync](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_CreateModelCustomizationJob.html) API action.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob1",
"Action": [
"bedrock:CreateModelCustomizationJob"
],
"Resource": [
"arn:aws:bedrock:`us-east-1`::foundation-model/\*",
"arn:aws:bedrock:`us-east-1`:`123456789012`:custom-model/\*",
"arn:aws:bedrock:`us-east-1`:`123456789012`:model-customization-job/\*"
]
},
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob2",
"Action": [
"bedrock:GetModelCustomizationJob",
"bedrock:StopModelCustomizationJob"
],
"Resource": [
"arn:aws:bedrock:`us-east-1`:`123456789012`:model-customization-job/\*"
]
},
{
"Effect": "Allow",
"Sid": "CreateModelCustomizationJob3",
"Action": [
"iam:PassRole"
],
"Resource": [
"arn:aws:iam::`123456789012`:role/`myRole`"
]
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
AWS Batch
AWS CodeBuild
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.