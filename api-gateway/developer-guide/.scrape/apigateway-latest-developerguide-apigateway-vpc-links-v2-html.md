---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-vpc-links-v2.html
title: Set up VPC links V2 in API Gateway
word_count: 580
filtered: true
elements_removed: 0
density_score: 0.91
---

Set up VPC links V2 in API Gateway - Amazon API Gateway
Set up VPC links V2 in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-vpc-links-v2)
[Create a VPC link V2 by using the AWS CLI](#apigateway-vpc-links-v2-create)[Delete a VPC link V2 by using the AWS CLI](#apigateway-vpc-links-v2-delete)[Availability by Region](#apigateway-vpc-links-v2-availability)
# Set up VPC links V2 in API Gateway
VPC links enable you to create private integrations that connect your API routes to
private resources in a VPC, such as Application Load Balancers or Amazon ECS container-based applications.
A private integration uses a VPC link V2 to encapsulate connections between API Gateway and
targeted VPC resources. You can reuse VPC links across different resources and APIs.
When you create a VPC link, API Gateway creates and manages [elastic network
interfaces](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-eni.html) for the VPC link V2 in your account. This process can take a few
minutes. When a VPC link V2 is ready to use, its state transitions from `PENDING` to
`AVAILABLE`.
###### Note
If no traffic is sent over the VPC link for 60 days, it becomes `INACTIVE`.
When a VPC link is in an `INACTIVE` state, API Gateway deletes all of the VPC
link’s network interfaces. This causes API requests that depend on the VPC link to fail.
If API requests resume, API Gateway reprovisions network interfaces. It can take a few minutes
to create the network interfaces and reactivate the VPC link. You can use the VPC link
status to monitor the state of your VPC link.
## Create a VPC link V2 by using the AWS CLI
To create a VPC link V2, all resources involved must be owned by the same AWS account. The following [create-vpc-link](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-vpc-link.html) command creates a VPC link:
```
`aws apigatewayv2 create-vpc-link --name MyVpcLink \\
--subnet-ids subnet-aaaa subnet-bbbb \\
--security-group-ids sg1234 sg5678`
```
###### Note
VPC links V2 are immutable. After you create a VPC link V2, you can’t change its subnets or
security groups.
## Delete a VPC link V2 by using the AWS CLI
The following [delete-vpc-link](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/delete-vpc-link.html) command deletes a VPC link.
```
`aws apigatewayv2 delete-vpc-link --vpc-link-id `abcd123``
```
## Availability by Region
VPC links V2 are supported in the following Regions and Availability Zones:
|Region name|Region|Supported Availability Zones|
|US East (Ohio)|us-east-2|
use2-az1, use2-az2, use2-az3
|
|US East (N. Virginia)|us-east-1|
use1-az1, use1-az2, use1-az4, use1-az5, use1-az6
|
|US West (N. California)|us-west-1|
usw1-az1, usw1-az3
|
|US West (Oregon)|us-west-2|
usw2-az1, usw2-az2, usw2-az3, usw2-az4
|
|Asia Pacific (Hong Kong)|ap-east-1|
ape1-az2, ape1-az3
|
|Asia Pacific (Mumbai)|ap-south-1|
aps1-az1, aps1-az2, aps1-az3
|
|Asia Pacific (Seoul)|ap-northeast-2|
apne2-az1, apne2-az2, apne2-az3
|
|Asia Pacific (Singapore)|ap-southeast-1|
apse1-az1, apse1-az2, apse1-az3
|
|Asia Pacific (Sydney)|ap-southeast-2|
apse2-az1, apse2-az2, apse2-az3
|
|Asia Pacific (Tokyo)|ap-northeast-1|
apne1-az1, apne1-az2, apne1-az4
|
|Canada (Central)|ca-central-1|
cac1-az1, cac1-az2
|
|Europe (Frankfurt)|eu-central-1|
euc1-az1, euc1-az2, euc1-az3
|
|Europe (Ireland)|eu-west-1|
euw1-az1, euw1-az2, euw1-az3
|
|Europe (London)|eu-west-2|
euw2-az1, euw2-az2, euw2-az3
|
|Europe (Paris)|eu-west-3|
euw3-az1, euw3-az3
|
|Europe (Spain)|eu-south-2|eus2-az1, eus2-az2, eus2-az3|
|Europe (Stockholm)|eu-north-1|
eun1-az1, eun1-az2, eun1-az3
|
|Middle East (Bahrain)|me-south-1|
mes1-az1, mes1-az2, mes1-az3
|
|South America (São Paulo)|sa-east-1|
sae1-az1, sae1-az2, sae1-az3
|
|AWS GovCloud (US-West)|us-gov-west-1|
usgw1-az1, usgw1-az2, usgw1-az3
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Private integration
Set up a private integration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.