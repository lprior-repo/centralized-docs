---
url: https://docs.aws.amazon.com/lambda/latest/dg/governance-code-signing.html
title: Lambda code signing with AWS Signer
word_count: 675
filtered: true
elements_removed: 0
density_score: 0.85
---

Lambda code signing with AWS Signer - AWS Lambda
Lambda code signing with AWS Signer - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#governance-code-signing)
# Lambda code signing with AWS Signer
[AWS Signer](https://docs.aws.amazon.com/signer/latest/developerguide/Welcome.html) is a fully managed code-signing service that allows you to validate your code
against a digital signature to confirm that code is unaltered and from a trusted publisher. AWS Signer
can be used in conjunction with AWS Lambda to verify that functions and layers are unaltered prior
to deployment into your AWS environments. This protects your organization from malicious actors
who might have gained credentials to create new or update existing
functions.
To set up code signing for your Lambda functions, start by creating an S3
bucket with versioning enabled. After that, create a signing profile with AWS Signer, specify
Lambda as the platform and then specify a period of days in which the signing profile is valid. Example:
```
` Signer:
Type: AWS::Signer::SigningProfile
Properties:
PlatformId: AWSLambda-SHA384-ECDSA
SignatureValidityPeriod:
Type: DAYS
Value: !Ref pValidDays`
```
Then use the signing profile and create a signing configuration with Lambda. You have to
specify what to do when the signing configuration sees an artifact that does not match a digital
signature that it expected: warn (but allow the deployment) or enforce (and block the deployment).
The example below is configured to enforce and block deployments.
```
` SigningConfig:
Type: AWS::Lambda::CodeSigningConfig
Properties:
AllowedPublishers:
SigningProfileVersionArns:
- !GetAtt Signer.ProfileVersionArn
CodeSigningPolicies:
UntrustedArtifactOnDeployment: Enforce`
```
You now have AWS Signer configured with Lambda to block untrusted deployments. Let's assume you've
finished coding a feature request and are now ready to deploy the function. The first step is
to zip the code up with the appropriate dependencies and then sign the artifact using the signing
profile that you created. You can do this by uploading the zip artifact to the S3 bucket and then
starting a signing job.
```
`aws signer start-signing-job \\
--source 's3={bucketName=`your-versioned-bucket`,key=`your-prefix/your-zip-artifact.zip`,version=`QyaJ3c4qa50LXV.9VaZgXHlsGbvCXxpT`}' \\
--destination 's3={bucketName=`your-versioned-bucket`,prefix=`your-prefix`/}' \\
--profile-name `your-signer-id``
```
You get an output as follows, where the `jobId` is the object that is created in
the destination bucket and prefix and `jobOwner` is the 12-digit AWS account ID where
the job was run.
```
`{
"jobId": "87a3522b-5c0b-4d7d-b4e0-4255a8e05388",
"jobOwner": "111122223333"
}`
```
And now you can deploy your function using the signed S3 object and the code signing configuration
that you created.
```
` Fn:
Type: AWS::Serverless::Function
Properties:
CodeUri: s3://your-versioned-bucket/your-prefix/87a3522b-5c0b-4d7d-b4e0-4255a8e05388.zip
Handler: fn.handler
Role: !GetAtt FnRole.Arn
CodeSigningConfigArn: !Ref pSigningConfigArn`
```
You can alternatively test a function deployment with the original unsigned source zip artifact.
The deployment should fail with the following message:
```
`Lambda cannot deploy the function. The function or layer might be signed using a signature that the client is not configured to accept. Check the provided signature for unsigned.`
```
If you are building and deploying your functions using the AWS Serverless Application Model (AWS SAM), the package command handles uploading the zip artifact to S3 and also starts the signing job and gets the signed
artifact. You can do this with the following command and parameters:
```
`sam package -t your-template.yaml \\
--output-template-file `your-output.yaml` \\
--s3-bucket `your-versioned-bucket` \\
--s3-prefix `your-prefix` \\
--signing-profiles `your-signer-id``
```
AWS Signer helps you verify that zip artifacts that are deployed into your accounts are trusted for deployment. You can include the process above in your CI/CD pipelines and require that all
functions have a code signing configuration attached using the techniques outlined in previous topics. By using code signing with your Lambda function deployments, you prevent malicious
actors who might have gotten credentials to create or update functions from injecting malicious
code in your functions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Detective controls with AWS Config
Code scanning
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.