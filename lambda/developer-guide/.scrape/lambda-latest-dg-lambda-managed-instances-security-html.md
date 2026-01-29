---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-security.html
title: Security and permissions
word_count: 406
filtered: true
elements_removed: 0
density_score: 0.90
---

Security and permissions - AWS Lambda
Security and permissions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-security)
[Key Security Concepts](#lambda-managed-instances-key-security-concepts)[Required Permissions](#lambda-managed-instances-required-permissions)[Best Practices](#lambda-managed-instances-security-best-practices)[Next steps](#lambda-managed-instances-security-next-steps)
# Security and permissions
Lambda Managed Instances use **capacity providers as trust boundaries**. Functions execute in containers within these instances, but containers do not provide security isolation between workloads. All functions assigned to the same capacity provider must be mutually trusted.
## Key Security Concepts
* **Capacity Provider**: The security boundary that defines trust levels for Lambda functions
* **Container Isolation**: Containers are not a security boundary - do not rely on them for security between untrusted workloads
* **Trust Separation**: Separate workloads that are not mutually trusted by using different capacity providers
### PassCapacityProvider Action
Users need the `lambda:PassCapacityProvider` permission to assign functions to capacity providers. This permission acts as a security gate, ensuring only authorized users can place functions in specific capacity providers.
Account administrators control which functions can use specific capacity providers through the `lambda:PassCapacityProvider` IAM action. This action is required when:
* Creating functions that use Lambda Managed Instances
* Updating function configurations to use a capacity provider
* Deploying functions via infrastructure as code
**Example IAM Policy**
```
`{
"Version": "2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "lambda:PassCapacityProvider",
"Resource": "arn:aws:lambda:\*:\*:capacity-provider:trusted-workloads-\*"
}
]
}`
```
### Service-Linked Role
AWS Lambda uses the `AWSServiceRoleForLambda` service-linked role to manage Lambda Managed Instances ec2 resources in your capacity providers.
## Best Practices
1. **Separate by Trust Level**: Create different capacity providers for workloads with different security requirements
2. **Use Descriptive Names**: Name capacity providers to clearly indicate their intended use and trust level (e.g., `production-trusted`, `dev-sandbox`)
3. **Apply Least Privilege**: Grant `PassCapacityProvider` permissions only for necessary capacity providers
4. **Monitor Usage**: Use AWS CloudTrail to monitor capacity provider assignments and access patterns
## Next steps
* Learn about [capacity providers for Lambda Managed Instances](./lambda-managed-instances-capacity-providers.html)
* Understand [scaling for Lambda Managed Instances](./lambda-managed-instances-scaling.html)
* Configure [VPC connectivity for your capacity providers](./lambda-managed-instances-networking.html)
* Review runtime-specific guides for [Java](./lambda-managed-instances-java-runtime.html), [Node.js](./lambda-managed-instances-nodejs-runtime.html), and [Python](./lambda-managed-instances-python-runtime.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Scaling
Operator role
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.