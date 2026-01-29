---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-private-custom-domains-consumer-create.html
title: API consumer: Associate your VPC endpoint with a private custom domain name shared with you
word_count: 1389
filtered: true
elements_removed: 0
density_score: 0.84
---

API consumer: Associate your VPC endpoint with a private custom domain name shared with you - Amazon API Gateway
API consumer: Associate your VPC endpoint with a private custom domain name shared with you - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-private-custom-domains-consumer-create)
[Prerequisites](#apigateway-private-custom-domains-consumer-prerequisites)[(Optional) Accept the private custom domain resource share](#apigateway-private-custom-domains-consumer-accept-resource-share)[Associate your VPC endpoint with a shared private
custom domain name](#apigateway-private-custom-domains-consumer-associate)[Create a Route53 hosted zone
](#apigateway-private-custom-domains-consumer-create-route-53-private-hosted-zone)[Create a Route53 DNS record
](#apigateway-private-custom-domains-consumer-create-route-53-record)[Next steps for an API consumer
](#apigateway-private-custom-domains-consumer-next-steps)
# API consumer: Associate your VPC endpoint with a private custom domain name shared with you
The following procedure shows how to consume a private domain name in another AWS account. Depending
on your trust relationship with the API provider, AWS RAM might complete some tasks for you.
When you are in a different AWS account from a private custom domain name, you can only associate your VPC
endpoint with a private custom domain name and invoke it. You can't view the `policy` or any other
parameters of the private custom domain name.
## Prerequisites
The following prerequisites are required to consume a private custom domain name in another AWS account:
* A VPC and a VPC endpoint for the `execute-api` service. Your VPC must have `enableDnsHostnames`
and `enableDnsSupport` set to `true`.
* We recommend that you configure at least two Availability Zones per VPC endpoint.
## (Optional) Accept the private custom domain resource share
If your API provider used AWS RAM to create a resource share, you have **12 hours** to
accept it. If you are in the same organization using AWS Organizations as the API provider, the share is automatically
accepted. If you are in an organization that has automatic shared resources enabled, the resource is
automatically shared with you.
AWS Management Console
To use the AWS Management Console, see [Accepting and rejecting resource share
invitations](https://docs.aws.amazon.com/ram/latest/userguide/working-with-shared-invitations.html) in the *AWS RAM User Guide*.
AWS CLI
To find all resources shared with you, use the following [get-resource-share-invitations](https://docs.aws.amazon.com/cli/latest/reference/ram/get-resource-share-invitations.html)
command:
```
`aws ram get-resource-share-invitations \\
--region us-west-2`
```
Use the resulting resource share ARN to accept the resource share invitation. The following
[accept-resource-share-invitation](https://docs.aws.amazon.com/cli/latest/reference/ram/accept-resource-share-invitation.html)
command accepts the resource share.
```
`aws ram accept-resource-share-invitation \\
--resource-share-invitation-arn arn:aws:ram:us-west-2:123456789012:resource-share-invitation/1e3477be-4a95-46b4-bbe0-c4001EXAMPLE \\
--region us-west-2`
```
## Associate your VPC endpoint with a shared private
custom domain name
Because private custom domain names aren't unique, you associate your VPC endpoint with the unique custom
domain name ARN. After you create your domain name access association, it can take up to 15 minutes for your
VPC endpoint to successfully invoke your private custom domain name. If
you have a VPC endpoint that you use to access a public custom domain name, don't use it to create any domain
name access associations.
AWS Management Console
###### To associate your VPC endpoint with a shared private custom domain name
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Domain name access associations**.
3. Choose **Create domain name access association**.
4. For **Domain name ARN**, select the domain name ARN that the API provider
shared with you.
The domain name ARN might not appear in the dropdown list. You can use the AWS RAM console to view
domain names shared with you and then copy the domain name ARN and enter it into this
field.
5. For **VPC endpoint ID**, select the VPC endpoint ID you want to form the domain
name access association with.
6. Choose **Create domain name access association**.
AWS CLI
Because private custom domain names aren't unique, you associate your VPC endpoint with the unique custom
domain name ARN. To find the domain name ARN, use one of the following commands.
1.
**AWS RAM**
The following [list-resources](https://docs.aws.amazon.com/cli/latest/reference/ram/list-resources.html) command
lists resources that are shared with you. The API provider must have used AWS RAM to share their private
custom domain with you to use this command.
```
`aws ram list-resources \\
--resource-owner OTHER-ACCOUNTS \\
--region us-west-2
--resource-type apigateway:Domainnames`
```
**API Gateway**
The following `get-domain-names` command lists all private custom domain names owned by other AWS accounts that you
can form domain name access associations with.
```
`aws apigateway get-domain-names \\
--resource-owner OTHER\_ACCOUNTS \\
--region us-west-2`
```
2. After your retrieve the ARN, use API Gateway to create the domain name access association between
your VPC endpoint and a shared private custom domain name. Use the following
`create-domain-name-access-association` command:
```
`aws apigateway create-domain-name-access-association \\
--access-association-source-type VPCE \\
--access-association-source 'vpce-1a2b3c4d5e6f1a2b3' \\
--domain-name-arn arn:aws:apigateway:us-west-2:111122223333:/domainnames/private.example.com+abcd1234"`
```
The output will look like the following.
```
`{
"domainNameAccessAssociationARN": "arn:aws:apigateway:us-west-2:444455556666:/domainnameaccessassociations/domainname/private.example.com+abcd1234/vpcesource/vpce-abcd1234efg",
"accessAssociationSource": "vpce-1a2b3c4d5e6f1a2b3",
"accessAssociationSourceType": "VPCE",
"domainNameARN" : "arn:aws:apigateway:us-west-1:111122223333:/domainnames/private.example.com+a1b2c3"
}`
```
After you associate your VPC endpoint with the private custom domain name, confirm that your API provider has updated
the policy of their private custom domain name to allow your VPC endpoint to invoke their domain name. For more
information, see [Allow other accounts to invoke your
private custom domain name](./apigateway-private-custom-domains-provider-share.html#apigateway-private-custom-domains-provider-policy-update).
## Create a Route53 hosted zone
To resolve the private custom domain name, you need to create a Route53 private hosted zone. A hosted zone is container that
holds information about how you want to route traffic for a domain within one or more VPCs without exposing your
resources to the internet. For more information, see [Working with private hosted zones](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zones-private.html).
AWS Management Console
To use the AWS Management Console, see
[Creating
a private hosted zone](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zone-private-creating.html) in the *Amazon Route53 Developer Guide*.
For **Name**, use the name of the private custom domain name. For **VPC
ID**, use the VPC containing the VPC endpoint that you used for your domain name access
association.
AWS CLI
The following [create-hosted-zone](https://docs.aws.amazon.com/cli/latest/reference/route53/create-hosted-zone.html) command
creates a private hosted zone:
```
`aws route53 create-hosted-zone --name private.example.com \\
--caller-reference 2014-04-01-18:47 \\
--hosted-zone-config Comment="command-line version",PrivateZone=true \\
--vpc VPCRegion=us-west-2,VPCId=vpc-abcd1234`
```
The output contains the hosted zone ID. You use the hosted zone ID in the following steps.
## Create a Route53 DNS record
After you create the hosted zone, you create an record to resolve the private custom domain. In this
example, you create an A record type. If you are using IPv6 for your VPC endpoint, create an AAAA record
type. If you are using dualstack for your VPC endpoint, create both an AAAA and an A record type.
AWS Management Console
To use the AWS Management Console, see [Routing traffic to an Amazon API Gateway API by using your domain name](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-to-api-gateway.html).
Use **Quick create** and turn on **Alias**. For endpoint, use the
VPC endpoint DNS name.
AWS CLI
To configure your DNS records to map the private custom domain name to its hostname of the given
hosted zone ID, first create a JSON file that contains the configuration for setting up a DNS record for
the private domain name.
The following `setup-dns-record.json` shows how to create a DNS `A` record to
map a private custom domain name to its private hostname. You provide the `DNSName` of your VPC
DNS ID, and the hosted zone ID you created in the previous step.
```
`{
"Changes": [
{
"Action": "UPSERT",
"ResourceRecordSet": {
"Name": "private.example.com",
"Type": "A",
"AliasTarget": {
"DNSName": "vpce-abcd1234.execute-api.us-west-2.vpce.amazonaws.com",
"HostedZoneId": "Z2OJLYMUO9EFXC",
"EvaluateTargetHealth": false
}
}
}
]
}
`
```
The following [change-resource-record-sets](https://docs.aws.amazon.com/cli/latest/reference/route53/change-resource-record-sets.html) command creates a DNS record for the private custom domain
name:
```
`aws route53 change-resource-record-sets \\
--hosted-zone-id ZABCDEFG1234 \\
--change-batch file://`path/to/your/setup-dns-record.json``
```
Replace the`hosted-zone-id` with the Route53 Hosted Zone ID of the DNS record set in your
account. The `change-batch` parameter value points to a JSON file.
## Next steps for an API consumer
You can now invoke the private API in your own AWS account. In your VPC, you can use the following curl
command to access your private custom domain name.
```
`curl https://private.example.com/v1`
```
For more information about other ways to invoke your private API, see [Invoke a private API using a custom domain name](./apigateway-private-api-test-invoke-url.html#apigateway-private-custom-domains-provider-invoke).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API provider: Share your private custom domain name using the API Gateway AWS CLI
API consumer: Delete your domain name access association with a private custom domain name
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.