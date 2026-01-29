---
url: https://docs.aws.amazon.com/lambda/latest/dg/deploying-github-actions.html
title: Using GitHub Actions to deploy Lambda functions
word_count: 308
filtered: true
elements_removed: 0
density_score: 0.85
---

Using GitHub Actions to deploy Lambda functions - AWS Lambda
Using GitHub Actions to deploy Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#deploying-github-actions)
[Example workflow](#deploying-github-actions-example)[Additional resources](#deploying-github-actions-resources)
# Using GitHub Actions to deploy Lambda functions
You can use [GitHub Actions](https://github.com/features/actions) to automatically deploy Lambda functions when you push code or configuration changes to your repository. The [Deploy Lambda Function](https://github.com/aws-actions/aws-lambda-deploy) action provides a declarative, simple YAML interface that eliminates the complexity of manual deployment steps.
## Example workflow
To configure automated Lambda function deployment, create a workflow file in your repository's
`.github/workflows/` directory:
###### Example GitHub Actions workflow for Lambda deployment
```
`name: Deploy AWS Lambda
on:
push:
branches:
- main
jobs:
deploy:
runs-on: ubuntu-latest
permissions:
id-token: write # Required for OIDC authentication
contents: read # Required to check out the repository
steps:
- uses: actions/checkout@v4
- name: Configure AWS credentials
uses: aws-actions/configure-aws-credentials@v4
with:
role-to-assume: arn:aws:iam::123456789012:role/GitHubActionRole
aws-region: us-east-1
- name: Deploy Lambda Function
uses: aws-actions/aws-lambda-deploy@v1
with:
function-name: my-lambda-function
code-artifacts-dir: ./dist`
```
This workflow runs when you push changes to the `main` branch. It checks out your repository,
configures AWS credentials using OpenID Connect (OIDC), and deploys your function using the code in the
`./dist` directory.
For additional examples including updating function configuration, deploying via S3 buckets, and dry run validation, see the
[Deploy Lambda Function README](https://github.com/aws-actions/aws-lambda-deploy).
## Additional resources
* [Configure AWS Credentials GitHub Action](https://github.com/aws-actions/configure-aws-credentials)
* [Configuring OpenID Connect in AWS](https://docs.github.com/en/actions/security-for-github-actions/security-hardening-your-deployments/configuring-openid-connect-in-amazon-web-services)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Local development
Infrastructure as code (IaC)
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.