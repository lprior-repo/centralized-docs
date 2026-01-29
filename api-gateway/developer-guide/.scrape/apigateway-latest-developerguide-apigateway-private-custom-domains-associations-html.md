---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-private-custom-domains-associations.html
title: Tasks of API providers and API consumers for custom domain names for private APIs
word_count: 1015
filtered: true
elements_removed: 0
density_score: 0.82
---

Tasks of API providers and API consumers for custom domain names for private APIs - Amazon API Gateway
Tasks of API providers and API consumers for custom domain names for private APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-private-custom-domains-associations)
[Tasks of an API provider](#apigateway-private-custom-domains-associations-provider)[Tasks of an API consumer](#apigateway-private-custom-domains-associations-consumer)
# Tasks of API providers and API consumers for custom domain names for private APIs
When you create a private custom domain name, you're an *API provider*. When you
invoke a private custom domain name, you're an *API consumer*. You can consume a private
custom domain name from your own AWS account or from another AWS account.
The following section explains the tasks required by the API provider and API consumer to use a
private custom domain name. If you want to invoke a private custom domain name in your own AWS account, you are
both the API provider and the API consumer. If you want to invoke a private custom domain in another
AWS account, depending on the trust relationship between the API provider and API consumer in AWS Organizations, AWS RAM
might complete some tasks for you.
## Tasks of an API provider
API providers create private APIs and map them to custom domain names.
API providers manage two resource policies to protect their private custom domain names. The first policy
is for the `execute-api` service and controls which VPC endpoints can invoke your private custom
domain name. In the private custom domain name configuration, it's called the `policy`.
The second policy is for the Amazon API Gateway Management service and controls which VPC endpoints in
other AWS accounts can form a domain name access association with your private custom domain name. A VPC
endpoint needs to form a domain name access association with a private custom domain name to invoke it. In the
private custom domain name configuration, it's the `managementPolicy`. You can use AWS RAM or API Gateway to
update this policy. If you don't plan on allowing VPC endpoints in other AWS accounts to invoke your custom
domain name, you don't edit the `managementPolicy`.
If you are an API provider, you must do the following:
1. Create a private API.
2. Update your private API's `policy` to grant your VPC endpoint access to your private API.
3. Create a private custom domain name.
4. Update your private custom domain name's `policy` to grant your VPC endpoint access to your private custom domain name.
5. Create a base path mapping or a routing rule to send traffic from your private API to your private
custom domain name. For more information, see [Send traffic
to your APIs through your custom domain name in API Gateway](./rest-api-routing-mode.html).
If you want to allow API consumers in other AWS accounts to access your private custom domain name, do the following:
1. Update the `managementPolicy` of your private custom domain name to allow API consumers in
other accounts to associate their VPC endpoints with your private
custom domain name. You can do this using the following
methods:
**AWS RAM**
With AWS RAM, if the API provider and the API consumer are
in the same organization using AWS Organizations, the resource share between provider and consumer is automatically
accepted. Otherwise, you should wait until the API consumer accepts the resource share.
**We recommend that you use AWS RAM to share your private custom domain name.**
**API Gateway**
With API Gateway, only the AWS CLI is supported. You must update your private custom domain name using a patch operation
and provide your own policy document for the `managementPolicy`.
2. Update the `policy` of your private custom domain name and any private APIs mapped to it to
grant access to the API consumer's VPC endpoint.
For instructions on how to provide your API to another AWS account, see [API provider: Share your private custom domain name using AWS RAM](./apigateway-private-custom-domains-provider-share.html).
## Tasks of an API consumer
API consumers associate their VPC endpoints with a domain name ARN to invoke a private custom domain name.
API consumers don't need to create an API Gateway API.
If you are an API consumer, do the following:
1. Create a VPC endpoint with private DNS enabled in Amazon VPC.
2. (Optional - if AWS RAM is used) Accept a private custom domain resource share in AWS RAM within **12
hours** of the resource share. If you and the API provider are in the same
organization, the resource share is automatically accepted.
3. Get the private custom domain name ARN. Because the private custom domain name URL is not unique,
you use the private custom domain name ARN to form the domain name access
association between your VPC endpoint and the private custom domain name. You can use AWS RAM to retrieve the
private custom domain name ARN.
4. Associate the private custom domain ARN with your VPC endpoint in API Gateway. This creates a secure
connection between your VPC endpoint and the private custom domain name. Traffic doesn't leave the Amazon
network.
5. Wait for the API provider to grant your VPC endpoint access to the private custom domain name and any
private APIs mapped to the private custom domain name. If you're both the API provider and the API
consumer, you grant your own VPC endpoint invoke access.
6. Create a Route53 Private Hosted Zone and a Route53 record to resolve the private custom domain name in Route53.
For instructions on how to consume an API in another AWS account, see [API consumer: Associate your VPC endpoint with a private custom domain name shared with you](./apigateway-private-custom-domains-consumer-create.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Custom domain names for private APIs
Tutorial: Create and invoke a custom domain name for private APIs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.