---
url: https://docs.aws.amazon.com/lambda/latest/api/API_CodeSigningPolicies.html
title: CodeSigningPolicies
word_count: 97
filtered: true
elements_removed: 0
density_score: 0.93
---

CodeSigningPolicies - AWS Lambda
CodeSigningPolicies - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_CodeSigningPolicies)
[Contents](#API_CodeSigningPolicies_Contents)[See Also](#API_CodeSigningPolicies_SeeAlso)
# CodeSigningPolicies
Code signing configuration [policies](https://docs.aws.amazon.com/lambda/latest/dg/configuration-codesigning.html#config-codesigning-policies) specify the validation failure action for signature mismatch or
expiry.
## Contents
**
UntrustedArtifactOnDeployment
**
Code signing configuration policy for deployment validation failure. If you set the policy to
`Enforce`, Lambda blocks the deployment request if signature validation checks
fail. If you set the policy to `Warn`, Lambda allows the deployment and issues a
new Amazon CloudWatch metric (`SignatureValidationErrors`) and also stores the
warning in the CloudTrail log.
Default value: `Warn`
Type: String
Valid Values: `Warn | Enforce`
Required: No