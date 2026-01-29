---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/grant-permissions-to-create-vpclink.html
title: Grant permissions for API Gateway to create a VPC
word_count: 343
filtered: true
elements_removed: 0
density_score: 0.75
---

Grant permissions for API Gateway to create a VPC link (legacy) - Amazon API Gateway
Grant permissions for API Gateway to create a VPC link (legacy) - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#grant-permissions-to-create-vpclink)
# Grant permissions for API Gateway to create a VPC
link (legacy)
###### Note
The following implementation of private integrations uses VPC links V1. VPC links V1 are legacy resources. We recommend
that you use [VPC links V2 for REST APIs](./apigateway-vpc-links-v2.html).
For you or a user in your account to create and maintain a VPC link, you or the user
must have permissions to create, delete, and view VPC endpoint service configurations,
change VPC endpoint service permissions, and examine load balancers. To grant such
permissions, use the following steps.
###### To grant permissions to create, update, and delete a VPC link
1. Create an IAM policy similar to the following:
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"apigateway:POST",
"apigateway:GET",
"apigateway:PATCH",
"apigateway:DELETE"
],
"Resource": [
"arn:aws:apigateway:`us-east-1`::/vpclinks",
"arn:aws:apigateway:`us-east-1`::/vpclinks/\*"
]
},
{
"Effect": "Allow",
"Action": [
"elasticloadbalancing:DescribeLoadBalancers"
],
"Resource": "\*"
},
{
"Effect": "Allow",
"Action": [
"ec2:CreateVpcEndpointServiceConfiguration",
"ec2:DeleteVpcEndpointServiceConfigurations",
"ec2:DescribeVpcEndpointServiceConfigurations",
"ec2:ModifyVpcEndpointServicePermissions"
],
"Resource": "\*"
}
]
}`
`
```
If you want to enable tagging for your VPC link, make sure to allow tagging operations. For more
information, see [Allow tagging operations](./apigateway-tagging-iam-policy.html#allow-tagging).
2. Create or choose an IAM role and attach the preceding policy to the
role.
3. Assign the IAM role to you or a user in your account who is creating VPC
links.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up a Network Load Balancer for private integrations (legacy)
Set up an API with private integrations using AWS CLI (legacy)
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.