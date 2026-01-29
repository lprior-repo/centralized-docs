---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-private-custom-domains-tutorial.html
title: Tutorial: Create and invoke a custom domain name for private APIs
word_count: 2233
filtered: true
elements_removed: 0
density_score: 0.83
---

Tutorial: Create and invoke a custom domain name for private APIs - Amazon API Gateway
Tutorial: Create and invoke a custom domain name for private APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-private-custom-domains-tutorial)
[Step 1: Create a private custom domain name
](#apigateway-private-custom-domains-provider-create-domain)[Step 2: Create a base path mapping to map your
private API to your private custom domain name](#apigateway-private-custom-domains-base-path-mapping)[Step 3: Create a domain name
access association between your custom domain name and a VPC endpoint ](#apigateway-private-custom-domains-provider-associate-with-vpce)[Step 4: Create a Route53 hosted zone
](#apigateway-private-custom-domains-provider-create-route-53-private-hosted-zone)[Step 5: Create a Route53 DNS record
](#apigateway-private-custom-domains-provider-create-route-53-record)[Step 6: Invoke your private custom domain name
](#apigateway-private-custom-domains-tutorial-invoke)[Step 7: Clean up](#apigateway-private-custom-domains-cleanup)[Best practices](#apigateway-private-custom-domains-best-practices)
# Tutorial: Create and invoke a custom domain name for private APIs
In this tutorial, you create a private custom domain name that you can invoke in a VPC in your own account. To
accomplish this, you are the API provider and the API consumer. You need an existing private API and VPC endpoint
to complete this tutorial. If you have a VPC endpoint that you use to access a public custom domain name, don't
use it for this tutorial or to create any domain name access associations.
## Step 1: Create a private custom domain name
You create your private custom domain name by specifying the domain name, the ACM certificate, and the
policy for the `execute-api` service to control which VPC endpoints can invoke it.
AWS Management Console
###### To create a private custom domain name
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Custom domain names**.
3. Choose **Add domain name**.
4. For **Domain name**, enter a domain name.
Your ACM certificate must cover this domain name, but the domain name doesn't need to be
unique.
5. Select **Private**.
6. For **Routing mode**, choose **API mappings only**.
7. For **ACM certificate**, select a certificate.
8. Choose **Add domain name**.
API Gateway provisions a domain name with a `deny` all resource policy. This is the resource policy for the
`execute-api` service. You need to update this resource
policy to grant access to your VPC endpoints to invoke your private custom domain name.
###### To update your resource policy
1. Choose the **Resource policy** tab, and then choose **Edit resource
policy**.
2. Enter the following resource policy in the code editor. Replace the VPC endpoint
`vpce-abcd1234efg` with your own VPC endpoint ID.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": [
"execute-api:/\*"
]
},
{
"Effect": "Deny",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": [
"execute-api:/\*"
],
"Condition" : {
"StringNotEquals": {
"aws:SourceVpce": "`vpce-abcd1234`"
}
}
}
]
}`
`
```
3. Choose **Save changes**.
AWS CLI
When you create a private custom domain name using the AWS CLI, you provide a resource policy for the
`execute-api` service to grant access to VPC
endpoints to invoke your private custom domain name, using the `--policy
file://policy.json` parameter. You can modify this policy later.
For this example, you'll attach the following resource policy as the `policy` by loading
parameters from a file. Copy and save this file as `policy.json`. This policy only allows incoming
traffic to a private custom domain name from the VPC endpoint `
`vpce-abcd1234efg``:
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": [
"execute-api:/\*"
]
},
{
"Effect": "Deny",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": [
"execute-api:/\*"
],
"Condition" : {
"StringNotEquals": {
"aws:SourceVpce": "`vpce-abcd1234`"
}
}
}
]
}`
`
```
The following [create-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-domain-name.html)
command creates a private custom domain name:
```
`aws apigateway create-domain-name \\
--domain-name 'private.example.com' \\
--certificate-arn 'arn:aws:acm:us-west-2:111122223333:certificate/a1b2c3d4-5678-90ab-cdef' \\
--security-policy 'TLS\_1\_2' \\
--endpoint-configuration '{"types":["PRIVATE"]}' \\
--policy file://policy.json`
```
The output will like the following.
```
`{
"domainName": "private.example.com",
"domainNameId": "abcd1234",
"domainNameArn": "arn:aws:apigateway:us-west-2:111122223333:/domainnames/private.example.com+abcd1234",
"certificateArn": "arn:aws:acm:us-west-2:111122223333:certificate/a1b2c3d4-5678-90ab-cdef",
"certificateUploadDate": "2024-09-10T10:31:20-07:00",
"endpointConfiguration": {
"types": [
"PRIVATE"
]
},
"domainNameStatus": "AVAILABLE",
"securityPolicy": "TLS\_1\_2",
"routingMode" : "API\_MAPPING\_ONLY",
"policy": "..."
}`
```
## Step 2: Create a base path mapping to map your
private API to your private custom domain name
After you create your private custom domain name, you map a private API to it. A base path mapping makes an
API accessible through the combination of the private custom domain name and an associated base path. We
recommend that you use a single private custom domain name as the hostname of multiple private APIs.
All API providers need to create a base path mapping, even if you don't plan on invoking your own API.
You also need to grant access for VPC endpoints to invoke any private APIs that you map to your private
custom domain name.
AWS Management Console
###### To create a base path mapping
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Custom domain names**.
3. Choose a private custom domain name.
4. On the **API mappings** tab, choose **Configure mappings**.
5. Choose **Add new mapping**.
6. Enter an **API**, a **Stage**, and optionally a **Path**.
7. Choose **Save**.
AWS CLI
The following [create-base-path-mapping](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-base-path-mapping.html) command
creates a mapping between a private API and a private custom domain name:
```
`aws apigateway create-base-path-mapping \\
--domain-name-id abcd1234 \\
--domain-name 'private.example.com' \\
--rest-api-id a1b2c3 \\
--stage prod \\
--base-path v1`
```
The output will look like the following.
```
`{
"basePath": "v1",
"restApiId": "a1b2c3",
"stage": "prod"
}`
```
For more flexibility on how you route traffic to your APIs, you can change the routing mode to
`ROUTING\_RULE\_ONLY` or `ROUTING\_RULE\_THEN\_API\_MAPPING` and create a routing rule. For
more information, see [Send traffic
to your APIs through your custom domain name in API Gateway](./rest-api-routing-mode.html).
###### Note
If you want other AWS accounts to invoke your private custom domain name, after you complete this
tutorial, follow the steps in [API provider: Share your private custom domain name using AWS RAM](./apigateway-private-custom-domains-provider-share.html).
## Step 3: Create a domain name
access association between your custom domain name and a VPC endpoint
Next, you create a domain name access association between your private custom domain name and your VPC
endpoint. Your VPC endpoint uses the domain name access association to invoke your private custom domain name
while isolated from the public internet.
AWS Management Console
###### To create a domain name access association
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Custom domain names**.
3. Choose a private custom domain name.
4. In the **Resource sharing** tab, for **Domain name access
associations**, choose **Create domain name access association**.
5. For **Domain name ARN**, select your domain name.
6. For **VPC endpoint ID**, select the VPC endpoint ID you provided access to in
step 1.
7. Choose **Domain name access association**.
You can also create your domain name access association using the
**Domain name access associations** page of the console.
AWS CLI
The following `create-domain-name-access-association` command creates a domain name access
association between your private custom domain name and your VPC
endpoint.
```
`aws apigateway create-domain-name-access-association \\
--domain-name-arn arn:aws:apigateway:us-west-2:111122223333:/domainnames/private.example.com+abcd1234 \\
--access-association-source vpce-abcd1234efg \\
--access-association-source-type VPCE \\
--region us-west-2`
```
The output will look like the following.
```
`{
"domainNameAccessAssociationARN": "arn:aws:apigateway:us-west-2:111122223333:/domainnameaccessassociations/domainname/private.example.com+abcd1234/vpcesource/vpce-abcd1234efg",
"accessAssociationSource": "vpce-abcd1234efg",
"accessAssociationSourceType": "VPCE",
"domainNameARN" : "arn:aws:apigateway:us-west-2:111122223333:/domainnames/private.example.com+abcd1234"
}`
```
After you create your domain name access association, it takes about 15 minutes to be ready. While you wait,
you can proceed with the following steps.
## Step 4: Create a Route53 hosted zone
After you update your resource policy and associate your private custom domain name with your VPC endpoint, you create a
private hosted zone in Route53 to resolve your custom domain name. A hosted zone is container that holds information about how you want to route
traffic for a domain within one or more VPCs without exposing your resources to the internet. For more information, see [Working with private hosted zones](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zones-private.html).
AWS Management Console
To use the AWS Management Console, see
[Creating
a private hosted zone](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zone-private-creating.html) in the *Amazon Route53 Developer Guide*.
For **Name**, use the name of your private custom domain name. For **VPC
ID**, use the VPC containing the VPC endpoint that you used in the previous steps.
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
## Step 5: Create a Route53 DNS record
After you create the hosted zone, you create an record to resolve your private custom domain name. You use
the hosted zone ID you created in the previous step. In this example, you create an A record type. If you are
using IPv6 for your VPC endpoint, create an AAAA record type. If you are using dualstack for your VPC endpoint, create both an AAAA and an A record type.
AWS Management Console
To use the AWS Management Console, see [Routing traffic to an Amazon API Gateway API by
using your domain name](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-to-api-gateway.html).
Use **Quick create** and turn on **Alias**. For endpoint, use the
VPC endpoint DNS name.
AWS CLI
To configure your DNS records to map the private custom domain name to its hostname of the given
hosted zone ID, you create a JSON file that contains the configuration for setting up a DNS record for
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
The following [change-resource-record-sets](https://docs.aws.amazon.com/cli/latest/reference/route53/change-resource-record-sets.html) command creates a DNS record for your private custom domain
name:
```
`aws route53 change-resource-record-sets \\
--hosted-zone-id ZABCDEFG1234 \\
--change-batch file://`path/to/your/setup-dns-record.json``
```
Replace the`hosted-zone-id` with the Route53 Hosted Zone ID of the DNS record set in your
account. The `change-batch` parameter value points to a JSON file.
If you don't plan on invoking your own private custom domain name, after you
confirm your private custom domain name is working, you can delete these resources.
## Step 6: Invoke your private custom domain name
You can now invoke your private custom domain name in your own AWS account. In your VPC, use the following curl
command to access your private custom domain name.
```
`curl https://private.example.com/v1`
```
For more information about other ways to invoke your private API, see [Invoke a private API using a custom domain name](./apigateway-private-api-test-invoke-url.html#apigateway-private-custom-domains-provider-invoke).
## Step 7: Clean up
To prevent unnecessary costs, delete the association between your VPC endpoint and your private custom domain name,
and then delete your private custom domain name.
AWS Management Console
###### To delete the domain name access association
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Domain name access associations**.
3. Select your domain name access association, and then choose **Delete**.
4. Confirm your choice, and then choose **Delete**.
After you delete your domain name access association, you can delete your private custom domain
name.
###### To delete your private custom domain name
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Custom domain names**.
3. Choose your private custom domain name.
4. Choose **Delete**.
5. Confirm your choice, and then choose **Delete**.
If necessary, you can also delete your VPC endpoint. For more information, see
[Delete an
interface endpoint](https://docs.aws.amazon.com/vpc/latest/privatelink/delete-interface-endpoint.html).
AWS CLI
###### To clean up
1. The following `delete-access-association`
command deletes the domain name access association:
```
`aws apigateway delete-domain-name-access-association \\
--domain-name-access-association-arn 'arn:aws:apigateway:us-west-2:111122223333:/domainnameaccessassociations/domainname/private.example.com+abcd1234/vpcesource/vpce-abcd1234efg' \\
--region us-west-2`
```
2. The following `delete-domain-name` command deletes your private custom domain name. This
command also removes all base path mappings.
```
`aws apigateway delete-domain-name \\
--domain-name test.private.com \\
--domain-name-id abcd1234`
```
If necessary, you can also delete your VPC endpoint. For more information, see
[Delete an
interface endpoint](https://docs.aws.amazon.com/vpc/latest/privatelink/delete-interface-endpoint.html).
## Best practices
We recommend that you use the following best practices when you create your private custom domain name:
* Use base path mapping or routing rules to send traffic from one private custom domain name to multiple
private APIs.
* When a VPC endpoint no longer needs access to a private custom domain name, delete the association. In
addition, remove the VPC endpoint from the `policy` for the `execute-api` service for the private custom domain.
* Configure at least two Availability Zones per VPC endpoint.
* Disable the default endpoint. We recommend that you disable the default endpoint to allow your API
consumers to only call your API from the custom domain name. For more information, see [Disable the default endpoint for REST APIs](./rest-api-disable-default-endpoint.html).
* We recommend that you provision a Route53 private hosted zone and an A-type record when you set up your
private custom domain name. If you don't plan on invoking your own private custom domain name, you can
delete these resources later.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API providers and API consumers
Working with cross-account private custom domain names
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.