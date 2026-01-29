---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-private-custom-domains-provider-stop-sharing.html
title: API provider: Stop sharing a private custom domain name using AWS RAM
word_count: 708
filtered: true
elements_removed: 0
density_score: 0.88
---

API provider: Stop sharing a private custom domain name using AWS RAM - Amazon API Gateway
API provider: Stop sharing a private custom domain name using AWS RAM - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-private-custom-domains-provider-stop-sharing)
[Stop sharing your private custom domain name](#apigateway-private-custom-domains-provider-dissociate-ram)[Reject the domain name access association](#apigateway-private-custom-domains-provider-reject)[Deny the API provider access to invoke your private custom
domain name](#apigateway-private-custom-domains-provider-deny-access)
# API provider: Stop sharing a private custom domain name using AWS RAM
To stop sharing your private custom domain name, first you stop the API consumer from creating more domain
name access associations by dissociating the resource share. Then, you reject the domain name access
association and remove the API consumer's VPC endpoint from your `policy` for the
`execute-api` service. The API consumer can then delete their
domain name access association.
## Stop sharing your private custom domain name
First, you stop the resource share using AWS RAM.
AWS Management Console
To use the AWS Management Console, see [Update a resource share in AWS RAM](https://docs.aws.amazon.com/ram/latest/userguide/working-with-sharing-update.html).
AWS CLI
The following [disassociate-resource-share](https://docs.aws.amazon.com/cli/latest/reference/ram/disassociate-resource-share.html) disassociates a resource share for your private custom domain
name.
```
`aws ram disassociate-resource-share \\
--region us-west-2 \\
--resource-arns arn:aws:apigateway:us-west-2:111122223333:/domainnames/private.example.com+abcd1234 \\
--principals 222222222222`
```
## Reject the domain name access association
After you stop sharing your resource using AWS RAM, you reject the domain name access association between a VPC endpoint in another account and
your private custom domain name.
###### Note
You can't reject a domain name access association in your own account. To
stop resource sharing, delete the domain name access association. For more information, see [Delete a domain name access association](./apigateway-private-custom-domains-tutorial.html#apigateway-private-custom-domains-cleanup).
When you reject a domain name access association with
a VPC endpoint, if an API consumer tries to call your private custom domain name, API Gateway rejects the call and
returns a `403` status code.
AWS Management Console
###### To reject a domain name access association
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Custom domain names**.
3. Choose the private custom domain name that you shared with other AWS accounts.
4. On the **Resource sharing**, choose the domain name access association you want
to reject.
5. Choose **Reject association**.
6. Confirm your choice, and then choose **Reject**.
AWS CLI
The following `reject-domain-name-access-association` command rejects the domain name access association between the VPC
endpoint and your private custom domain name:
```
`aws apigateway reject-domain-name-access-association \\
--domain-name-access-association-arn arn:aws:apigateway:us-west-2:444455556666:/domainnameaccessassociations/domainname/private.example.com+abcd1234/vpcesource/vpce-abcd1234efg \\
--domain-name-arn arn:aws:apigateway:us-west-2:111122223333:/domainnames/private.example.com+abcd1234`
```
## Deny the API provider access to invoke your private custom
domain name
After you reject the domain name access association, you remove the API consumer's VPC endpoint from your `policy` for the
`execute-api` service.
AWS Management Console
###### To remove the API consumer's VPC endpoint from your resource policy
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Custom domain names**.
3. Choose the private custom domain name that you shared with other AWS accounts.
4. On the **Resource policy** tab, choose **Edit**.
5. Remove the VPC endpoint from the policy.
6. Choose **Save changes**.
AWS CLI
The following [update-domain-name](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-domain-name.html) command uses a patch operation to update the `policy` for
the `execute-api` service for a private custom domain name. This new
`policy` removes an additional VPC endpoint ID added in
[Allow other accounts to invoke your
private custom domain name](./apigateway-private-custom-domains-provider-share.html#apigateway-private-custom-domains-provider-policy-update):
```
`aws apigateway update-domain-name
--domain-name private.example.com \\
--domain-name-id abcd1234 \\
--patch-operations op=replace,path=/policy,value='"{\\"Version\\": \\"2012-10-17\\", \\"Statement\\": [{\\"Effect\\": \\"Allow\\",\\"Principal\\": \\"\*\\",\\"Action\\": \\"execute-api:Invoke\\",\\"Resource\\":[\\"execute-api:/\*\\"]},{\\"Effect\\": \\"Deny\\",\\"Principal\\": \\"\*\\",\\"Action\\": \\"execute-api:Invoke\\",\\"Resource\\":[\\"execute-api:/\*\\"],\\"Condition\\":{\\"StringNotEquals\\":{\\"aws:SourceVpce\\": \\"vpce-abcd1234efg\\"}}}]}"`
```
The API consumer should then delete the domain name access association. You can't delete it for them. For
more information, see [API consumer: Delete your domain name access association with a private custom domain name](./apigateway-private-custom-domains-consumer-delete-domain-name-access-association.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API provider: Share your private custom domain name using AWS RAM
API provider: Share your private custom domain name using the API Gateway AWS CLI
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.