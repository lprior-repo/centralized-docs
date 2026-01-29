---
url: https://docs.aws.amazon.com/lambda/latest/dg/snapstart-security.html
title: Security model for Lambda SnapStart
word_count: 236
filtered: true
elements_removed: 0
density_score: 0.90
---

Security model for Lambda SnapStart - AWS Lambda
Security model for Lambda SnapStart - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#snapstart-security)
# Security model for Lambda SnapStart
Lambda SnapStart supports encryption at rest. Lambda encrypts snapshots with an AWS KMS key. By default,
Lambda uses an AWS managed key. If this default behavior suits your workflow, then you don't need to set up
anything else. Otherwise, you can use the `--kms-key-arn` option in the [create-function](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/create-function.html) or [update-function-configuration](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-function-configuration.html) command to provide an AWS KMS customer managed key. You might do this to control rotation of the
KMS key or to meet the requirements of your organization for managing KMS keys. Customer managed keys incur standard
AWS KMS charges. For more information, see [AWS Key Management Service
pricing](https://aws.amazon.com/kms/pricing/).
When you delete a SnapStart function or function version, all `Invoke` requests to that function or function version fail. Lambda removes all resources associated with deleted snapshots in compliance with the General Data Protection Regulation (GDPR).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Monitoring
Best practices
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.