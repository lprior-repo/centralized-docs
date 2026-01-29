---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals-create-portal-product.html
title: Create a portal product in API Gateway
word_count: 711
filtered: true
elements_removed: 0
density_score: 0.85
---

Create a portal product in API Gateway - Amazon API Gateway
Create a portal product in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals-create-portal-product)
[Considerations](#apigateway-portals-portal-product-considerations)[Create a portal product](#apigateway-portals-portal-product-create)
# Create a portal product in API Gateway
The following procedure shows how to create a portal product. A portal is a collection of
*portal products*. After
you create your portal product, you create product REST endpoints and product pages. To learn about portal products,
see [Portal products in API Gateway](./apigateway-portals-portal-product.html).
## Considerations
The following considerations might impact how you create a portal product:
* Your portal product can contain both private and public REST APIs. Private APIs aren't supported for the
try it functionality and, as a result, have a visual difference in your portal. As a portal owner, you might
need to provide documentation to explain this.
* If you create your portal product using the AWS CLI or AWS SDKs, your portal won't have any product
endpoints or product pages. You need to add these resources using the AWS CLI or console. To learn how to create
a product REST endpoint, see [Create a product REST endpoint in API Gateway](./apigateway-portals-create-product-rest-endpoint.html). To learn how
to create a product page, see [Create a product page in API Gateway](./apigateway-portals-create-product-page.html).
## Create a portal product
The following procedure shows how to create a portal product.
###### To create a portal product
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Portal products**.
3. Choose **Create product**.
4. For **Product name**, enter the name of your portal product.
5. For **Product description**, enter a description.
6. Choose **Next**.
7. To select your product REST endpoints, under **API endpoints** choose an API, and then
choose a stage.
8. To add an endpoint to your product REST endpoints, select the API endpoint, and then choose **Add
to product**.
###### Note
Do not choose **Next** without first choosing **Add to product**.
![Portal product](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/apigateway-portal-product.png)
The API endpoint will appear in the **Selected API endpoints** list.
9. Choose **Next**.
10. Review your selection and choose **Create product**.
After you create your portal product using the console, all your product pages and product REST endpoint pages
are drafts and won't appear in a portal. To have your product pages and product REST endpoint pages visible to
consumers, you need to add your draft to a section. If you create your portal product using the AWS CLI or
AWS SDKs, you add the draft to the section in the AWS CLI command. Regardless of how you add your draft to a
section, you must publish the portal that uses your portal products for it to be visible to consumers.
###### To add your draft to a page section
1. Your drafts are listed in the **Documentation** tab. There are **Draft
documentation pages** for your product pages and **Draft API reference pages** for
your product REST endpoint pages. Choose **Draft API reference pages**.
2. Choose a draft API reference page.
If you don't have any product REST endpoints, you won't have any draft API reference pages. To learn how
to create a product REST endpoint, see [Create a product REST endpoint in API Gateway](./apigateway-portals-create-product-rest-endpoint.html).
3. Choose **Edit page**.
4. On this page, you can overwrite any existing API documentation parts or use the API Gateway documentation. To
allow the contents of your product REST endpoint page to be visible to consumers, under
**Section name**, enter a name. If this was the `/pets-GET` endpoint, the page
name could be `Pets`.
5. Choose **Save changes**.
6. The new page name you created appears under the **API reference pages** section.
To allow the new page to be visible to consumers, you still need to republish your portal. For more
information, see [Publish a portal in API Gateway](./apigateway-portals-publish-portal.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Portal product
Create a product REST endpoint
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.