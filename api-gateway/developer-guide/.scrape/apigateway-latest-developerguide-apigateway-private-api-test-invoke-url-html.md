---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-private-api-test-invoke-url.html
title: Invoke a private
word_count: 1031
filtered: true
elements_removed: 0
density_score: 0.86
---

Invoke a private API - Amazon API Gateway
Invoke a private API - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-private-api-test-invoke-url)
[Invoke a private API using a custom domain name](#apigateway-private-custom-domains-provider-invoke)[Invoke a private API without using a custom domain name](#apigateway-private-api-invoke-without-custom-domain-name)
# Invoke a private
API
You can only invoke a private API from within a VPC using a VPC endpoint. Your private API must have a resource policy that allows
specific VPCs and VPC endpoints to invoke your API.
If you invoke a private API without using a custom domain name or private DNS
names and your APIs or domain name uses a security policy that starts with
`SecurityPolicy\_`, you must set the endpoint access mode to `BASIC`. For more information,
see [Endpoint access mode](./apigateway-security-policies.html#apigateway-security-policies-endpoint-access-mode).
## Invoke a private API using a custom domain name
To invoke a private API using a custom domain name, your VPC endpoint needs a domain name access association
with a custom domain name, and the custom domain name needs to allow access for the VPC endpoint to invoke it.
For more information, see [Custom domain names for private APIs in API Gateway](./apigateway-private-custom-domains.html).
There are no differences between
invoking a private custom domain name in a VPC in your own AWS account or in a different AWS account.
### Use your custom domain name
Inside your VPC, you can invoke your API using the custom domain name. The following example is a curl
command to invoke your private custom domain name:
```
`curl https://private.example.com`
```
### Use endpoint-specific private DNS
hostnames
You can invoke your API using the custom domain name and the endpoint-specific private DNS
hostname.
```
`curl https://`private-dns-hostname`.execute-api.`region`.vpce.amazonaws.com/`basepath` -H 'Host:`custom-domain-name`'`
```
The following example is a curl
command to invoke your custom domain name using an endpoint-specific private DNS
hostname:
```
`curl https://vpce-123456-abc000.execute-api.us-east-2.vpce.amazonaws.com/test -H 'Host:private.example.com'`
```
## Invoke a private API without using a custom domain name
To invoke your private API without using a custom domain name, you need to identify the DNS names for your API. The following procedure shows how to find your DNS names.
AWS Management Console
###### To find the DNS names
1. Sign in to the AWS Management Console and open the Amazon VPC console at
[https://console.aws.amazon.com/vpc/](https://console.aws.amazon.com/vpc/).
2. In the main navigation pane, choose **Endpoints** and then
choose your interface VPC endpoint for API Gateway.
3. In the **Details** pane, you'll see five values in the
**DNS names** field. The first three are the public DNS names
for your API. The other two are the private DNS names for it.
AWS CLI
Use the following
[describe-vpc-endpoints](https://docs.aws.amazon.com/cli/latest/reference/ec2/describe-vpc-endpoints.html) command to list your DNS values.
```
`aws ec2 describe-vpc-endpoints --vpc-endpoint-ids vpce-01234567abcdef012`
```
The first three are the public DNS names
for your API. The other two are the private DNS names for it.
### Invoke a private API using a Route53 alias
You can associate or disassociate a VPC endpoint with your private API. For more information, see [(Optional) Associate or disassociate a VPC endpoint
with a private API](./apigateway-private-api-create.html#associate-private-api-with-vpc-endpoint).
After you associate your VPC endpoints with your private API, you can use the following base URL to invoke the API:
```
`https://`{rest-api-id}`-`{vpce-id}`.execute-api.`{region}`.amazonaws.com/`{stage}``
```
For example, if you set up the `GET /pets` method for the `test` stage, and your REST API ID was `01234567ab`, and your VPC endpoint ID was `vpce-01234567abcdef012`, and your Region was
`us-west-2`, you can invoke your API as:
```
`curl -v https://01234567ab-vpce-01234567abcdef012.execute-api.us-west-2.amazonaws.com/test/pets`
```
### Invoke a private API using private DNS names
If you've enabled private DNS, you can access your private API using the following private
DNS name:
```
``{restapi-id}`.execute-api.`{region}`.amazonaws.com`
```
The base URL to invoke the API is in the following format:
```
`https://`{restapi-id}`.execute-api.`{region}`.amazonaws.com/`{stage}``
```
For example, if you set up the `GET /pets` method for the `test` stage, and your REST API ID
was `01234567ab` and your Region was `us-west-2`, you could
invoke your private API by entering the following URL in a browser:
```
`https://01234567ab.execute-api.us-west-2.amazonaws.com/test/pets`
```
Alternatively, you could use the following cURL command to invoke your private API:
```
`curl -X GET https://01234567ab.execute-api.us-west-2.amazonaws.com/test/pets `
```
###### Warning
If you enable private DNS for your
VPC endpoint, you won't be able to access the default endpoint for public APIs. For more information, see
[Why can't I connect to my public API from an API Gateway VPC endpoint?](https://repost.aws/knowledge-center/api-gateway-vpc-connections).
### Invoke a private API using Direct Connect
You can use Direct Connect to establish a dedicated private connection from an
on-premises network to Amazon VPC and access your private API endpoint over that
connection by using public DNS names.
You can also use private DNS names to access your private API from an on-premises network by setting
up an Amazon Route53 Resolver inbound endpoint and forwarding it all DNS queries of the private DNS from your remote network.
For more information, see [Forwarding inbound DNS queries to
your VPCs](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/resolver-forwarding-inbound-queries.html) in the *Amazon Route53 Developer Guide*.
### Invoke a private API using endpoint-specific public DNS
hostnames
You can access your private API using endpoint-specific DNS hostnames. These are
public DNS hostnames containing the VPC endpoint ID or API ID for your private
API.
The generated base URL is in the following format:
```
`https://`{public-dns-hostname}`.execute-api.`{region}`.vpce.amazonaws.com/`{stage}``
```
For example, if you set up the `GET /pets` method for the `test` stage, and your REST API ID
was `abc1234`, its public DNS hostname was
`vpce-def-01234567`, and your Region was
`us-west-2`, you could invoke your private API using its VPCe ID by using the
`Host` header in a cURL command:
```
`curl -v https://vpce-def-01234567.execute-api.us-west-2.vpce.amazonaws.com/test/pets -H 'Host: abc1234.execute-api.us-west-2.amazonaws.com'`
```
Alternatively, you can invoke your private API via its API ID by using the
`x-apigw-api-id` header in a cURL command in the following
format:
```
`curl -v https://`{public-dns-hostname}`.execute-api.`{region}`.vpce.amazonaws.com/`{stage}` -H 'x-apigw-api-id:`{api-id}`'`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create a custom domain name for private APIs using CloudFormation
Monitor
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.