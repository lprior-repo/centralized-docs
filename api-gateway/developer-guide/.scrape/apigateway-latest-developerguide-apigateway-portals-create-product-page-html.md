---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals-create-product-page.html
title: Create a product page in API Gateway
word_count: 605
filtered: true
elements_removed: 0
density_score: 0.87
---

Create a product page in API Gateway - Amazon API Gateway
Create a product page in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals-create-product-page)
[Considerations](#apigateway-portals-create-product-page-considerations)[Create a product page](#apigateway-portals-create-product-page-create)
# Create a product page in API Gateway
A product page helps your product consumers understand and use your products. The documentation is at the
product-level, so any portals that include your product have this documentation. API Gateway supports two types of
product pages written in [GitHub Flavored Markdown](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax):
Overview page
When you create your product, API Gateway automatically generates an **Overview**
page for your product based on the information you provide when you create your product. This page has additional
formatting that can't be changed. You can modify the text or remove the page from the portal product.
Custom pages
A custom page is a page written in Markdown. You might use this section for terms and conditions or a
tutorial on how to use your product. You can change any custom page into an overview page. When you change a
custom page into an overview page, the page content is formatted to match the overview page.
## Considerations
The following considerations might impact your use of product pages:
* This documentation is not at the API-level and doesn't include any methods, resources, or any request parameters.
To learn how to modify the documentation at the API-level for a product, see [Update a product REST endpoint in API Gateway](./apigateway-portals-update-product-endpoint.html).
* You can't directly upload a `.md` file to your product.
* You can modify the order of the product pages by modifying the page list in the display order of your
portal product. For more information, see [Update a portal product in API Gateway](./apigateway-portals-update-portal-product.html).
* You need to republish any portals that include your product for documentation changes to take effect.
* If you have multiple portals that use the same portal product, the product pages are the same across all
portals. To have the same product REST endpoints but different product pages, create multiple
products.
## Create a product page
The following procedure shows how to create a product page.
###### To create a product page
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose
**Portal products**.
3. Choose a product.
4. Under **Documentation**, choose **Create custom page**.
5. For **Page title**, enter the page title.
6. For **Body**, enter your supplemental documentation using GitHub Flavored Markdown.
The **Preview** section shows how your content appears in a portal. The
final visual style might change based on your portal settings.
7. Choose **Create product page**.
8. Your product page is now a draft. Choose **Draft documentation pages**, and
then choose your product page to finalize the content of the page.
9. Choose **Edit page**.
10. For **Section name**, enter a name or choose an existing section. When you add your
product page to a section, consumers can view your content on a published portal.
11. Choose **Save changes**.
To allow the new page to be visible to consumers, you still need to republish your portal. For more
information, see [Publish a portal in API Gateway](./apigateway-portals-publish-portal.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create a product REST endpoint
Update a portal product
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.