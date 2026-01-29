---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-private.html
title: Create private integrations
word_count: 752
filtered: true
elements_removed: 0
density_score: 0.85
---

Create private integrations for HTTP APIs in API Gateway - Amazon API Gateway
Create private integrations for HTTP APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-develop-integrations-private)
[Considerations](#http-api-develop-integrations-private-considerations)[Create a private integration
using an Application Load Balancer or Network Load Balancer ](#http-api-develop-integrations-private-ELB)[Create a private
integration using AWS Cloud Map service discovery](#http-api-develop-integrations-private-Cloud-Map)
# Create private integrations
for HTTP APIs in API Gateway
Private integrations enable you to create API integrations with private resources in a
VPC, such as Application Load Balancers or Amazon ECS container-based applications.
You can expose your resources in a VPC for access by clients outside of the VPC by using
private integrations. You can control access to your API by using any of the [
authorization methods](./http-api-access-control.html) that API Gateway supports.
###### Note
To create a private integration, you must first create a VPC link. VPC links V2 are now supported for both
HTTP and REST APIs. To learn more about VPC links V2, see [Set up VPC links V2 in API Gateway](./apigateway-vpc-links-v2.html).
After you’ve created a VPC link V2, you can set up private integrations that connect to an
Application Load Balancer, Network Load Balancer, or resources registered with an AWS Cloud Map service.
## Considerations
The following considerations might impact your use of private integrations:
* All resources must be owned by the same AWS account. This includes the load balancer or AWS Cloud Map service,
VPC link and HTTP API.
* By default, private integration traffic uses the HTTP protocol. To use HTTPS, specify a [`tlsConfig`](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/apis-apiid-integrations-integrationid.html).
To do this using the AWS Management Console, when you create your private
integration, choose **Advanced settings** and then enter a secure server name.
* For private integrations, API Gateway includes the [stage](./http-api-stages.html) portion of the API
endpoint in the request to your backend resources. For example, a request to the `test` stage of an API
includes `test/`route-path`` in the request to your private integration. To
remove the stage name from the request to your backend resources, use [parameter mapping](./http-api-parameter-mapping.html) to overwrite the request path to `$request.path`.
## Create a private integration
using an Application Load Balancer or Network Load Balancer
Before you create a private integration, you must create a VPC link V2. To learn more
about VPC links V2, see [Set up VPC links V2 in API Gateway](./apigateway-vpc-links-v2.html).
To create a private integration with an Application Load Balancer or Network Load Balancer, create an HTTP proxy
integration, specify the VPC link to use, and provide the listener ARN of the load
balancer.
The following [create-integration](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-integration.html)
command creates a private integration that connects to a load balancer by using a VPC link:
```
`aws apigatewayv2 create-integration --api-id `api-id` --integration-type HTTP\_PROXY \\
--integration-method GET --connection-type VPC\_LINK \\
--connection-id `VPC-link-ID` \\
--integration-uri `arn:aws:elasticloadbalancing:us-east-2:123456789012:listener/app/my-load-balancer/50dc6c495c0c9188/0467ef3c8400ae65`
--payload-format-version 1.0`
```
## Create a private
integration using AWS Cloud Map service discovery
Before you create a private integration, you must create a VPC link V2. To learn more
about VPC links, see [Set up VPC links V2 in API Gateway](./apigateway-vpc-links-v2.html).
For integrations with AWS Cloud Map, API Gateway uses `DiscoverInstances` to identify
resources. You can use query parameters to target specific resources. The registered
resources' attributes must include IP addresses and ports. API Gateway distributes requests
across healthy resources that are returned from `DiscoverInstances`. To learn
more, see [DiscoverInstances](https://docs.aws.amazon.com/cloud-map/latest/api/API_DiscoverInstances.html) in
the AWS Cloud Map API Reference.
###### Note
If you use Amazon ECS to populate entries in AWS Cloud Map, you must configure your Amazon ECS task
to use SRV records with Amazon ECS Service Discovery or turn on Amazon ECS Service Connect. For more information, see
[Interconnecting
services](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/interconnecting-services.html) in the
Amazon Elastic Container Service Developer Guide.
To create a private integration with AWS Cloud Map, create an HTTP proxy integration, specify
the VPC link to use, and provide the ARN of the AWS Cloud Map service.
The following [create-integration](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-integration.html)
command creates a private integration that uses AWS Cloud Map service discovery to identify resources:
```
`aws apigatewayv2 create-integration --api-id `api-id` --integration-type HTTP\_PROXY \\
--integration-method GET --connection-type VPC\_LINK \\
--connection-id `VPC-link-ID` \\
--integration-uri `arn:aws:servicediscovery:us-east-2:123456789012:service/srv-id?stage=prod&amp;&amp;deployment=green\_deployment`
--payload-format-version 1.0`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
AWS service integrations reference
CORS
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.