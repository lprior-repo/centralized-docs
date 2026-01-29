---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-iam-policy-examples-for-api-execution.html
title: IAM policy
word_count: 264
filtered: true
elements_removed: 0
density_score: 0.84
---

IAM policy examples for API execution permissions - Amazon API Gateway
IAM policy examples for API execution permissions - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-iam-policy-examples-for-api-execution)
# IAM policy
examples for API execution permissions
For permissions model and other background information, see [Control
access for invoking an API](./api-gateway-control-access-using-iam-policies-to-invoke-api.html).
The following policy statement gives the user permission to call any POST method along
the path of `mydemoresource`, in the stage of
`test`, for the API with the identifier of `a123456789`, assuming the corresponding API has been deployed to the AWS
region of us-east-1:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"execute-api:Invoke"
],
"Resource": [
"arn:aws:execute-api:us-east-1:\*:a123456789/test/POST/my-demo-resource-path/\*"
]
}
]
}`
`
```
The following example policy statement gives the user permission to call any method on
the resource path of `petstorewalkthrough/pets`, in any stage, for the
API with the identifier of `a123456789`, in any AWS
region where the corresponding API has been deployed:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"execute-api:Invoke"
],
"Resource": [
"arn:aws:execute-api:\*:\*:a123456789/\*/\*/petstorewalkthrough/pets"
]
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Control
access for invoking an API
Use VPC endpoint policies for private APIs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.