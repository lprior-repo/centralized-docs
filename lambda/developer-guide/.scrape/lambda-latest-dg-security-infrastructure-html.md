---
url: https://docs.aws.amazon.com/lambda/latest/dg/security-infrastructure.html
title: Infrastructure security in AWS Lambda
word_count: 214
filtered: true
elements_removed: 0
density_score: 0.87
---

Infrastructure security in AWS Lambda - AWS Lambda
Infrastructure security in AWS Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#security-infrastructure)
# Infrastructure security in AWS Lambda
As a managed service, AWS Lambda is protected by AWS global network security. For
information about AWS security services and how AWS protects infrastructure, see [AWS Cloud Security](https://aws.amazon.com/security/). To design your AWS
environment using the best practices for infrastructure security, see [Infrastructure
Protection](https://docs.aws.amazon.com/wellarchitected/latest/security-pillar/infrastructure-protection.html) in *Security Pillar AWS Well‐Architected
Framework*.
You use AWS published API calls to access Lambda through the network. Clients must
support the following:
* Transport Layer Security (TLS). We require TLS 1.2 and recommend TLS 1.3.
* Cipher suites with perfect forward secrecy (PFS) such as DHE (Ephemeral
Diffie-Hellman) or ECDHE (Elliptic Curve Ephemeral Diffie-Hellman). Most modern systems
such as Java 7 and later support these modes.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Resilience
Securing workloads with public endpoints
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.