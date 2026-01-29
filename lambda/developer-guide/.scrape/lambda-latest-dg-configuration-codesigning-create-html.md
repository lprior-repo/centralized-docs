---
url: https://docs.aws.amazon.com/lambda/latest/dg/configuration-codesigning-create.html
title: Creating code signing configurations for Lambda
word_count: 459
filtered: true
elements_removed: 0
density_score: 0.89
---

Creating code signing configurations for Lambda - AWS Lambda
Creating code signing configurations for Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#configuration-codesigning-create)
[Configuration prerequisites](#config-codesigning-prereqs)[Creating code signing configurations](#config-codesigning-config-console)[Enabling code signing for a function](#config-codesigning-function-console)
# Creating code signing configurations for Lambda
To enable code signing for a function, you create a *code signing configuration* and attach
it to the function. A code signing configuration defines a list of allowed signing profiles and the policy action
to take if any of the validation checks fail.
###### Note
Functions defined as container images do not support code signing.
###### Sections
* [Configuration prerequisites](#config-codesigning-prereqs)
* [Creating code signing configurations](#config-codesigning-config-console)
* [Enabling code signing for a function](#config-codesigning-function-console)
## Configuration prerequisites
Before you can configure code signing for a Lambda function, use AWS Signer to do the following:
* Create one or more [signing profiles](https://docs.aws.amazon.com/signer/latest/developerguide/signing-profiles.html).
* Use a signing profile to [create a signed code package for your function](https://docs.aws.amazon.com/signer/latest/developerguide/lambda-workflow.html).
## Creating code signing configurations
A code signing configuration defines a list of allowed signing profiles and the signature validation
policy.
###### To create a code signing configuration (console)
1. Open the [Code signing configurations
page](https://console.aws.amazon.com/lambda/home#/code-signing-configurations) of the Lambda console.
2. Choose **Create configuration**.
3. For **Description**, enter a descriptive name for the configuration.
4. Under **Signing profiles**, add up to 20 signing profiles to the configuration.
1. For **Signing profile version ARN**, choose a profile version's Amazon Resource Name
(ARN), or enter the ARN.
2. To add an additional signing profile, choose **Add signing profiles**.
3. Under **Signature validation policy**, choose **Warn** or
**Enforce**.
4. Choose **Create configuration**.
## Enabling code signing for a function
To enable code signing for a function, add a code signing configuration to the function.
###### Important
Code signing configurations only prevent new deployments of unsigned code. If you add a code signing configuration to an existing function that has unsigned code, that code keeps running until you deploy a new code package.
###### To associate a code signing configuration with a function (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose the function for which you want to enable code signing.
3. Open the **Configuration** tab.
4. Scroll down and choose **Code signing**.
5. Choose **Edit**.
6. In **Edit code signing**, choose a code signing configuration for this function.
7. Choose **Save**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Code signing
Permissions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.