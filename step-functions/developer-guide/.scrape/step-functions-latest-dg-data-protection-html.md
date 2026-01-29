---
url: https://docs.aws.amazon.com/step-functions/latest/dg/data-protection.html
title: Data protection and encryption in Step Functions
word_count: 563
filtered: true
elements_removed: 0
density_score: 0.85
---

Data protection and encryption in Step Functions - AWS Step Functions
Data protection and encryption in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#data-protection)
# Data protection and encryption in Step Functions
The AWS [shared responsibility model](https://aws.amazon.com/compliance/shared-responsibility-model/)
applies to data protection in AWS Step Functions. As described in this model, AWS is
responsible for protecting the global infrastructure that runs all of the AWS Cloud. You are
responsible for maintaining control over your content that is hosted on this infrastructure.
You are also responsible for the security configuration and management tasks for the AWS services
that you use. For more information about data privacy, see the [Data Privacy FAQ](https://aws.amazon.com/compliance/data-privacy-faq/). For information about data protection in Europe, see the [AWS Shared
Responsibility Model and GDPR](https://aws.amazon.com/blogs/security/the-aws-shared-responsibility-model-and-gdpr/) blog post on the *AWS Security
Blog*.
For data protection purposes, we recommend that you protect AWS account
credentials and set up individual users with AWS IAM Identity Center or AWS Identity and Access Management (IAM). That way, each user is given only the permissions necessary to fulfill their job duties. We also recommend that you secure your data in the following ways:
* Use multi-factor authentication (MFA) with each account.
* Use SSL/TLS to communicate with AWS resources. We require TLS 1.2 and recommend TLS 1.3.
* Set up API and user activity logging with AWS CloudTrail. For information about using CloudTrail trails to capture AWS activities, see [Working with CloudTrail trails](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-trails.html) in the *AWS CloudTrail User Guide*.
* Use AWS encryption solutions, along with all default security controls within AWS services.
* Use advanced managed security services such as Amazon Macie, which assists in discovering
and securing sensitive data that is stored in Amazon S3.
* If you require FIPS 140-3 validated cryptographic modules when accessing AWS through
a command line interface or an API, use a FIPS endpoint. For more information about the
available FIPS endpoints, see [Federal
Information Processing Standard (FIPS) 140-3](https://aws.amazon.com/compliance/fips/).
We strongly recommend that you never put confidential or sensitive information, such as your
customers' email addresses, into tags or free-form text fields such as a **Name** field. This includes when you work with Step Functions or other AWS services
using the console, API, AWS CLI, or AWS SDKs. Any data that you enter into
tags or free-form text fields used for names may be used for billing or diagnostic logs. If you
provide a URL to an external server, we strongly recommend that you do not include credentials
information in the URL to validate your request to that server.
With customer managed AWS KMS keys, you can secure customer data that includes **protected health information (PHI)** from unauthorized access. Step Functions is integrated with CloudTrail, so you can view and audit the most recent events in the CloudTrail console in the event history.
###### Topics
* [Data at rest encryption in Step Functions](./encryption-at-rest.html)
* [Data in transit encryption in Step Functions](./encryption-in-transit.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Securing state machines
Data at rest encryption
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.