---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api-content-provision-and-consumption.html
title: Control
word_count: 252
filtered: true
elements_removed: 0
density_score: 0.82
---

Control access to API documentation in API Gateway - Amazon API Gateway
Control access to API documentation in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-documenting-api-content-provision-and-consumption)
# Control
access to API documentation in API Gateway
If you have a dedicated documentation team to write and edit your API documentation,
you can configure separate access permissions for your developers (for API development)
and for your writers or editors (for content development). This is especially
appropriate when a third-party vendor is involved in creating the documentation for you.
To grant your documentation team the access to create, update, and publish your API
documentation, you can assign the documentation team an IAM role with the following
IAM policy, where `account\_id` is the AWS account ID of
your documentation team.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "StmtDocPartsAddEditViewDelete",
"Effect": "Allow",
"Action": [
"apigateway:GET",
"apigateway:PUT",
"apigateway:POST",
"apigateway:PATCH",
"apigateway:DELETE"
],
"Resource": [
"arn:aws:apigateway:us-east-1:`111111111111`:/restapis/\*/documentation/\*"
]
}
]
}`
`
```
For information on setting permissions to access API Gateway resources, see [How Amazon API Gateway works with
IAM](./security_iam_service-with-iam.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Import API
documentation
SDK generation
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.