---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals-use-shared-products.html
title: Add a shared portal product to your portal in API Gateway
word_count: 598
filtered: true
elements_removed: 0
density_score: 0.86
---

Add a shared portal product to your portal in API Gateway - Amazon API Gateway
Add a shared portal product to your portal in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals-use-shared-products)
[Considerations](#apigateway-portals-use-shared-products-considerations)[(Optional) Accept the resource share](#apigateway-portals-use-shared-products-accept)[Add a shared product to your portal](#apigateway-portals-use-shared-products-add)
# Add a shared portal product to your portal in API Gateway
As a portal owner, you can use portal products shared with you by other accounts in your portal.
You maintain full control of your portal.
## Considerations
The following considerations might impact how you share portal resources:
* You must accept the product resource share from the product owner to use their product in your portal.
If you and the product owner are in the same Organizations, AWS RAM can complete some sharing steps for you.
* If the product owner unshares a product while it's in your published portal, the product will still be
visible on the portal, but you won't be able to publish the portal again until you remove this product. It's
your responsibility as the portal owner to remove the product from your portal, as the product owner can't
do that for you.
* You can view the portal products, the product pages, and the product REST endpoint pages, but you cannot modify any of these
resources.
* If a product is shared with you, you can't share it with another account.
## (Optional) Accept the resource share
After your product owner creates a resource share, you have **12 hours** to
accept it. If you are in the same organization using AWS Organizations as the product owner, the share is automatically
accepted. If you are in an organization that has automatic shared resources enabled, the resource is
automatically shared with you.
AWS Management Console
To use the AWS Management Console, see [Accepting and rejecting resource share
invitations](https://docs.aws.amazon.com/ram/latest/userguide/working-with-shared-invitations.html) in the *AWS RAM User Guide*.
AWS CLI
To find all the resources shared with you, use the following [get-resource-share-invitations](https://docs.aws.amazon.com/cli/latest/reference/ram/get-resource-share-invitations.html)
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
## Add a shared product to your portal
After you accept the resource share, you add the product to your portal.
AWS Management Console
###### To add a shared product to your portal
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose
**Portals**.
3. Choose a portal.
4. In the **Products** tab, for
**Portal products**, choose
**Add products**.
5. Add a product to your portal. Shared products are shown as `shared` in the products
list.
6. Choose
**Save changes**.
AWS CLI
To find all the portal products shared with you, use the following `get-portal-products`
command:
```
`aws apigatewayv2 get-portal-products \\
--resource-owner OTHER\_ACCOUNTS \\
--region us-west-2`
```
To add a shared portal product to your portal, use the following `update-portal` command:
```
`aws apigateway update-portal \\
--included-portal-product-arns arn:aws:apigateway:us-west-2:111122223333:/portalproducts/p000000000 \\
--region us-west-2`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Share your portal product with a portal owner
Enable try it for a product REST endpoint in your portal
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.