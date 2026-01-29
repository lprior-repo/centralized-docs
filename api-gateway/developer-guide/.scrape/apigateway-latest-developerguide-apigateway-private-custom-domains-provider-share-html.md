---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-private-custom-domains-provider-share.html
title: API provider: Share your private custom domain name using AWS RAM
word_count: 1076
filtered: true
elements_removed: 0
density_score: 0.80
---

API provider: Share your private custom domain name using AWS RAM - Amazon API Gateway
API provider: Share your private custom domain name using AWS RAM - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-private-custom-domains-provider-share)
[Considerations for sharing
your private custom domain name](#apigateway-private-custom-domains-provider-share-considerations)[Allow other accounts to
create domain name access associations with your private custom domain name](#apigateway-private-custom-domains-provider-management-policy-update)[Allow other accounts to invoke your
private custom domain name](#apigateway-private-custom-domains-provider-policy-update)
# API provider: Share your private custom domain name using AWS RAM
You can provide API consumers in other AWS accounts access to your private custom domain name. In this
section, you learn how to share your private custom domain name using AWS RAM and how to control access to
your private custom domain name.
## Considerations for sharing
your private custom domain name
The following considerations might impact how you provide access to your private custom domain name using
AWS RAM. To learn how to share your private custom domain name without using AWS RAM, see [API provider: Share your private custom domain name using the API Gateway AWS CLI](./apigateway-private-custom-domains-provider-share-cli.html).
* Private custom domain names are shared at the AWS Region level. Both the private custom domain name
and the VPC endpoint need to be in the same AWS Region.
* You can use one resource share with multiple principals, and after you create the resource share, you
can add more principals to it. We recommend that when possible, you reuse your resource
share.
* You always need to grant the API consumer's VPC endpoint access to invoke your private custom
domain name and any private APIs mapped to it.
* If the API consumer and API provider are in the same organization using AWS Organizations,
the resource share is automatically accepted. You still need to create the resource share using
AWS RAM.
* If the API consumer and API provider are in the same organization using AWS Organizations and
resource sharing within your organization is enabled, any principals in the organization that you share
with are automatically granted access to the resource shares. There is no need for an invitation and you
can skip the resource share.
* If the API consumer doesn't accept the resource share within **12
hours**, the API provider must share the resource
again.
* After you create the resource share, AWS RAM updates the `managementPolicy` for the
Amazon API Gateway Management service for your private custom domain name to prevent access to
principals without explicit `allow` access. For more information, see [Determining
whether a request is allowed or denied within an account](https://docs.aws.amazon.com//IAM/latest/UserGuide/reference_policies_evaluation-logic.html#policy-eval-denyallow) in the IAM User Guide.
The updated `managementPolicy` will look like the following:
JSON
****
```
``{
"Version":"2012-10-17",
"Id": "abcd1234-1234-abcd-abcd-1234abcdefg",
"Statement": [
{
"Sid": "APIGatewayPrivateDomainNameManagementPolicyDefaultPermission-org",
"Effect": "Allow",
"Principal": "\*",
"Action": "apigateway:CreateAccessAssociation",
"Resource": "arn:aws:apigateway:us-west-2:111122223333:/domainnames/private.example.com+abcd1234",
"Condition": {
"StringEquals": {
"aws:PrincipalOrgID": "o-1234abcd"
},
"StringNotEquals": {
"aws:PrincipalAccount": "111122223333"
}
}
}
]
}`
`
```
AWS RAM has prevented principals without explicit `allow` access to create access
associations with your private custom domain name, by adding the following:
```
`"StringNotEquals": {
"aws:PrincipalAccount": "111122223333"
}`
```
You can still use the principal in the AWS account who created the private custom domain name to
create domain name access associations.
## Allow other accounts to
create domain name access associations with your private custom domain name
First, you grant access to another AWS account to create domain name access associations with your private
custom domain name.
AWS Management Console
To use the AWS Management Console, see
[Creating a
resource share in AWS RAM](https://docs.aws.amazon.com/ram/latest/userguide/working-with-sharing-create.html) in the *AWS RAM User Guide*.
For **Select resource type**, choose
**API Gateway Private Custom Domains**.
AWS CLI
The following [create-resource-share](https://docs.aws.amazon.com/cli/latest/reference/ram/create-resource-share.html)
creates a resource share for your private custom domain name. It can take a few minutes for the resource and principal associations to complete.
For principals, provide an account ID or an Organizations ID, such as
`arn:aws:organizations::123456789012:organization/o-1234abcd`. You can provide multiple principals for your resource share.
```
`aws ram create-resource-share \\
--region us-west-2 \\
--name privateCustomDomain-resource-share \\
--permission-arns arn:aws:ram::aws:permission/APIGatewayPrivateDomainNameManagementPolicyDefaultPermission \\
--resource-arns arn:aws:apigateway:us-west-2:111122223333:/domainnames/private.example.com+abcd1234 \\
--principals 222222222222`
```
After you have provided access to another AWS account, API consumers in that account must create a
domain name access association between their VPC endpoint with your private custom domain name. You can't
create the domain name access association for them. For more information, see [Associate your VPC endpoint with a shared private
custom domain name](./apigateway-private-custom-domains-consumer-create.html#apigateway-private-custom-domains-consumer-associate).
## Allow other accounts to invoke your
private custom domain name
Next, you grant access for the API consumer's VPC endpoint to invoke your private custom domain
name and any private APIs mapped to it.
AWS Management Console
###### To allow VPC endpoints in other accounts to invoke your private custom domain name
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Custom domain names**.
3. Choose the private custom domain name that you shared with other AWS accounts.
4. On the **Resource policy** tab, choose **Edit resource
policy**.
5. Add the VPC endpoint ID of the API consumer to your resource policy.
You can find the VPC endpoint ID of the API consumer on the **Domain name access
associations** section of the **Resource sharing** tab on
the **Domain details** page of your private custom domain name.
6. Choose **Save changes**.
AWS CLI
The following `policy` for the `execute-api` service allows incoming traffic to a private custom domain name from both VPC
endpoint ``vpce-abcd1234efg`` and
``vpce-xyz000abc``.
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
"aws:SourceVpce": [
"`vpce-abcd1234`",
"`vpce-xyzz0000`"
]
}
}
}
]
}`
`
```
The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html)
command uses a patch operation to update the `policy` for a private custom domain name:
```
`aws apigateway update-domain-name
--domain-name private.example.com \\
--domain-name-id abcd1234 \\
--patch-operations op=replace,path=/policy,value='"{\\"Version\\": \\"2012-10-17\\", \\"Statement\\": [{\\"Effect\\": \\"Allow\\",\\"Principal\\": \\"\*\\",\\"Action\\": \\"execute-api:Invoke\\",\\"Resource\\":[\\"execute-api:/\*\\"]},{\\"Effect\\": \\"Deny\\",\\"Principal\\": \\"\*\\",\\"Action\\": \\"execute-api:Invoke\\",\\"Resource\\":[\\"execute-api:/\*\\"],\\"Condition\\":{\\"StringNotEquals\\":[\\"vpce-abcd1234efg\\", \\"vpce-xyz000abc\\"]}}}]}"`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Working with cross-account private custom domain names
API provider: Stop sharing a private custom domain name using AWS RAM
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.