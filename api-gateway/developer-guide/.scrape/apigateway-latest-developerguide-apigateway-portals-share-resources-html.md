---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals-share-resources.html
title: Share portal products in API Gateway
word_count: 573
filtered: true
elements_removed: 0
density_score: 0.80
---

Share portal products in API Gateway - Amazon API Gateway
Share portal products in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals-share-resources)
[Considerations](#apigateway-portals-share-resources-considerations)
# Share portal products in API Gateway
You can share a portal product across AWS accounts using AWS RAM. When you share a portal product, the other
account can use your portal product in their own portal. With shared portal products, you can create a single catalog of your
organization's APIs and enforce governance standards across your API ecosystem. At the same time, sharing portal products
provides flexibility for API providers to develop, test, and maintain APIs in their own accounts.
## Considerations
The following considerations might impact how you share portal resources:
* When you share your product with another account, that account cannot modify any properties of your REST
API. This includes the integration endpoints, the authorization strategy, or the stage configuration.
* When you add another account's portal product into your portal, the portal product owner cannot view or control any
other properties of your portal. The portal product owner only knows that the product is being used in
your portal.
* API Gateway portal products are shared at the AWS Region level.
* You can use one resource share with multiple principals, and after you create the resource share, you
can add more principals to it. We recommend that when possible, you reuse your resource
share.
* If both accounts are in the same organization using AWS Organizations,
the resource share is automatically accepted. You still need to create the resource share using
AWS RAM.
* If both accounts are in the same organization using AWS Organizations and
resource sharing within your organization is enabled, any principals in the organization that you share
with are automatically granted access to the resource shares. There is no need for an invitation and you
can skip the resource share.
* If the account you shared the product with doesn't accept the resource share within **12
hours**, you must share the resource
again.
* After you create the resource share, AWS RAM updates the product sharing policy of your product to prevent
access to principals without explicit `allow` access. For more information, see [Determining
whether a request is allowed or denied within an account](https://docs.aws.amazon.com//IAM/latest/UserGuide/reference_policies_evaluation-logic.html#policy-eval-denyallow) in the IAM User Guide.
The updated resource policy will look like the following:
```
`{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"apigateway:GetProduct",
"apigateway:ListEndpoints",
"apigateway:ListPages",
"apigateway:GetEndpoint",
"apigateway:GetPage"
],
"Resource": [
"arn:aws:apigateway:us-east-1:111122223333:/portalproducts/`product-id`",
"arn:aws:apigateway:us-east-1:111122223333:/portalproducts/`product-id`/\*"
]
}
]
}`
```
AWS RAM has prevented principals without explicit allow access to add your product to their portal, by adding
the following:
```
`"StringNotEquals": { "aws:PrincipalAccount": "555555555555" }`
```
To learn how to share a product, see [Share your portal product with a portal owner in API Gateway](./apigateway-portals-share-products.html). To learn
how to add a product shared with you to your portal, see
[Add a shared portal product to your portal in API Gateway](./apigateway-portals-use-shared-products.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Update product page
Share your portal product with a portal owner
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.