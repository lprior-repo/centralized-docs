---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals-publish-portal.html
title: Publish a portal in API Gateway
word_count: 287
filtered: true
elements_removed: 0
density_score: 0.89
---

Publish a portal in API Gateway - Amazon API Gateway
Publish a portal in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals-publish-portal)
[Considerations](#apigateway-portals-publish-considerations)[Publish a portal](#apigateway-portals-publish-procedure)
# Publish a portal in API Gateway
For API consumers to access your portal, you must publish it. A portal URL can be discovered by anyone
on the internet. We recommend that you preview and secure your portal before publishing it.
## Considerations
It might take API Gateway a few minutes to publish your portal. You can monitor the
**Publish status** in the console.
## Publish a portal
The following procedure shows how to publish a portal.
###### To publish a portal
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose
**Portals**.
3. Choose a portal.
4. Choose **Publish portal**.
5. (Optional) For **Description of changes**, enter a description of your change.
When you publish a portal, we recommend that you always provide a brief description of your changes.
6. Choose **Publish**.
It takes API Gateway a few minutes to finish publishing your portal. API Gateway provides a link to your portal when
it's available.
To delete your portal, you must disable it first. For more information, see [Disable a portal in API Gateway](./apigateway-portals-disable-portal.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Preview a portal
Use a portal
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.