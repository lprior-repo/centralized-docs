---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/security-traffic-privacy.html
title: Internetwork traffic privacy
word_count: 234
filtered: true
elements_removed: 0
density_score: 0.84
---

Internetwork traffic privacy - Amazon API Gateway
Internetwork traffic privacy - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#security-traffic-privacy)
# Internetwork traffic privacy
Using Amazon API Gateway, you can create private REST APIs that can be accessed only from your
Amazon Virtual Private Cloud (VPC). The VPC uses an [interface
VPC endpoint](https://docs.aws.amazon.com/vpc/latest/userguide/vpce-interface.html), which is an endpoint network interface that you create in your
VPC. Using [ resource
policies](./apigateway-private-api-create.html#apigateway-private-api-set-up-resource-policy), you can allow or deny access to your API from selected VPCs and VPC
endpoints, including across AWS accounts. Each endpoint can be used to access multiple
private APIs. You can also use Direct Connect to establish a connection from an on-premises
network to Amazon VPC and access your private API over that connection. In all cases, traffic to
your private API uses secure connections and does not leave the Amazon network; it is
isolated from the public internet. To learn more, see [Private REST APIs in API Gateway](./apigateway-private-apis.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Data encryption
Identity and access management
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.