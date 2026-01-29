---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-private-custom-domains-consumer-delete-domain-name-access-association.html
title: API consumer: Delete your domain name access association with a private custom domain name
word_count: 266
filtered: true
elements_removed: 0
density_score: 0.91
---

API consumer: Delete your domain name access association with a private custom domain name - Amazon API Gateway
API consumer: Delete your domain name access association with a private custom domain name - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-private-custom-domains-consumer-delete-domain-name-access-association)
# API consumer: Delete your domain name access association with a private custom domain name
If you are an API consumer, at any time, you can delete the access association resource. The API provider
can't delete the domain name access association for you.
We recommend that you always delete a domain name access association when you're no longer using
it.
AWS Management Console
###### To delete the domain name access association
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. In the main navigation pane, choose **Domain name access associations**.
3. Select your domain name access association, and then choose **Delete**.
4. Confirm your choice, and then choose **Delete**.
AWS CLI
The following `delete-access-association` command deletes the
access association:
```
`aws apigateway delete-domain-name-access-association \\
--domain-name-access-association-arn 'arn:aws:apigateway:us-west-2:444455556666:/domainnameaccessassociations/domainname/private.example.com+abcd1234/vpcesource/vpce-abcd1234efg'`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API consumer: Associate your VPC endpoint with a private custom domain name shared with you
Create a custom domain name for private APIs using CloudFormation
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.