---
url: https://docs.aws.amazon.com/lambda/latest/dg/configuration-codesigning.html
title: Using code signing to verify code integrity with Lambda
word_count: 467
filtered: true
elements_removed: 0
density_score: 0.87
---

Using code signing to verify code integrity with Lambda - AWS Lambda
Using code signing to verify code integrity with Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#configuration-codesigning)
[Signature validation](#config-codesigning-valid)
# Using code signing to verify code integrity with Lambda
Code signing helps ensure that only trusted code is deployed to your Lambda functions. Using AWS Signer, you can create digitally signed code packages for your functions. When you [add a code signing configuration to a function](./configuration-codesigning-create.html), Lambda verifies that all new code deployments are signed by a trusted source. Because code signing validation checks run at deployment time, there is no impact on function execution.
###### Important
Code signing configurations only prevent new deployments of unsigned code. If you add a code signing configuration to an existing function that has unsigned code, that code keeps running until you deploy a new code package.
When you enable code signing for a function, any [layers](./chapter-layers.html) that you add to the function must also be signed by an allowed signing profile.
There is no additional charge for using AWS Signer or code signing for AWS Lambda.
## Signature validation
Lambda performs the following validation checks when you deploy a signed code package to your function:
1. **Integrity**: Validates that the code package has not been modified since it was signed. Lambda
compares the hash of the package with the hash from the signature.
2. **Expiry**: Validates that the signature of the code package has not expired.
3. **Mismatch**: Validates that the code package is signed with an allowed signing profile
4. **Revocation**: Validates that the signature of the code package has not been revoked.
When you create a code signing configuration, you can use the [UntrustedArtifactOnDeployment](https://docs.aws.amazon.com/lambda/latest/api/API_CodeSigningPolicies.html#lambda-Type-CodeSigningPolicies-UntrustedArtifactOnDeployment) parameter to specify how Lambda should respond if the expiry, mismatch, or revocation checks fail. You can choose one of these actions:
* `Warn`: This is the default setting. Lambda allows the deployment of the
code package, but issues a warning. Lambda issues a new Amazon CloudWatch metric
(`SignatureValidationErrors`) and also stores the warning in the CloudTrail log.
* `Enforce` Lambda issues a warning (the same as for the `Warn` action) and blocks the deployment of the
code package.
###### Topics
* [Creating code signing configurations for Lambda](./configuration-codesigning-create.html)
* [Configuring IAM policies for Lambda code signing configurations](./config-codesigning-policies.html)
* [Using tags on code signing configurations](./tags-csc.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Securing workloads with public endpoints
Create configuration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.