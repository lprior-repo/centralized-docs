---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-private-custom-domains-provider-share-cli.html
title: API provider: Share your private custom domain name using the API Gateway AWS CLI
word_count: 935
filtered: true
elements_removed: 0
density_score: 0.84
---

API provider: Share your private custom domain name using the API Gateway AWS CLI - Amazon API Gateway
API provider: Share your private custom domain name using the API Gateway AWS CLI - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-private-custom-domains-provider-share-cli)
[Grant access to your private custom domain name](#apigateway-private-custom-domains-provider-share-cli-allow)[Deny access to your private custom domain name](#apigateway-private-custom-domains-provider-share-cli-deny)[Example policies used in this procedure](#apigateway-private-custom-domains-provider-share-cli-policies)
# API provider: Share your private custom domain name using the API Gateway AWS CLI
You can share a private custom domain name using the API Gateway AWS CLI, but we recommend that you use AWS RAM to reduce
your operational overhead. For instructions on how to use AWS RAM to share your private custom domain name, see
[API provider: Share your private custom domain name using AWS RAM](./apigateway-private-custom-domains-provider-share.html).
To share a private custom domain name using the API Gateway AWS CLI, you grant other AWS accounts access to create
domain name access associations and invoke your private custom domain name. You do this by updating the
`managementPolicy` for the API Gateway Management service and the `policy` for the
`execute-api` service for your private custom domain name. You also need to grant access for the
API consumer's VPC endpoint in the resource policy for any private APIs mapped to your private custom domain
name.
The API consumer still needs to create a domain name access association in their own account between their
VPC endpoint and your private custom domain name. You can't do this for them.
###### To grant access to your private custom domain name
1. To update the `managementPolicy` for the API Gateway Management service, you create a JSON file that contains the patch operation
to update the policy. The following `patch-managementPolicy.json` replaces the current
`managementPolicy` with an example policy that grants AWS accounts 111122223333
and 444455556666 access to create domain name access associations with the private custom domain
name `private.example.com`.
```
`[{
"op": "replace",
"path": "/managementPolicy",
"value": "{\\"Version\\":\\"2012-10-17\\", \\"Statement\\":[{\\"Effect\\":\\"Allow\\",\\"Principal\\":{\\"AWS\\":[\\"arn:aws:iam::111122223333:root\\", \\"arn:aws:iam::444455556666:root\\"]},\\"Action\\":\\"apigateway:CreateAccessAssociation\\",\\"Resource\\":\\"arn:aws:apigateway:us-west-2:111122223333:/domainnames/private.example.com+abcd1234\\"}]}"
}]`
```
The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html) command updates the
`managementPolicy` using `patch-managementPolicy.json`.
```
`aws apigateway update-domain-name \\
--domain-name private.example.com \\
--domain-name-id abcd1234 \\
--patch-operations file://patch-managementPolicy.json`
```
Once you grant access, you need to notify the API consumer that they can form the domain name access
association. If you use AWS RAM, AWS RAM will do this step for you.
2. To update the `policy` for the `execute-api` service, you create a JSON file that contains the patch operation to update
the policy. The following `patch-policy.json` replaces the current `policy` with an
example policy that grants two VPC endpoints to invoke the private custom domain name
`private.example.com`.
```
`[{
"op": "replace",
"path": "/policy",
"value": "{\\"Version\\": \\"2012-10-17\\", \\"Statement\\": [{\\"Effect\\": \\"Allow\\",\\"Principal\\": \\"\*\\",\\"Action\\": \\"execute-api:Invoke\\",\\"Resource\\": \\"arn:aws:execute-api:us-west-2:111122223333:/domainnames/private.example.com+abcd1234\\"},{\\"Effect\\": \\"Deny\\",\\"Principal\\": \\"\*\\",\\"Action\\": \\"execute-api:Invoke\\",\\"Resource\\": \\"arn:aws:execute-api:us-west-2:111122223333:/domainnames/private.example.com+abcd1234\\",\\"Condition\\": {\\"StringNotEquals\\": {\\"aws:SourceVpce\\": [\\"vpce-abcd1234\\",\\"vpce-xyzz0000\\"]}}}]}"
}]`
```
Use the following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html) command to update the
`policy` using `patch-policy.json`.
```
`aws apigateway update-domain-name \\
--domain-name private.example.com \\
--domain-name-id abcd1234 \\
--patch-operations file://patch-policy.json`
```
## Deny access to your private custom domain name
To stop sharing your private custom domain name, you need to reject the domain name access association
between your private custom domain name and the API consumer's VPC endpoint.
###### To deny access to your private custom domain name
1. The following
`reject-domain-name-access-association` command rejects the domain name access association.
```
`aws apigateway reject-domain-name-access-association \\
--domain-name-access-association-arn arn:aws:apigateway:us-west-2:444455556666:/domainnameaccessassociations/domainname/private.example.com+abcd1234/vpcesource/vpce-abcd1234 \\
--domain-name-arn arn:aws:apigateway:us-west-2:111122223333:/domainnames/private.example.com+abcd1234`
```
2. Modify the `patch-managementPolicy.json` to remove access for the API provider's account to
create a domain name access association with your private custom domain name. The following `patch-managementPolicy.json` removes one account from the
`managementPolicy`:
```
`[{
"op": "replace",
"path": "/managementPolicy",
"value": "{\\"Version\\":\\"2012-10-17\\", \\"Statement\\":[{\\"Effect\\":\\"Allow\\",\\"Principal\\":\\"\*\\",\\"Action\\":\\"apigateway:CreateAccessAssociation\\",\\"Resource\\":\\"arn:aws:apigateway:us-west-2:111122223333:/domainnames/private.example.com+abcd1234\\"}]}"
}]`
```
The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html) command to updates the
`managementPolicy` using `patch-managementPolicy.json`.
```
`aws apigateway update-domain-name \\
--domain-name private.example.com \\
--domain-name-id abcd1234 \\
--patch-operations file://patch-managementPolicy.json`
```
3. Modify the `patch-policy.json` to remove access for the API provider's VPC endpoint to
invoke your private custom domain name. The following `patch-policy.json` removes the VPC endpoint ID from the
`policy`:
```
`[{
"op": "replace",
"path": "/policy",
"value": "{\\"Version\\":\\"2012-10-17\\", \\"Statement\\":[{\\"Effect\\":\\"Allow\\",\\"Principal\\":\\"\*\\",\\"Action\\":\\"execute-api:Invoke\\",\\"Resource\\":\\"arn:aws:execute-api:us-west-2:111122223333:/domainnames/private.example.com+abcd1234\\"},{\\"Effect\\":\\"Deny\\",\\"Principal\\":\\"\*\\",\\"Action\\":\\"execute-api:Invoke\\",\\"Resource\\":\\"arn:aws:execute-api:us-west-2:111122223333:/domainnames/private.example.com+abcd1234\\",\\"Condition\\":{\\"StringNotEquals\\":{\\"aws:SourceVpce\\":\\"vpce-abcd1234\\"}}}]}"
}]`
```
The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html) command updates the
`policy` using `patch-policy.json`.
```
`aws apigateway update-domain-name \\
--domain-name private.example.com \\
--domain-name-id abcd1234 \\
--patch-operations file://patch-policy.json`
```
## Example policies used in this procedure
The following section shows the example policies used in the previous procedure.
The following example policy is for the `managementPolicy` for the Amazon API Gateway Management
service. This policy grants AWS accounts 111122223333 and 444455556666 access to create
domain name access associations with the private custom domain name `private.example.com`.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": {
"AWS": [
"111122223333",
"444455556666"
]
},
"Action": "apigateway:CreateAccessAssociation",
"Resource": "arn:aws:apigateway:us-west-2:`111122223333`:/domainnames/private.example.com+a1b2c3"
}
]
}`
`
```
The following example policy is the policy for the `policy` for the `execute-api`
service. This policy grants VPC endpoints `vpce-abcd1234` and `vpce-xyzz0000` access
to invoke the private custom domain name.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": "arn:aws:execute-api:us-west-2:111122223333:/domainnames/private.example.com+abcd1234"
},
{
"Effect": "Deny",
"Principal": "\*",
"Action": "execute-api:Invoke",
"Resource": "arn:aws:execute-api:us-west-2:111122223333:/domainnames/private.example.com+abcd1234",
"Condition": {
"StringNotEquals": {
"aws:SourceVpce": [
"vpce-abcd1234",
"vpce-xyzz0000"
]
}
}
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API provider: Stop sharing a private custom domain name using AWS RAM
API consumer: Associate your VPC endpoint with a private custom domain name shared with you
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.