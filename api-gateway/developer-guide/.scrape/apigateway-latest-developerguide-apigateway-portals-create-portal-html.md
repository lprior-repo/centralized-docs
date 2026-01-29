---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-portals-create-portal.html
title: Create a portal in API Gateway
word_count: 1019
filtered: true
elements_removed: 0
density_score: 0.85
---

Create a portal in API Gateway - Amazon API Gateway
Create a portal in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-portals-create-portal)
[Choose a domain name](#apigateway-portals-create-portal-domain-name)[Create a portal](#apigateway-portals-create-portal-create)
# Create a portal in API Gateway
A portal is a collection of products. You control the authorization, branding, and publication of your portal.
When you create a portal, its publish status is `Disabled`. This means your portal is not discoverable
on the web. To let consumers access your portal, you need to publish it. Before you publish your portal, you can
also preview it.
API Gateway supports portals on the following browsers:
* Firefox
* Google Chrome
* Microsoft Edge
* Safari
## Choose a domain name
By default, your portal URL is hosted at the following Amazon-owned domain name:
```
`https://p-`portalId`.apigw-portal.`us-east-1`.on.aws`
```
To customize the domain name, you can provide a domain name that you own and an SSL certificate for the custom
domain. The certificate must be managed with [AWS Certificate Manager](https://docs.aws.amazon.com/acm/latest/userguide/) (ACM) in
US East (N. Virginia) and be supported by Amazon CloudFront. For more information, see
[Requirements for using SSL/TLS certificates with CloudFront](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cnames-and-https-requirements.html).
When you use a custom domain name, API Gateway creates a CloudFront distribution, secured in transit with your ACM
certificate. Then, API Gateway provides an alias record, which you add to your DNS configuration. The alias record is
the Amazon-owned domain name. You must add this
alias record to direct traffic to the CloudFront distribution for your custom domain. The domain name you provide must
be unique and not already be used by the aliases list on an existing CloudFront distribution.
If you use your own domain name, API Gateway uses the `TLS\_1\_2` security
policy to protect your portal. For more information, see
[Supported protocols and ciphers between viewers](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/secure-connections-supported-viewer-protocols-ciphers.html). You can't change this value.
## Create a portal
You use the API Gateway console to create and configure your portal.
The following procedures guide you through all required steps. You can
also choose **Skip to Review and create** at the end of
the first procedure to create an empty portal.
The following procedure shows how to create a portal. First, you define your portal details.
###### To define your portal details
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose
**Portals**.
3. Choose **Create portal**.
4. Under portal details, for **Portal name**, enter a name.
5. For **Portal description**, enter a description.
6. For **Portal authorization**, select how to authorize your portal.
* If this is your first time creating a portal, we recommend that you choose
**None**. If you plan on adding any production APIs to your portal, use
**Require authorization**.
* To require portal consumers to authenticate to use your portal, choose **Require
authorization**, and do the following:
1. For **Amazon Cognito user pool**, select your user pool.
The user pool must be in the same Region as your portal.
2. For **Application client ID**, choose the app client ID.
In your app client, you must set the callback URL to the portal's default URL.
3. For **User pool domain**, enter the user pool domain. For more
information, see
[Configuring a user pool domain](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-assign-domain.html).
4. For **Domain configuration**, select the domain name for your portal.
* If this is your first time creating a portal, we recommend that you choose
**Default domain**. Once you publish your portal, it will be
available at
`https://p-`portalId`.apigw-portal.`us-east-1`.on.aws`.
* To provide a custom domain name that you own, select **Custom domain name** and do the following:
1. For **Domain name**, enter a fully qualified domain name.
2. For **ACM certificate**, choose an ACM certificate that covers the domain name.
You'll need to add the alias record that API Gateway
provisions to your DNS record, in order to serve traffic from your portal.
3. To add a CloudWatch RUM app monitor to your portal, for **RUM app monitor**, choose an
app monitor. For more information, see
[CloudWatch RUM](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-RUM.html).
4. Choose **Next**.
After you define your portal details, you add portal products. You can change the portal products in your portal at any
time, but you need to republish your portal for the changes to take effect.
###### To select your portal products
1. From the resource list, select the products to add to your portal.
2. Choose **Next**.
After you select your products, you customize the portal design. To upload a portal logo, you must have an
image uploaded to Amazon S3. You can change these values at any
time, but you need to republish your portal for the changes to take effect.
###### (Optional) To customize your portal
1. For **Portal logo**, provide the S3 URI for your file.
Only `.svg`, `.png`, or `.jpg` are supported.
2. To customize the design of your portal, you have two options:
1. You can choose **Portal theme** to apply a pre-made color theme on your portal.
2. You can use the color selectors to select all your portal design options.
3. Choose **Next**.
After you select the portal design, review the information for the portal.
###### To review your portal
* Review your settings on the review and create page. When you're satisfied with the settings, choose
**Create**.
If you use an Amazon Cognito user pool to control access to your portal, you must set the callback URL to the portal's
default URL in the app client. For more information, see [Application-specific settings with app
clients](https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-client-apps.html).
Your portal won't be available to consumers after you create it. You must
publish your portal for it to be accessible on the internet. For more information, see [Publish a portal in API Gateway](./apigateway-portals-publish-portal.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Delete a portal product
Update a portal
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.