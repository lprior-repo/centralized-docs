---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals-create-product-rest-endpoint.html
title: Create a product REST endpoint in API Gateway
word_count: 673
filtered: true
elements_removed: 0
density_score: 0.86
---

Create a product REST endpoint in API Gateway - Amazon API Gateway
Create a product REST endpoint in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals-create-product-rest-endpoint)
[Create a product REST endpoint](#apigateway-portals-create-product-rest-endpoint-create)
# Create a product REST endpoint in API Gateway
A product REST endpoint is an access point to your portal product. Each endpoint consists of the path and method
of a REST API and the stage it's deployed to. The endpoint includes any documentation you've defined for your REST
API, or can be updated with custom documentation. When you create your product REST endpoint, you configure the following
settings:
* The product REST endpoint's operation name. If your REST API has the path and method `GET /pets`, in a
portal, the name is `/pets`. Using the operation name, you can change the product REST
endpoint name to `Pet summaries`. This doesn't impact your existing API.
* The try it functionality for your product endpoint. This lets API consumers try your APIs in your portal. If
you create a product endpoint for a private API, the try it functionality is set to `UNAVAILABLE`.
For more information, see [Enable try it for an API Gateway product REST endpoint in your portal](./apigateway-portals-try-it.html).
* If you currently have documentation parts defined for your REST API, you can either import the documentation
or override your existing documentation and create new product-level documentation using [GitHub Flavored Markdown](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax). Any documentation overrides
do not propagate back to your API Gateway API documentation parts. If you import your current documentation parts, API Gateway
syncs changes across your API to your product REST endpoint.
## Create a product REST endpoint
The following procedure shows how to add a product REST endpoint to an existing portal product. To learn how
to create a portal product and a product REST endpoint together, see [Create a portal product in API Gateway](./apigateway-portals-create-portal-product.html).
###### To create a product REST endpoint for a product portal
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Portal products**.
3. Choose a portal product.
4. Choose the **Associated endpoints** tab, and then choose
**Add endpoints**.
5. To select your product REST endpoints, under **API endpoints** choose an API, and then
choose a stage.
6. To add an endpoint to your product REST endpoints, select the API endpoint, and then choose
**Add to product**.
The API endpoint will appear in the **Selected API endpoints** list.
7. Choose **Submit**.
8. Your product REST endpoint page is now a draft. Choose **Draft API reference pages**, and
then choose your product REST endpoint page to finalize the content of the page.
9. Choose **Edit page**.
10. To override the existing API Gateway documentation, for **Documentation source**, turn on
**Override the documentation** and do the following:
1. For **Display content**, choose **Create override**.
2. For **Operation name**, enter a new operation name.
3. For **Page body**, enter your custom documentation.
4. (Optional) For **Endpoint**, enter a custom endpoint that appears in your
portal.
5. To let customers invoke your API in your portal, select
**Try it functionality**. For more information, see [Enable try it for an API Gateway product REST endpoint in your portal](./apigateway-portals-try-it.html).
6. For **Section name**, enter a name or choose an existing section. When you add your
product REST endpoint to a section, consumers can view your content on a published portal.
7. Choose **Save changes**.
To allow the new page to be visible to consumers, you still need to republish your portal. For more
information, see [Publish a portal in API Gateway](./apigateway-portals-publish-portal.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create a portal product
Create a product page
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.