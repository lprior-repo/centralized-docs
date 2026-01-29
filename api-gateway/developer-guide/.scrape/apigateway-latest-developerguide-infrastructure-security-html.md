---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/infrastructure-security.html
title: Infrastructure security in Amazon API Gateway
word_count: 289
filtered: true
elements_removed: 0
density_score: 0.86
---

Infrastructure security in Amazon API Gateway - Amazon API Gateway
Infrastructure security in Amazon API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#infrastructure-security)
# Infrastructure security in Amazon API Gateway
As a managed service, Amazon API Gateway is protected by AWS global network security. For
information about AWS security services and how AWS protects infrastructure, see [AWS Cloud Security](https://aws.amazon.com/security/). To design your AWS
environment using the best practices for infrastructure security, see [Infrastructure
Protection](https://docs.aws.amazon.com/wellarchitected/latest/security-pillar/infrastructure-protection.html) in *Security Pillar AWS Well‐Architected
Framework*.
You use AWS published API calls to access API Gateway through the network. Clients must
support the following:
* Transport Layer Security (TLS). We require TLS 1.2 and recommend TLS 1.3.
* Cipher suites with perfect forward secrecy (PFS) such as DHE (Ephemeral
Diffie-Hellman) or ECDHE (Elliptic Curve Ephemeral Diffie-Hellman). Most modern systems
such as Java 7 and later support these modes.
You can call these API operations from any network location, but API Gateway does support resource-based access policies, which can include restrictions
based on the source IP address. You can also use resource-based policies to control access from specific Amazon Virtual Private Cloud (Amazon VPC) endpoints or
specific VPCs. Effectively, this isolates network access to a given API Gateway resource from only the specific VPC within the AWS network.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Resilience
Configuration and vulnerability analysis
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.