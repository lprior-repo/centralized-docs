---
url: https://docs.aws.amazon.com/step-functions/latest/dg/security.html
title: Security in AWS Step Functions
word_count: 906
filtered: true
elements_removed: 0
density_score: 0.86
---

Security in AWS Step Functions - AWS Step Functions
Security in AWS Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#security)
[Compliance validation](#compliance)[Resilience](#disaster-recovery-resiliency)[Infrastructure security](#infrastructure-security)
# Security in AWS Step Functions
Cloud security at AWS is the highest priority. As an AWS customer, you benefit from a data center and
network architecture that is built to meet the requirements of the most security-sensitive organizations.
Security is a shared responsibility between AWS and you. The [shared responsibility model](https://aws.amazon.com/compliance/shared-responsibility-model/) describes this as security
*of* the cloud and security *in* the cloud:
* **Security of the cloud** – AWS is responsible for protecting the
infrastructure that runs AWS services in the AWS Cloud. AWS also provides you with services that you can
use securely. Third-party auditors regularly test and verify the effectiveness of our security as part of the
[AWS compliance programs](https://aws.amazon.com/compliance/programs/). To learn about the
compliance programs that apply to AWS Step Functions, see [AWS Services in Scope by Compliance Program](https://aws.amazon.com/compliance/services-in-scope/).
* **Security in the cloud** – Your responsibility is determined by the
AWS service that you use. You are also responsible for other factors including the sensitivity of your data,
your company’s requirements, and applicable laws and regulations.
This documentation helps you understand how to apply the shared responsibility model when using Step Functions. The following topics show you how to configure Step Functions to meet your security and compliance objectives. You also learn how to use other AWS services that help you to monitor and secure your Step Functions resources.
Step Functions uses IAM to control access to other AWS services and resources. For an overview of how IAM works, see [Overview of Access Management](https://docs.aws.amazon.com/IAM/latest/UserGuide/introduction_access-management.html) in the *IAM User Guide*. For an overview of security credentials, see [AWS Security Credentials](https://docs.aws.amazon.com/general/latest/gr/aws-security-credentials.html) in the *Amazon Web Services General Reference*.
## Compliance validation for Step Functions
Third-party auditors assess the security and compliance of AWS Step Functions as part of multiple
AWS compliance programs. These include SOC, PCI, FedRAMP, HIPAA, and others.
For a list of AWS services in scope of specific compliance programs, see [AWS
Services in Scope by Compliance Program](https://aws.amazon.com/compliance/services-in-scope/). For general information, see [AWS Compliance
Programs](https://aws.amazon.com/compliance/programs/).
You can download third-party audit reports using AWS Artifact. For more
information, see [Downloading Reports in AWS Artifact](https://docs.aws.amazon.com/artifact/latest/ug/downloading-documents.html).
Your compliance responsibility when using Step Functions is determined by the sensitivity of your data, your company's compliance objectives, and applicable laws and regulations. AWS provides the following resources to help with compliance:
* [Security and Compliance Quick Start Guides](https://aws.amazon.com/quickstart/?awsf.quickstart-homepage-filter=categories#security-identity-compliance) – These deployment
guides discuss architectural considerations and provide steps for deploying
security- and compliance-focused baseline environments on AWS.
* [Architecting for HIPAA Security and Compliance on Amazon Web Services](https://docs.aws.amazon.com/whitepapers/latest/architecting-hipaa-security-and-compliance-on-aws/aws-step-functions.html) –
This whitepaper describes how companies can use AWS to create HIPAA-compliant
applications.
* [AWS
Compliance Resources](https://aws.amazon.com/compliance/resources/) – This collection of workbooks and guides might apply
to your industry and location.
* [Evaluating Resources
with Rules](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html) in the *AWS Config Developer Guide* –
The AWS Config service assesses how well your resource configurations comply with
internal practices, industry guidelines, and regulations.
* [AWS Security Hub CSPM](https://docs.aws.amazon.com/securityhub/latest/userguide/what-is-securityhub.html)
– This AWS service provides a comprehensive view of your security state
within AWS that helps you check your compliance with security industry standards
and best practices.
## Resilience in Step Functions
The AWS global infrastructure is built around AWS Regions and Availability Zones.
AWS Regions provide multiple physically separated and isolated Availability Zones, which
are connected with low-latency, high-throughput, and highly redundant networking. With
Availability Zones, you can design and operate applications and databases that automatically
fail over between zones without interruption. Availability Zones are more highly available,
fault tolerant, and scalable than traditional single or multiple data center
infrastructures.
For more information about AWS Regions and Availability Zones, see [AWS Global
Infrastructure](https://aws.amazon.com/about-aws/global-infrastructure/).
In addition to the AWS global infrastructure, Step Functions offers several features to help
support your data resiliency and backup needs.
## Infrastructure security in Step Functions
As a managed service, is protected by AWS global network security. For
information about AWS security services and how AWS protects infrastructure, see [AWS Cloud Security](https://aws.amazon.com/security/). To design your AWS
environment using the best practices for infrastructure security, see [Infrastructure
Protection](https://docs.aws.amazon.com/wellarchitected/latest/security-pillar/infrastructure-protection.html) in *Security Pillar AWS Well‐Architected
Framework*.
You use AWS published API calls to access through the network. Clients must
support the following:
* Transport Layer Security (TLS). We require TLS 1.2 and recommend TLS 1.3.
* Cipher suites with perfect forward secrecy (PFS) such as DHE (Ephemeral
Diffie-Hellman) or ECDHE (Elliptic Curve Ephemeral Diffie-Hellman). Most modern systems
such as Java 7 and later support these modes.
You can call the AWS API operations from any network location, but Step Functions doesn't support
resource-based access policies, which can include restrictions based on the source IP
address. You can also use Step Functions policies to control access from specific Amazon Virtual Private Cloud (Amazon VPC) endpoints or specific VPCs. Effectively, this isolates network
access to a given Step Functions resource from only the specific VPC within the AWS network.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
AWS Step Functions
Data protection
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.