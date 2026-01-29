---
url: https://docs.aws.amazon.com/lambda/latest/dg/configuration-vpc-endpoints.html
title: Connecting inbound interface VPC endpoints for Lambda
word_count: 1141
filtered: true
elements_removed: 0
density_score: 0.86
---

Connecting inbound interface VPC endpoints for Lambda - AWS Lambda
Connecting inbound interface VPC endpoints for Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#configuration-vpc-endpoints)
[Considerations for Lambda interface endpoints](#vpc-endpoint-considerations)[Creating an interface endpoint for Lambda](#vpc-endpoint-create)[Creating an interface endpoint policy for Lambda](#vpc-endpoint-policy)
# Connecting inbound interface VPC endpoints for Lambda
If you use Amazon Virtual Private Cloud (Amazon VPC) to host your AWS resources, you can establish a connection between your VPC and
Lambda. You can use this connection to invoke your Lambda function without crossing the public internet.
To establish a private connection between your VPC and Lambda, create an [interface VPC endpoint](https://docs.aws.amazon.com/vpc/latest/privatelink/vpce-interface.html). Interface endpoints are powered
by [AWS PrivateLink](https://aws.amazon.com/privatelink), which enables you to privately access Lambda APIs
without an internet gateway, NAT device, VPN connection, or AWS Direct Connect connection. Instances in your VPC don't
need public IP addresses to communicate with Lambda APIs. Traffic between your VPC and Lambda does not leave the AWS
network.
Each interface endpoint is represented by one or more [elastic network interfaces](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-eni.html) in your subnets. A network
interface provides a private IP address that serves as an entry point for traffic to Lambda.
###### Sections
* [Considerations for Lambda interface endpoints](#vpc-endpoint-considerations)
* [Creating an interface endpoint for Lambda](#vpc-endpoint-create)
* [Creating an interface endpoint policy for Lambda](#vpc-endpoint-policy)
## Considerations for Lambda interface endpoints
Before you set up an interface endpoint for Lambda, be sure to review [Interface endpoint properties and
limitations](https://docs.aws.amazon.com/vpc/latest/privatelink/vpce-interface.html#vpce-interface-limitations) in the *Amazon VPC User Guide*.
You can call any of the Lambda API operations from your VPC. For example, you can invoke the Lambda function by
calling the `Invoke` API from within your VPC.
For the full list of Lambda APIs, see [Actions](https://docs.aws.amazon.com/lambda/latest/dg/API_Operations.html) in the Lambda API
reference.
`use1-az3` is a limited capacity Region for Lambda VPC functions. You shouldn't use subnets in
this availability zone with your Lambda functions because this can result in reduced zonal redundancy in the event of an outage.
### Keep-alive for persistent connections
Lambda purges idle connections over time, so you must use a keep-alive directive to maintain persistent
connections. Attempting to reuse an idle connection when invoking a function results in a connection error. To
maintain your persistent connection, use the keep-alive directive associated with your runtime. For an example,
see [Reusing Connections with Keep-Alive in Node.js](https://docs.aws.amazon.com/sdk-for-javascript/v3/developer-guide/node-reusing-connections.html) in the *AWS SDK for JavaScript Developer Guide*.
### Billing Considerations
There is no additional cost to access a Lambda function through an interface endpoint. For more Lambda pricing
information, see [AWS Lambda Pricing](https://aws.amazon.com/lambda/pricing/).
Standard pricing for AWS PrivateLink applies to interface endpoints for Lambda. Your AWS account is billed for
every hour an interface endpoint is provisioned in each Availability Zone and for data processed through the
interface endpoint. For more interface endpoint pricing information, see [AWS PrivateLink pricing](https://aws.amazon.com/privatelink/pricing/).
### VPC Peering Considerations
You can connect other VPCs to the VPC with interface endpoints using [VPC peering](https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html). VPC peering is a networking connection
between two VPCs. You can establish a VPC peering connection between your own two VPCs, or with a VPC in another
AWS account. The VPCs can also be in two different AWS Regions.
Traffic between peered VPCs stays on the AWS network and does not traverse the public internet. Once VPCs are
peered, resources like Amazon Elastic Compute Cloud (Amazon EC2) instances, Amazon Relational Database Service (Amazon RDS) instances, or VPC-enabled Lambda functions in
both VPCs can access the Lambda API through interface endpoints created in the one of the VPCs.
## Creating an interface endpoint for Lambda
You can create an interface endpoint for Lambda using either the Amazon VPC console or the AWS Command Line Interface (AWS CLI). For
more information, see [Creating an interface endpoint](https://docs.aws.amazon.com/vpc/latest/privatelink/vpce-interface.html#create-interface-endpoint)
in the *Amazon VPC User Guide*.
###### To create an interface endpoint for Lambda (console)
1. Open the [Endpoints page](https://console.aws.amazon.com/vpc/home?#Endpoints) of the Amazon VPC
console.
2. Choose **Create Endpoint**.
3. For **Service category**, verify that **AWS services** is
selected.
4. For **Service Name**, choose
**com.amazonaws.`region`.lambda**. Verify that the
**Type** is **Interface**.
5. Choose a VPC and subnets.
6. To enable private DNS for the interface endpoint, select the **Enable DNS Name** check
box. We recommend that you enable private DNS names for your VPC endpoints for AWS services.
This ensures that requests that use the public service endpoints, such as requests made
through an AWS SDK, resolve to your VPC endpoint.
7. For **Security group**, choose one or more security groups.
8. Choose **Create endpoint**.
To use the private DNS option, you must set the `enableDnsHostnames` and
`enableDnsSupportattributes` of your VPC. For more information, see [Viewing and updating DNS support for your VPC](https://docs.aws.amazon.com/vpc/latest/userguide/vpc-dns.html#vpc-dns-updating)
in the *Amazon VPC User Guide*. If you enable private DNS for the interface endpoint, you can make
API requests to Lambda using its default DNS name for the Region, for example,
`lambda.us-east-1.amazonaws.com`. For more service endpoints, see [Service endpoints and quotas](https://docs.aws.amazon.com/general/latest/gr/aws-service-information.html) in the
*AWS General Reference*.
For more information, see [Accessing a service through an
interface endpoint](https://docs.aws.amazon.com/vpc/latest/privatelink/vpce-interface.html#access-service-though-endpoint) in the *Amazon VPC User Guide*.
For information about creating and configuring an endpoint using CloudFormation, see the [AWS::EC2::VPCEndpoint](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcendpoint.html)
resource in the *AWS CloudFormation User Guide*.
###### To create an interface endpoint for Lambda (AWS CLI)
Use the [create-vpc-endpoint](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/ec2/create-vpc-endpoint.html) command and specify the VPC ID, VPC endpoint type (interface), service name, subnets that will use the endpoint, and security groups to associate with the endpoint's network interfaces. For example:
```
`aws ec2 create-vpc-endpoint
--vpc-id `vpc-ec43eb89`
--vpc-endpoint-type Interface
--service-name com.amazonaws.us-east-1.lambda
--subnet-id subnet-abababab
--security-group-id `sg-1a2b3c4d``
```
## Creating an interface endpoint policy for Lambda
To control who can use your interface endpoint and which Lambda functions the user can access, you can attach
an endpoint policy to your endpoint. The policy specifies the following information:
* The principal that can perform actions.
* The actions that the principal can perform.
* The resources on which the principal can perform actions.
For more information, see [Controlling access to services with VPC endpoints](https://docs.aws.amazon.com/vpc/latest/privatelink/vpc-endpoints-access.html) in the *Amazon VPC User Guide*.
###### Example: Interface endpoint policy for Lambda actions
The following is an example of an endpoint policy for Lambda. When attached to an endpoint, this policy
allows user `MyUser` to invoke the function `my-function`.
###### Note
You need to include both the qualified and the unqualified function ARN in the resource.
```
`{
"Statement":[
{
"Principal":
{
"AWS": "arn:aws:iam::111122223333:user/MyUser"
},
"Effect":"Allow",
"Action":[
"lambda:InvokeFunction"
],
"Resource": [
"arn:aws:lambda:us-east-2:123456789012:function:my-function",
"arn:aws:lambda:us-east-2:123456789012:function:my-function:\*"
]
}
]
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Internet access for VPC functions
File system
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.