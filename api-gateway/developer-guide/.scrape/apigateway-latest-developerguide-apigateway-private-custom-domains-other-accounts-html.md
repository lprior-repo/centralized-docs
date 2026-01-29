---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-private-custom-domains-other-accounts.html
title: Working with cross-account private custom domain names
word_count: 461
filtered: true
elements_removed: 0
density_score: 0.86
---

Working with cross-account private custom domain names - Amazon API Gateway
Working with cross-account private custom domain names - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-private-custom-domains-other-accounts)
[Best practices for working with
cross-account private custom domain names](#apigateway-private-custom-domains-other-accounts-best-practices)
# Working with cross-account private custom domain names
This section explains how to work with cross-account private custom domain names. You can provide a private
custom domain name to another AWS account and use another AWS account to invoke a private custom domain
name.
You can share your private custom domain name to another AWS account using AWS Resource Access Manager or API Gateway. AWS Resource Access Manager
(AWS RAM) helps you securely share your resources across AWS accounts and within your organization or organizational
units (OUs). For more information, see
[What is AWS Resource Access Manager](https://docs.aws.amazon.com/ram/latest/userguide/what-is.html).
For instructions on how to share a private custom domain name with another AWS account using AWS RAM, see [API provider: Share your private custom domain name using AWS RAM](./apigateway-private-custom-domains-provider-share.html).
For instructions on how to share a private custom domain name with another AWS account using API Gateway, see [API provider: Share your private custom domain name using the API Gateway AWS CLI](./apigateway-private-custom-domains-provider-share-cli.html).
For instructions on how to consume a private custom domain name in another AWS account, see [API consumer: Associate your VPC endpoint with a private custom domain name shared with you](./apigateway-private-custom-domains-consumer-create.html).
## Best practices for working with
cross-account private custom domain names
We recommend the following best practices for working with cross-account private custom domain names:
* Use AWS RAM to share your private custom domain names. When you use AWS RAM, you can reduce operational overhead and you don't have to create a
`managementPolicy` for the Amazon API Gateway Management service.
* Use the `resource-owner` parameter when you list your private custom
domain names or domain name access associations. Use the `resource-owner` parameter to only list the
resources owned by you or by other AWS accounts.
The following example shows how to get all domain name access associations that you own:
```
`aws apigateway get-domain-name-access-associations --resource-owner SELF`
```
Use `--resource-owner OTHER\_ACCOUNTS` to list all the domain name access associations that other
accounts have formed with your private custom domain name.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Create and invoke a custom domain name for private APIs
API provider: Share your private custom domain name using AWS RAM
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.