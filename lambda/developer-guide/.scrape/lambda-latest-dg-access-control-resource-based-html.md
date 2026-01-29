---
url: https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html
title: Viewing resource-based IAM policies in Lambda
word_count: 477
filtered: true
elements_removed: 0
density_score: 0.88
---

Viewing resource-based IAM policies in Lambda - AWS Lambda
Viewing resource-based IAM policies in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#access-control-resource-based)
[Supported API actions](#permissions-resource-api)
# Viewing resource-based IAM policies in Lambda
Lambda supports resource-based permissions policies for Lambda functions and layers. You can use resource-based policies to grant access to other [AWS accounts](./permissions-function-cross-account.html), [organizations](./permissions-function-organization.html), or [services](./permissions-function-services.html). Resource-based policies apply to a single function, version, alias, or layer version.
Console
###### To view a function's resource-based policy
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Choose **Configuration** and then choose **Permissions**.
4. Scroll down to **Resource-based policy** and then choose **View policy document**. The resource-based policy shows the permissions that are applied when another account or AWS service
attempts to access the function. The following example shows a statement that allows Amazon S3 to invoke a function
named `my-function` for a bucket named `amzn-s3-demo-bucket` in account
`123456789012`.
###### Example resource-based policy
****
```
``{
"Version":"2012-10-17",
"Id": "default",
"Statement": [
{
"Sid": "lambda-allow-s3-my-function",
"Effect": "Allow",
"Principal": {
"Service": "s3.amazonaws.com"
},
"Action": "lambda:InvokeFunction",
"Resource": "arn:aws:lambda:us-east-2:123456789012:function:my-function",
"Condition": {
"StringEquals": {
"AWS:SourceAccount": "123456789012"
},
"ArnLike": {
"AWS:SourceArn": "arn:aws:s3:::amzn-s3-demo-bucket"
}
}
}
]
}`
`
```
AWS CLI
To view a function's resource-based policy, use the `get-policy` command.
```
``aws lambda get-policy \\
--function-name my-function \\
--output text``
```
You should see the following output:
****
```
``{"Version":"2012-10-17", "Id":"default","Statement":[{"Sid":"sns","Effect":"Allow","Principal":{"Service":"s3.amazonaws.com"},"Action":"lambda:InvokeFunction","Resource":"arn:aws:lambda:us-east-2:123456789012:function:my-function","Condition":{"ArnLike":{"AWS:SourceArn":"arn:aws:sns:us-east-2:123456789012:lambda\*"}}}]}`
`
```
For versions and aliases, append the version number or alias to the function name.
```
``aws lambda get-policy --function-name my-function:PROD``
```
To remove permissions from your function, use `remove-permission`.
```
``aws lambda remove-permission \\
--function-name example \\
--statement-id sns``
```
Use the `get-layer-version-policy` command to view the permissions on a layer.
```
``aws lambda get-layer-version-policy \\
--layer-name my-layer \\
--version-number 3 \\
--output text``
```
You should see the following output:
```
b0cd9796-d4eb-4564-939f-de7fe0b42236 {"Sid":"engineering-org","Effect":"Allow","Principal":"\*","Action":"lambda:GetLayerVersion","Resource":"arn:aws:lambda:us-west-2:123456789012:layer:my-layer:3","Condition":{"StringEquals":{"aws:PrincipalOrgID":"o-t194hfs8cz"}}}"
```
Use `remove-layer-version-permission` to remove statements from the policy.
```
``aws lambda remove-layer-version-permission --layer-name my-layer --version-number 3 --statement-id engineering-org``
```
## Supported API actions
The following Lambda API actions support resource-based policies:
* [CreateAlias](https://docs.aws.amazon.com/lambda/latest/api/API_CreateAlias.html)
* [DeleteAlias](https://docs.aws.amazon.com/lambda/latest/api/API_DeleteAlias.html)
* [DeleteFunction](https://docs.aws.amazon.com/lambda/latest/api/API_DeleteFunction.html)
* [DeleteFunctionConcurrency](https://docs.aws.amazon.com/lambda/latest/api/API_DeleteFunctionConcurrency.html)
* [DeleteFunctionEventInvokeConfig](https://docs.aws.amazon.com/lambda/latest/api/API_DeleteFunctionEventInvokeConfig.html)
* [DeleteProvisionedConcurrencyConfig](https://docs.aws.amazon.com/lambda/latest/api/API_DeleteProvisionedConcurrencyConfig.html)
* [GetAlias](https://docs.aws.amazon.com/lambda/latest/api/API_GetAlias.html)
* [GetFunction](https://docs.aws.amazon.com/lambda/latest/api/API_GetFunction.html)
* [GetFunctionConcurrency](https://docs.aws.amazon.com/lambda/latest/api/API_GetFunctionConcurrency.html)
* [GetFunctionConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_GetFunctionConfiguration.html)
* [GetFunctionEventInvokeConfig](https://docs.aws.amazon.com/lambda/latest/api/API_GetFunctionEventInvokeConfig.html)
* [GetPolicy](https://docs.aws.amazon.com/lambda/latest/api/API_GetPolicy.html)
* [GetProvisionedConcurrencyConfig](https://docs.aws.amazon.com/lambda/latest/api/API_GetProvisionedConcurrencyConfig.html)
* [Invoke](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html)
* [InvokeFunctionUrl](./urls-auth.html) (permission only)
* [ListAliases](https://docs.aws.amazon.com/lambda/latest/api/API_ListAliases.html)
* [ListFunctionEventInvokeConfigs](https://docs.aws.amazon.com/lambda/latest/api/API_ListFunctionEventInvokeConfigs.html)
* [ListProvisionedConcurrencyConfigs](https://docs.aws.amazon.com/lambda/latest/api/API_ListProvisionedConcurrencyConfigs.html)
* [ListTags](https://docs.aws.amazon.com/lambda/latest/api/API_ListTags.html)
* [ListVersionsByFunction](https://docs.aws.amazon.com/lambda/latest/api/API_ListVersionsByFunction.html)
* [PublishVersion](https://docs.aws.amazon.com/lambda/latest/api/API_PublishVersion.html)
* [PutFunctionConcurrency](https://docs.aws.amazon.com/lambda/latest/api/API_PutFunctionConcurrency.html)
* [PutFunctionEventInvokeConfig](https://docs.aws.amazon.com/lambda/latest/api/API_PutFunctionEventInvokeConfig.html)
* [PutProvisionedConcurrencyConfig](https://docs.aws.amazon.com/lambda/latest/api/API_PutProvisionedConcurrencyConfig.html)
* [TagResource](https://docs.aws.amazon.com/lambda/latest/api/API_TagResource.html)
* [UntagResource](https://docs.aws.amazon.com/lambda/latest/api/API_UntagResource.html)
* [UpdateAlias](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateAlias.html)
* [UpdateFunctionCode](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionCode.html)
* [UpdateFunctionEventInvokeConfig](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionEventInvokeConfig.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Layer access
Function access for AWS services
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.