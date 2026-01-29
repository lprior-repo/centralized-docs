---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals-share-products.html
title: Share your portal product with a portal owner in API Gateway
word_count: 511
filtered: true
elements_removed: 0
density_score: 0.84
---

Share your portal product with a portal owner in API Gateway - Amazon API Gateway
Share your portal product with a portal owner in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals-share-products)
[Considerations](#apigateway-portals-share-products-considerations)[Share your portal product](#apigateway-portals-share-products-share)
# Share your portal product with a portal owner in API Gateway
As a product owner, you can share your product with a portal owner in another AWS account using AWS RAM. You
maintain full control of all product pages and product REST endpoint pages.
## Considerations
The following considerations might impact how you share portal products:
* You can share or unshare your portal products at any time. If you unshare a portal product while it's
being used in a published portal, the portal owner won't be able to view the portal product, view any updates
made to it, or republish or preview the portal until they remove the portal product.
* You can view the portal products that you've shared with other accounts.
* You can't modify any portals that contain your portal product, unless you create the portal
yourself.
## Share your portal product
The following procedure shows how to create a resource share.
AWS Management Console
To use the AWS Management Console, see
[Creating a
resource share in AWS RAM](https://docs.aws.amazon.com/ram/latest/userguide/working-with-sharing-create.html) in the *AWS RAM User Guide*.
For **Select resource type**, choose
**API Gateway Product**.
AWS CLI
The following [create-resource-share](https://docs.aws.amazon.com/cli/latest/reference/ram/create-resource-share.html)
creates a resource share for your private custom domain name. It can take a few minutes for the resource and principal associations to complete.
For principals, provide an account ID or an Organizations ID, such as
`arn:aws:organizations::123456789012:organization/o-1234abcd`. You can provide multiple principals for your resource share.
```
`aws ram create-resource-share \\
--region us-west-2 \\
--name portal-product-resource-share \\
--permission-arns arn:aws:ram::aws:permission/AWSRAMDefaultPermissionAPIGatewayDeveloperPortalProduct \\
--resource-arns arn:aws:apigateway:us-west-2:111122223333:/portalproducts/p000000000 \\
--principals 222222222222`
```
To unshare your portal product, use AWS RAM to delete the resource share.
At any time, you can modify the product sharing policy to modify which principals can use your portal
products in their portals.
###### To update the product sharing policy
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose
**Portal products**.
3. Choose a product.
4. In the **Product sharing** tab, for
**Product sharing policy**, choose
**Manage product sharing policy**.
5. If you haven't shared your product before, your product sharing policy will deny all access to other
accounts to your portal products. You need to update the policy to allow access for certain accounts to your
portal product.
6. After you have updated your product sharing policy, choose
**Save changes**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Share portal products
Add a shared portal product to your portal
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.