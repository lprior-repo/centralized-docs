---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/vpc-links-v1.html
title: vpc links v1.html
word_count: 310
filtered: true
elements_removed: 0
density_score: 0.86
---

Private integration using VPC links V1 (legacy) - Amazon API Gateway
Private integration using VPC links V1 (legacy) - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#vpc-links-v1)
###### Note
The following implementation of private integrations uses VPC links V1. VPC links V1 are legacy resources. We recommend
that you use [VPC links V2 for REST APIs](./apigateway-vpc-links-v2.html).
To create a private integration, you must first create a Network Load Balancer. Your Network Load Balancer must have a [listener](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-listeners.html)
that routes requests to resources in your VPC. To improve the availability of your API, ensure that your Network Load Balancer
routes traffic to resources in more than one Availability Zone in the AWS Region. Then, you create a VPC link
that you use to connect your API and your Network Load Balancer. After you create a VPC link, you create private integrations to
route traffic from your API to resources in your VPC through your VPC link and Network Load Balancer. The Network Load Balancer and
API must be owned by the same AWS account.
###### Topics
* [Set up a Network Load Balancer
for API Gateway private integrations (legacy)](./set-up-nlb-for-vpclink-using-console.html)
* [Grant permissions for API Gateway to create a VPC
link (legacy)](./grant-permissions-to-create-vpclink.html)
* [Set up an API Gateway API with private
integrations using the AWS CLI (legacy)](./set-up-api-with-vpclink-cli.html)
* [API Gateway accounts used for private integrations (legacy)](./set-up-api-with-vpclink-accounts.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up a private integration
Set up a Network Load Balancer for private integrations (legacy)
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.