---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals-update-product-endpoint.html
title: Update a product REST endpoint in API Gateway
word_count: 355
filtered: true
elements_removed: 0
density_score: 0.88
---

Update a product REST endpoint in API Gateway - Amazon API Gateway
Update a product REST endpoint in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals-update-product-endpoint)
[Update a product REST endpoint](#apigateway-portals-update-product-endpoint-update)
# Update a product REST endpoint in API Gateway
When you update a product REST endpoint, you can change all the settings of the product endpoint except for the path
and method of a REST API and the stage it's deployed to. Any changes to your product endpoints are updated
automatically, but you need to republish any portals that use your products to see the changes reflected in a
portal.
## Update a product REST endpoint
The following procedure shows how to update a product REST endpoint to overwrite the current API documentation with
custom API reference documentation.
###### To update a product REST endpoint
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose
**Portal products**.
3. Choose a product.
4. In the **Documentation** tab, under **API reference pages**, choose the name of a product REST endpoint, such as
**/dogs - GET**.
5. For **Preview**, choose **Edit page**.
6. For **Documentation source**, turn on **Override the documentation**.
7. For **Display content**, choose
**Override the existing content**.
If you choose **Remove all content**, the content is removed from the page, but you can
choose **Override the existing content** to access the documentation again.
8. In **Page body**, enter custom API reference documentation using [GitHub Flavored Markdown](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax).
This documentation will not propagate back to API Gateway.
9. Choose **Save changes**.
10. You must republish any portals that use this portal product for the changes to take effect.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Update a portal product
Update product page
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.