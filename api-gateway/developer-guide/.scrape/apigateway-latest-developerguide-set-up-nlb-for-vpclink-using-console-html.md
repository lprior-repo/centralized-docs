---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-nlb-for-vpclink-using-console.html
title: Set up a Network Load Balancer
word_count: 528
filtered: true
elements_removed: 0
density_score: 0.84
---

Set up a Network Load Balancer for API Gateway private integrations (legacy) - Amazon API Gateway
Set up a Network Load Balancer for API Gateway private integrations (legacy) - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#set-up-nlb-for-vpclink-using-console)
# Set up a Network Load Balancer
for API Gateway private integrations (legacy)
###### Note
The following implementation of private integrations uses VPC links V1. VPC links V1 are legacy resources. We recommend
that you use [VPC links V2 for REST APIs](./apigateway-vpc-links-v2.html).
The following procedure outlines the steps to set up a Network Load Balancer (NLB)
for API Gateway private integrations using the Amazon EC2 console and provides references for
detailed instructions for each step.
For each VPC you have resources in, you only need to configure one NLB and one VPCLink. The NLB supports
multiple [listeners](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-listeners.html) and [target
groups](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/load-balancer-target-groups.html) per NLB. You can configure each service as a specific listener on the NLB and use a single
VPCLink to connect to the NLB. When creating the private integration in API Gateway you then define each service using
the specific port that is assigned for each service. For more information, see [Tutorial: Create a REST API with a private integration](./getting-started-with-private-integration.html). The Network Load Balancer and API must be owned by the
same AWS account.
###### To create a Network Load Balancer for private integration using the API Gateway
console
1. Sign in to the AWS Management Console and open the Amazon EC2 console at
[https://console.aws.amazon.com/ec2/](https://console.aws.amazon.com/ec2/).
2. Set up a web server on an Amazon EC2 instance. For an example setup, see [Installing a LAMP Web Server on Amazon Linux 2](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-lamp-amazon-linux-2.html).
3. Create a Network Load Balancer, register the EC2 instance with a target
group, and add the target group to a listener of the Network Load Balancer. For
details, follow the instructions in [Getting Started with Network Load Balancers](https://docs.aws.amazon.com/elasticloadbalancing/latest/network/network-load-balancer-getting-started.html).
4. After the Network Load Balancer is created, do the following:
1.
Note the ARN of the Network Load Balancer. You will need it to
create a VPC link in API Gateway for integrating the API with the VPC resources behind
the Network Load Balancer.
2. Turn off security group evaluation for PrivateLink.
* To turn off security group evaluation for PrivateLink traffic using the console, you can choose the
**Security** tab, and then **Edit**. In the **Security
settings**, clear **Enforce inbound rules on PrivateLink traffic**.
* Use the following [set-security-groups](https://docs.aws.amazon.com/cli/latest/reference/elbv2/set-security-groups.html) command to turn
off security group evaluation for PrivateLink traffic:
```
`aws elbv2 set-security-groups --load-balancer-arn `arn:aws:elasticloadbalancing:us-east-2:111122223333:loadbalancer/net/my-loadbalancer/abc12345` \\
--security-groups `sg-123345a` --enforce-security-group-inbound-rules-on-private-link-traffic off `
```
###### Note
Do not add any dependencies to API Gateway CIDRs as they are bound to change without notice.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Private integration using VPC links V1 (legacy)
Grant permissions for API Gateway to create a VPC
link (legacy)
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.