---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals-use-portal.html
title: Use an API Gateway portal
word_count: 533
filtered: true
elements_removed: 0
density_score: 0.85
---

Use an API Gateway portal - Amazon API Gateway
Use an API Gateway portal - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals-use-portal)
[Use a portal](#apigateway-portals-use-portal-use)[Troubleshoot your portal](#apigateway-portals-use-portal-troubleshoot)
# Use an API Gateway portal
After you publish your portal, it's available on the web. To use your portal, copy the **Portal default
domain name** or the **domain name** and enter it in a browser. If your portal uses a
domain name, the default domain name can't be used to access your portal.
## Use a portal
The following procedure shows how to access a portal and use try it for a product REST endpoint.
###### To use a portal
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose
**Portals**.
3. Choose a portal.
4. Copy the **Portal default
domain name** or the **domain name** and enter it in a browser.
5. On the portal homepage, you can search for portal products and discover product pages. Use the search at the top of the portal to
search all the content in your portal. Use the search bar
under **Products** to search for portal products.
6. Choose a portal product.
7. Choose a product REST endpoint.
8. In the **Try it** window, enter parameters to try a call to your API.
9. Choose **Try it**. API Gateway will invoke your API and show the results in the window.
## Troubleshoot your portal
The following provides troubleshooting advice for errors and issues that you might encounter when using a portal.
### The portal webpage site can’t be reached
If you receive an error message that the portal webpage site can't be reached, make sure you published your
portal. If you used a domain name, make sure that you added the alias record that API Gateway provided to direct
traffic to the CloudFront distribution for your custom domain.
### Try it failed
If you are unable to use try it for a product REST endpoint, make sure that the try it state is
`ACTIVE`. In addition, make sure you entered the correct parameters to invoke your API.
### Pages are not appearing in your portal
If you created a product REST endpoint page or a product page and it's not visible in your portal, make sure
your pages are not drafts. To verify this, do the following:
###### To verify your pages are not in drafts
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose
**Portal products**.
3. Choose your portal product.
4. In the **Documentation** tab, confirm your pages are not in the **Draft documentation pages** or
**Draft API reference pages** section, but are in the **Custom documentation pages** or
**API reference pages**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Publish a portal
Disable a portal
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.