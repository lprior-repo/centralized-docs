---
url: https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars-encryption.html
title: Securing Lambda environment variables
word_count: 921
filtered: true
elements_removed: 0
density_score: 0.85
---

Securing Lambda environment variables - AWS Lambda
Securing Lambda environment variables - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#configuration-envvars-encryption)
[Managing permissions to your server-side
encryption KMS key](#managing-permissions-to-your-server-side-encryption-key)
# Securing Lambda environment variables
For securing your environment variables, you can use server-side encryption to protect your data at rest and
client-side encryption to protect your data in transit.
###### Note
To increase database security, we recommend that you use AWS Secrets Manager instead of environment variables to store
database credentials. For more information, see [Use Secrets Manager secrets in Lambda functions](./with-secrets-manager.html).
###### Security at rest
Lambda always provides server-side encryption at rest with an AWS KMS key. By default, Lambda uses an
AWS managed key. If this default behavior suits your workflow, you don't need to set up anything else. Lambda
creates the AWS managed key in your account and manages the permissions for you. AWS doesn't charge you to
use this key.
If you prefer, you can provide an AWS KMS customer managed key instead. You might do this to have control over rotation of
the KMS key or to meet the requirements of your organization for managing KMS keys. When you use a customer managed key,
only users in your account with access to the KMS key can view or manage environment variables on the
function.
Customer managed keys incur standard AWS KMS charges. For more information, see [AWS Key Management Service pricing](https://aws.amazon.com/kms/pricing/).
###### Security in transit
For additional security, you can enable helpers for encryption in transit, which ensures that your
environment variables are encrypted client-side for protection in transit.
###### To configure encryption for your environment variables
1. Use the AWS Key Management Service (AWS KMS) to create any customer managed keys for Lambda to use for server-side and client-side
encryption. For more information, see [Creating keys](https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html) in the
*AWS Key Management Service Developer Guide*.
2. Using the Lambda console, navigate to the **Edit environment variables** page.
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Choose **Configuration**, then choose **Environment variables** from
the left navigation bar.
4. In the **Environment variables** section, choose **Edit**.
5. Expand **Encryption configuration**.
6. (Optional) Enable console encryption helpers to use client-side encryption to protect your data in
transit.
1. Under **Encryption in transit**, choose **Enable helpers for encryption in
transit**.
2. For each environment variable that you want to enable console encryption helpers for, choose
**Encrypt** next to the environment variable.
3. Under AWS KMS key to encrypt in transit, choose a customer managed key that you created at the beginning of
this procedure.
4. Choose **Execution role policy** and copy the policy. This policy grants permission
to your function's execution role to decrypt the environment variables.
Save this policy to use in the last step of this procedure.
5. Add code to your function that decrypts the environment variables. To see an example, choose
**Decrypt secrets snippet**.
6. (Optional) Specify your customer managed key for encryption at rest.
1. Choose **Use a customer master key**.
2. Choose a customer managed key that you created at the beginning of this procedure.
3. Choose **Save**.
4. Set up permissions.
If you're using a customer managed key with server-side encryption, grant permissions to any users or roles that you
want to be able to view or manage environment variables on the function. For more information, see [Managing permissions to your server-side
encryption KMS key](#managing-permissions-to-your-server-side-encryption-key).
If you're enabling client-side encryption for security in transit, your function needs permission to call
the `kms:Decrypt` API operation. Add the policy that you saved previously in this procedure to the
function's [execution role](./lambda-intro-execution-role.html).
## Managing permissions to your server-side
encryption KMS key
No AWS KMS permissions are required for your user or the function's execution role to use the default
encryption key. To use a customer managed key, you need permission to use the key. Lambda uses your permissions
to create a grant on the key. This allows Lambda to use it for encryption.
* `kms:ListAliases` – To view keys in the Lambda console.
* `kms:CreateGrant`, `kms:Encrypt` – To configure a customer managed key on a
function.
* `kms:Decrypt` – To view and manage environment variables that are encrypted with a
customer managed key.
You can get these permissions from your AWS account or from a key's resource-based permissions policy.
`ListAliases` is provided by the [managed policies for
Lambda](./access-control-identity-based.html). Key policies grant the remaining permissions to users in the **Key users**
group.
Users without `Decrypt` permissions can still manage functions, but they can't view environment
variables or manage them in the Lambda console. To prevent a user from viewing environment variables, add a
statement to the user's permissions that denies access to the default key, a customer managed key, or all keys.
###### Example IAM policy – Deny access by key ARN
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "VisualEditor0",
"Effect": "Deny",
"Action": [
"kms:Decrypt"
],
"Resource": "arn:aws:kms:us-east-2:`111122223333`:key/3be10e2d-xmpl-4be4-bc9d-0405a71945cc"
}
]
}`
`
```
For details on managing key permissions, see [Key policies in AWS KMS](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html) in the
*AWS Key Management Service Developer Guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Environment variables
Attaching functions to a VPC
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.