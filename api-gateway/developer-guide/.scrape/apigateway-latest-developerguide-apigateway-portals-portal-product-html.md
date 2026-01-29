---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals-portal-product.html
title: Portal products in API Gateway
word_count: 589
filtered: true
elements_removed: 0
density_score: 0.88
---

Portal products in API Gateway - Amazon API Gateway
Portal products in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals-portal-product)
[Pet adoption portal product example](#apigateway-portals-portal-product-example)[Next steps
](#apigateway-portals-portal-product-next-steps)
# Portal products in API Gateway
A *portal product* represents a service or functionality that you want to share. Your portal
product is a collection of product REST endpoints and product pages. *Product REST endpoints* are
the access points to your portal product, and they consist of the path and method of a REST API and the stage it's
deployed to. *Product pages* are documentation that you provide to explain how API consumers can
use your product endpoints. A portal product can contain the entire `Prod` stage as a collection of
product REST endpoints or just the `GET /pets` resource deployed to the
`Prod` stage as a singular product REST endpoint.
Your portal product is customizable. You can add custom documentation, rename the product REST endpoints,
reorganize the display order, add new sections, and share products across AWS accounts. In order for any changes
you make to your portal product to take effect, you must republish any portals that use your portal product.
## Pet adoption portal product example
As an example, you could have multiple REST APIs that represent a pet adoption service. You could use API Gateway to
create a `pet adoption` portal product. This portal product would help customers discover which APIs
they should use to meet and adopt pets. This portal product uses REST APIs that you already created, but it allows
you to regroup and organize them. You can also provide documentation about the terms and conditions of using your
pet adoption portal product and let customers try out your APIs. This information is all stored in your portal
product.
The following table shows three APIs that represent a pet adoption portal product, and their corresponding
product REST endpoint operation names and page section names.
|REST API ID|REST API path and method|REST API stage|Operation name|Page section|
|`kf5387miad`|`GET /dogs`|Prod|`View dogs`|`AdoptAnimals`|
|`kf5387miad`|`GET /dogs/{dogId}`|Prod|`View dog`|`AdoptAnimals`|
|`ra8obxcevg`|`GET /cats`|Prod|`View cats`|`AdoptAnimals`|
|`ra8obxcevg`|`GET /cats/{catId}`|Prod|`View cat`|`AdoptAnimals`|
|`h0rpx9cm62`|` ANY /user/{userId}/{petId+}`|Beta|`Request visit`|`AdoptProcess`|
In this example, two REST APIs, `ra8obxcevg` and `kf5387miad`, are grouped together in the
`AdoptAnimals` section. The result of this ordering navigation would look like the following in a
portal:
![Pet adoption portal](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/apigateway-portal.png)
Because the `ANY` method is a catch-all method, the portal shows all the supported HTTP methods.
This portal also contains documentation that was created by the portal product owner.
## Next steps
To get started with portal products, you can do the following:
* To create a portal product, see [Create a portal product in API Gateway](./apigateway-portals-create-portal-product.html).
* To learn about the try it functionality, see [Enable try it for an API Gateway product REST endpoint in your portal](./apigateway-portals-try-it.html).
* To learn about product pages, see [Create a product page in API Gateway](./apigateway-portals-create-product-page.html).
* To learn about sharing your portal product, see [Share portal products in API Gateway](./apigateway-portals-share-resources.html).
After you create a portal product, you can publish it to a portal. For more information, see [Create a portal in API Gateway](./apigateway-portals-create-portal.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API Gateway portals
Create a portal product
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.