---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals-update-portal-product.html
title: Update a portal product in API Gateway
word_count: 357
filtered: true
elements_removed: 0
density_score: 0.85
---

Update a portal product in API Gateway - Amazon API Gateway
Update a portal product in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals-update-portal-product)
[Update a portal product](#apigateway-portals-update-portal-product-update)
# Update a portal product in API Gateway
When you update a portal product, you can change the name, description, or the display order of how your product
REST endpoints and product pages appear. To modify the display order, you modify the page list for product pages and
the section name for product endpoints. If you do this using the AWS CLI or SDKs, you specify the product page ARN or
the product REST endpoint ARN. You need to
republish any portals that use your products for your API consumers to see the changes reflected in a portal.
## Update a portal product
The following procedure shows how to change the section list to reorder the product REST endpoints as they
will appear in a portal. In this procedure, we assume you have at least two product REST endpoints in your portal
product.
###### To update a portal product
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose
**Portal products**.
3. Choose a product.
4. In the **Documentation** tab, for
**Product pages**, choose **Manage**.
5. Drag and drop the product REST endpoints to reorder them. You can also move product REST endpoints and
product pages into the draft pages section. Pages in the draft pages are won't be visible in your portal.
6. (Optional) Rename the product
REST endpoint names or add a new API reference section. These changes won't impact your REST APIs.
7. Choose **Save changes**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create a product page
Update a product REST endpoint
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.