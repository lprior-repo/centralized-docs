---
url: https://docs.aws.amazon.com/lambda/latest/dg/file-processing-app.html
title: Create a serverless file-processing app
word_count: 5147
filtered: true
elements_removed: 0
density_score: 0.86
---

Create a serverless file-processing app - AWS Lambda
Create a serverless file-processing app - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#file-processing-app)
[Create the source code files](#file-processing-app-download)[Deploy the app](#file-processing-app-deploy)[Test the app](#file-processing-app-test)[Next steps](#file-processing-app-next-steps)
# Create a serverless file-processing app
One of the most common use cases for Lambda is to perform file processing tasks. For example, you might use a Lambda function
to automatically create PDF files from HTML files or images, or to create thumbnails when a user uploads an image.
In this example, you create an app which automatically encrypts PDF files when they are uploaded to an Amazon Simple Storage Service (Amazon S3) bucket.
To implement this app, you create the following resources:
* An S3 bucket for users to upload PDF files to
* A Lambda function in Python which reads the uploaded file and creates an encrypted, password-protected version of it
* A second S3 bucket for Lambda to save the encrypted file in
You also create an AWS Identity and Access Management (IAM) policy to give your Lambda function permission to perform read and write operations
on your S3 buckets.
![Diagram showing flow of data between an S3 bucket, a Lambda function and another S3 bucket](https://docs.aws.amazon.com/images/lambda/latest/dg/images/ExampleApps/file_process_resources.png)
###### Tip
If you’re brand new to Lambda, we recommend that you start with the tutorial [Create your first Lambda function](./getting-started.html) before
creating this example app.
You can deploy your app manually by creating and configuring resources with the AWS Management Console or the AWS Command Line Interface (AWS CLI). You can
also deploy the app by using the AWS Serverless Application Model (AWS SAM). AWS SAM is an infrastructure as code (IaC) tool. With IaC, you don’t create
resources manually, but define them in code and then deploy them automatically.
If you want to learn more about using Lambda with IaC before deploying this example app, see [Using Lambda with infrastructure as code (IaC)](./foundation-iac.html).
## Create the Lambda function source code files
Create the following files in your project directory:
* `lambda\_function.py` - the Python function code for the Lambda function that performs the file encryption
* `requirements.txt` - a manifest file defining the dependencies that your Python function code requires
Expand the following sections to view the code and to learn more about the role of each file. To create the
files on your local machine, either copy and paste the code below, or download the files from the [aws-lambda-developer-guide GitHub repo](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/file-processing-python).
Copy and paste the following code into a file named `lambda\_function.py`.
```
`from pypdf import PdfReader, PdfWriter
import uuid
import os
from urllib.parse import unquote\_plus
import boto3
# Create the S3 client to download and upload objects from S3
s3\_client = boto3.client('s3')
def lambda\_handler(event, context):
# Iterate over the S3 event object and get the key for all uploaded files
for record in event['Records']:
bucket = record['s3']['bucket']['name']
key = unquote\_plus(record['s3']['object']['key']) # Decode the S3 object key to remove any URL-encoded characters
download\_path = f'/tmp/{uuid.uuid4()}.pdf' # Create a path in the Lambda tmp directory to save the file to
upload\_path = f'/tmp/converted-{uuid.uuid4()}.pdf' # Create another path to save the encrypted file to
# If the file is a PDF, encrypt it and upload it to the destination S3 bucket
if key.lower().endswith('.pdf'):
s3\_client.download\_file(bucket, key, download\_path)
encrypt\_pdf(download\_path, upload\_path)
encrypted\_key = add\_encrypted\_suffix(key)
s3\_client.upload\_file(upload\_path, f'{bucket}-encrypted', encrypted\_key)
# Define the function to encrypt the PDF file with a password
def encrypt\_pdf(file\_path, encrypted\_file\_path):
reader = PdfReader(file\_path)
writer = PdfWriter()
for page in reader.pages:
writer.add\_page(page)
# Save the new PDF to a file
with open(encrypted\_file\_path, "wb") as file:
writer.write(file)
# Define a function to add a suffix to the original filename after encryption
def add\_encrypted\_suffix(original\_key):
filename, extension = original\_key.rsplit('.', 1)
return f'{filename}\_encrypted.{extension}'`
```
###### Note
In this example code, a password for the encrypted file (`my-secret-password`) is hardcoded into the
function code. In a production application, don't include sensitive information like passwords in your function code. Instead, [create an AWS Secrets Manager secret](https://docs.aws.amazon.com/secretsmanager/latest/userguide/create_secret.html) and then [use the AWS Parameters and Secrets Lambda extension](./with-secrets-manager.html) to retrieve your credentials in your Lambda function.
The python function code contains three functions - the [handler function](./python-handler.html) that Lambda runs
when your function is invoked, and two separate function named `add\_encrypted\_suffix` and `encrypt\_pdf` that the handler calls to perform the PDF encryption.
When your function is invoked by Amazon S3, Lambda passes a JSON formatted *event* argument to the function that contains details about the
event that caused the invocation. In this case, the information includes name of the S3 bucket and the object keys for the uploaded files.
To learn more about the format of event object for Amazon S3, see [Process Amazon S3 event notifications with Lambda](./with-s3.html).
Your function then uses the AWS SDK for Python (Boto3) to download the PDF files specified in the event object to its local temporary storage directory, before
encrypting them using the [`pypdf`](https://pypi.org/project/pypdf/) library.
Finally, the function uses the Boto3 SDK to store the encrypted file in your S3 destination bucket.
Copy and paste the following code into a file named `requirements.txt`.
```
`boto3
pypdf`
```
For this example, your function code has only two dependencies that aren't part of the standard Python library -
the SDK for Python (Boto3) and the `pypdf` package the function uses to perform the PDF encryption.
###### Note
A version of the SDK for Python (Boto3) is included as part of the Lambda runtime, so your code would run without adding Boto3 to your
function's deployment package. However, to maintain full control of your function's dependencies and avoid possible issues with
version misalignment, best practice for Python is to include all function dependencies in your function's deployment package.
See [Runtime dependencies in Python](./python-package.html#python-package-dependencies) to learn more.
## Deploy the app
You can create and deploy the resources for this example app either manually or by using AWS SAM. In a production environment, we
recommend that you use an IaC tool like AWS SAM to quickly and repeatably deploy whole serverless applications without using manual processes.
To deploy your app manually:
* Create source and destination Amazon S3 buckets
* Create a Lambda function that encrypts a PDF file and saves the encrypted version to an S3 bucket
* Configure a Lambda trigger that invokes your function when objects are uploaded to your source bucket
Before you begin, make sure that [Python](https://www.python.org/downloads/) is installed on your build machine.
#### Create two S3 buckets
First create two S3 buckets. The first bucket is the source bucket you will upload your PDF files to. The second bucket is used by
Lambda to save the encrypted file when you invoke your function.
Console
###### To create the S3 buckets (console)
1. Open the [General purpose buckets](https://console.aws.amazon.com/s3/buckets) page of the Amazon S3 console.
2. Select the AWS Region closest to your geographical location. You can change your region using the drop-down list at the top of the screen.
![Image showing drop down region menu in S3 console](https://docs.aws.amazon.com/images/lambda/latest/dg/images/console_region_select.png)
3. Choose **Create bucket**.
4. Under **General configuration**, do the following:
1. For **Bucket type**, ensure **General purpose** is selected.
2. For **Bucket name**, enter a globally unique name that meets the Amazon S3 [bucket naming rules](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
Bucket names can contain only lower case letters, numbers, dots (.), and hyphens (-).
3. Leave all other options set to their default values and choose **Create bucket**.
4. Repeat steps 1 to 4 to create your destination bucket. For **Bucket name**, enter ``amzn-s3-demo-bucket-encrypted``,
where ``amzn-s3-demo-bucket`` is the name of the source bucket you just created.
AWS CLI
Before you begin, make sure that the [AWS CLI is installed](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) on your build machine.
###### To create the Amazon S3 buckets (AWS CLI)
1. Run the following CLI command to create your source bucket. The name you choose for your bucket must be globally unique and
follow the Amazon S3 [bucket naming rules](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
Names can only contain lower case letters, numbers, dots (.), and hyphens (-). For `region` and `LocationConstraint`,
choose the [AWS Region](https://docs.aws.amazon.com/general/latest/gr/lambda-service.html) closest to your geographical
location.
```
``aws s3api create-bucket --bucket `amzn-s3-demo-bucket` --region `us-east-2` \\
--create-bucket-configuration LocationConstraint=`us-east-2```
```
Later in the tutorial, you must create your Lambda function in the same AWS Region as your source bucket, so make a note of the
region you chose.
2. Run the following command to create your destination bucket. For the bucket name, you must use ``amzn-s3-demo-bucket-encrypted``,
where ``amzn-s3-demo-bucket`` is the name of the source bucket you created in step 1. For `region`
and `LocationConstraint`, choose the same AWS Region you used to create your source bucket.
```
``aws s3api create-bucket --bucket `amzn-s3-demo-bucket-encrypted` --region `us-east-2` \\
--create-bucket-configuration LocationConstraint=`us-east-2```
```
#### Create an execution role
An execution role is an IAM role that grants a Lambda function permission to access AWS services and resources. To give your function read and write access to Amazon S3, you attach the
[AWS managed policy](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_managed-vs-inline.html#aws-managed-policies)
`AmazonS3FullAccess`.
Console
###### To create an execution role and attach the `AmazonS3FullAccess` managed policy (console)
1. Open the [Roles](https://console.aws.amazon.com/iam/home/roles) page in the IAM console.
2. Choose **Create role**.
3. For **Trusted entity type**, select **AWS service**, and for **Use case**,
select **Lambda**.
4. Choose **Next**.
5. Add the `AmazonS3FullAccess` managed policy by doing the following:
1. In **Permissions policies**, enter `AmazonS3FullAccess` into the search bar.
2. Select the checkbox next to the policy.
3. Choose **Next**.
4. In **Role details**, for **Role name** enter `LambdaS3Role`.
5. Choose **Create Role**.
AWS CLI
###### To create an execution role and attach the `AmazonS3FullAccess` managed policy (AWS CLI)
1. Save the following JSON in a file named `trust-policy.json`. This trust policy allows Lambda to use the role’s
permissions by giving the service principal `lambda.amazonaws.com` permission to call the AWS Security Token Service (AWS STS) `AssumeRole`
action.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": {
"Service": "lambda.amazonaws.com"
},
"Action": "sts:AssumeRole"
}
]
}`
`
```
2. From the directory you saved the JSON trust policy document in, run the following CLI command to create the execution role.
```
``aws iam create-role --role-name LambdaS3Role --assume-role-policy-document file://trust-policy.json``
```
3. To attach the `AmazonS3FullAccess` managed policy, run the following CLI command.
```
``aws iam attach-role-policy --role-name LambdaS3Role --policy-arn arn:aws:iam::aws:policy/AmazonS3FullAccess``
```
#### Create the function deployment package
To create your function, you create a *deployment package* containing your function code and its dependencies. For this
application, your function code uses a separate library for the PDF encryption.
###### To create the deployment package
1. Navigate to the project directory containing the `lambda\_function.py` and `requirements.txt`
files you created or downloaded from GitHub earlier and create a new directory named `package`.
2. Install the dependencies specified in the `requirements.txt` file in your `package` directory by running the following command.
```
``pip install -r requirements.txt --target ./package/``
```
3. Create a .zip file containing your application code and its dependencies. In Linux or MacOS, run the following commands from your
command line interface.
```
``cd package
zip -r ../lambda\_function.zip .
cd ..
zip lambda\_function.zip lambda\_function.py``
```
In Windows, use your preferred zip tool to create the `lambda\_function.zip` file. Make sure that your
`lambda\_function.py` file and the folders containing your dependencies are all at the root of the .zip file.
You can also create your deployment package using a Python virtual environment. See [Working with .zip file archives for Python Lambda functions](./python-package.html)
#### Create the Lambda function
You now use the deployment package you created in the previous step to deploy your Lambda function.
Console
###### To create the function (console)
To create your Lambda function using the console, you first create a basic function containing some ‘Hello world’ code. You then
replace this code with your own function code by uploading the.zip file you created in the previous step.
To ensure that your function doesn't time out when encrypting large PDF files, you configure the function's memory and timeout settings.
You also set the function's log format to JSON. Configuring JSON formatted logs is necessary when using the provided test script so it can read the
function's invocation status from CloudWatch Logs to confirm successful invocation.
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Make sure you're working in the same AWS Region you created your S3 bucket in. You can change your region using the drop-down
list at the top of the screen.
![Image showing drop down region menu in Lambda console](https://docs.aws.amazon.com/images/lambda/latest/dg/images/console_region_select.png)
3. Choose **Create function**.
4. Choose **Author from scratch**.
5. Under **Basic information**, do the following:
1. For **Function name**, enter ``EncryptPDF``.
2. For **Runtime** choose **Python 3.12**.
3. For **Architecture**, choose **x86\_64**.
4. Attach the execution role you created in the previous step by doing the following:
1. Expand the **Change default execution role** section.
2. Select **Use an existing role**.
3. Under **Existing role**, select your role (`LambdaS3Role`).
4. Choose **Create function**.
###### To upload the function code (console)
1. In the **Code source** pane, choose **Upload from**.
2. Choose **.zip file**.
3. Choose **Upload**.
4. In the file selector, select your .zip file and choose **Open**.
5. Choose **Save**.
###### To configure the function memory and timeout (console)
1. Select the **Configuration** tab for your function.
2. In the **General configuration** pane, choose **Edit**.
3. Set **Memory** to 256 MB and **Timeout** to 15 seconds.
4. Choose **Save**.
###### To configure the log format (console)
1. Select the **Configuration** tab for your function.
2. Select **Monitoring and operations tools**.
3. In the **Logging configuration** pane, choose **Edit**.
4. For **Logging configuration**, select **JSON**.
5. Choose **Save**.
AWS CLI
###### To create the function (AWS CLI)
* Run the following command from the directory containing your `lambda\_function.zip`
file.For the `region` parameter, replace `us-east-2` with the region you created your
S3 buckets in.
```
``aws lambda create-function --function-name EncryptPDF \\
--zip-file fileb://lambda\_function.zip --handler lambda\_function.lambda\_handler \\
--runtime python3.12 --timeout 15 --memory-size 256 \\
--role arn:aws:iam::`123456789012`:role/LambdaS3Role --region `us-east-2` \\
--logging-config LogFormat=JSON``
```
#### Configure an Amazon S3 trigger to invoke the function
For your Lambda function to run when you upload a file to your source bucket, you need to configure a trigger for your function. You can
configure the Amazon S3 trigger using either the console or the AWS CLI.
###### Important
This procedure configures the S3 bucket to invoke your function every time that an object is created in the bucket. Be sure to
configure this only on the source bucket. If your Lambda function creates objects in the same bucket that invokes it, your function can be
[invoked continuously in a loop](https://serverlessland.com/content/service/lambda/guides/aws-lambda-operator-guide/recursive-runaway). This can result
in un expected charges being billed to your AWS account.
Console
###### To configure the Amazon S3 trigger (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console and choose your function (`EncryptPDF`).
2. Choose **Add trigger**.
3. Select **S3**.
4. Under **Bucket**, select your source bucket.
5. Under **Event types**, select **All object create events**.
6. Under **Recursive invocation**, select the check box to acknowledge that using the same S3 bucket for input
and output is not recommended. You can learn more about recursive invocation patterns in Lambda by reading
[Recursive patterns that cause run-away Lambda functions](https://serverlessland.com/content/service/lambda/guides/aws-lambda-operator-guide/recursive-runaway)
in Serverless Land.
7. Choose **Add**.
When you create a trigger using the Lambda console, Lambda automatically creates a [resource based policy](https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html)
to give the service you select permission to invoke your function.
AWS CLI
###### To configure the Amazon S3 trigger (AWS CLI)
1. Add a [resource based policy](https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html) to your function that allows your Amazon S3 source bucket to invoke your function when you add a file.
A resource-based policy statement gives other AWS services permission to invoke your function. To give Amazon S3 permission to invoke
your function, run the following CLI command. Be sure to replace the `source-account` parameter with your own AWS account ID and to
use your own source bucket name.
```
``aws lambda add-permission --function-name EncryptPDF \\
--principal s3.amazonaws.com --statement-id s3invoke --action "lambda:InvokeFunction" \\
--source-arn arn:aws:s3:::`amzn-s3-demo-bucket` \\
--source-account `123456789012```
```
The policy you define with this command allows Amazon S3 to invoke your function only when an action takes place on your source bucket.
###### Note
Although S3 bucket names are globally unique, when using resource-based policies it is best practice to specify that the
bucket must belong to your account. This is because if you delete a bucket, it is possible for another AWS account to create a
bucket with the same Amazon Resource Name (ARN).
2. Save the following JSON in a file named `notification.json`. When applied to your source bucket, this JSON
configures the bucket to send a notification to your Lambda function every time a new object is added. Replace the AWS account
number and AWS Region in the Lambda function ARN with your own account number and region.
```
`{
"LambdaFunctionConfigurations": [
{
"Id": "EncryptPDFEventConfiguration",
"LambdaFunctionArn": "arn:aws:lambda:`us-east-2:123456789012`:function:EncryptPDF",
"Events": [ "s3:ObjectCreated:Put" ]
}
]
}`
```
3. Run the following CLI command to apply the notification settings in the JSON file you created to your source bucket. Replace
`amzn-s3-demo-bucket` with the name of your own source bucket.
```
``aws s3api put-bucket-notification-configuration --bucket `amzn-s3-demo-bucket` \\
--notification-configuration file://notification.json``
```
To learn more about the `put-bucket-notification-configuration` command and the
`notification-configuration` option, see [put-bucket-notification-configuration](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/s3api/put-bucket-notification-configuration.html)
in the *AWS CLI Command Reference*.
Before you begin, make sure that [Docker](https://docs.docker.com/get-docker/) and [the latest version of the AWS SAMCLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html) are installed on your build machine.
1. In your project directory, copy and paste the following code into a file named `template.yaml`. Replace the placeholder bucket names:
* For the source bucket, replace `amzn-s3-demo-bucket` with any name that complies with the [S3 bucket naming rules](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
* For the destination bucket, replace `amzn-s3-demo-bucket-encrypted` with `&lt;source-bucket-name&gt;-encrypted`, where `&lt;source-bucket&gt;` is the name you chose for your source bucket.
```
`AWSTemplateFormatVersion: '2010-09-09'
Transform: AWS::Serverless-2016-10-31
Resources:
EncryptPDFFunction:
Type: AWS::Serverless::Function
Properties:
FunctionName: EncryptPDF
Architectures: [x86\_64]
CodeUri: ./
Handler: lambda\_function.lambda\_handler
Runtime: python3.12
Timeout: 15
MemorySize: 256
LoggingConfig:
LogFormat: JSON
Policies:
- AmazonS3FullAccess
Events:
S3Event:
Type: S3
Properties:
Bucket: !Ref PDFSourceBucket
Events: s3:ObjectCreated:\*
PDFSourceBucket:
Type: AWS::S3::Bucket
Properties:
BucketName: `amzn-s3-demo-bucket`
EncryptedPDFBucket:
Type: AWS::S3::Bucket
Properties:
BucketName: `amzn-s3-demo-bucket-encrypted``
```
The AWS SAM template defines the resources you create for your app. In this example, the template defines a Lambda function using the
`AWS::Serverless::Function` type and two S3 buckets using the `AWS::S3::Bucket` type. The bucket names specified in the
template are placeholders. Before you deploy the app using AWS SAM, you need to edit the template to rename the buckets with globally unique names that
meet the [S3 bucket naming rules](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html). This step is
explained further in Deploy the resources using AWS SAM.
The definition of the Lambda function resource configures a trigger for the function using the `S3Event` event property. This
trigger causes your function to be invoked whenever an object is created in your source bucket.
The function definition also specifies an AWS Identity and Access Management (IAM) policy to be attached to the function's [execution role](./lambda-intro-execution-role.html).
The [AWS managed policy](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_managed-vs-inline.html#aws-managed-policies)
`AmazonS3FullAccess` gives your function the permissions it needs to read and write objects to Amazon S3.
* Run the following command from the directory in which you saved your `template.yaml`, `lambda\_function.py`,
and `requirements.txt`files.
```
``sam build --use-container``
```
This command gathers the build artifacts for your application and places them in the proper format and location to deploy them. Specifying
the `--use-container` option builds your function inside a Lambda-like Docker container. We use it here so you don't need to have Python 3.12
installed on your local machine for the build to work.
During the build process, AWS SAM looks for the Lambda function code in the location you specified with the `CodeUri`
property in the template. In this case, we specified the current directory as the location (`./`).
If a `requirements.txt` file is present, AWS SAM uses it to gather the specified dependencies. By default, AWS SAM creates a .zip
deployment package with your function code and dependencies. You can also choose to deploy your function as a container image using the
[PackageType](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-function.html#sam-function-packagetype)
property.
* To deploy your application and create the Lambda and Amazon S3 resources specified in your AWS SAM template, run the following
command.
```
``sam deploy --guided``
```
Using the `--guided` flag means that AWS SAM will show you prompts to guide you through the deployment process. For this
deployment, accept the default options by pressing Enter.
During the deployment process, AWS SAM creates the following resources in your AWS account:
* An CloudFormation [stack](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-whatis-concepts.html#cfn-concepts-stacks)
named `sam-app`
* A Lambda function with the name `EncryptPDF`
* Two S3 buckets with the names you chose when you edited the `template.yaml` AWS SAM template file
* An IAM execution role for your function with the name format `sam-app-EncryptPDFFunctionRole-`2qGaapHFWOQ8``
When AWS SAM finishes creating your resources, you should see the following message:
```
`Successfully created/updated stack - sam-app in us-east-2`
```
## Test the app
To test your app, upload a PDF file to your source bucket, and confirm that Lambda creates an encrypted version of the file in your
destination bucket. In this example, you can either test this manually using the console or the AWS CLI, or by using the provided test script.
For production applications, you can use traditional test methods and techniques, such as unit testing, to confirm the
correct functioning of your Lambda function code. Best practice is also to conduct tests like those in the provided test script which perform integration
testing with real, cloud-based resources. Integration testing in the cloud confirms that your infrastructure has been correctly deployed and that events flow
between different services as expected. To learn more, see [How to test serverless functions and applications](./testing-guide.html).
You can test your function manually by adding a PDF file to
your Amazon S3 source bucket. When you add your file to the source bucket, your Lambda function should be automatically invoked and should store an encrypted
version of the file in your target bucket.
Console
###### To test your app by uploading a file (console)
1. To upload a PDF file to your S3 bucket, do the following:
1. Open the [Buckets](https://console.aws.amazon.com/s3/buckets) page of the Amazon S3 console and choose your source bucket.
2. Choose **Upload**.
3. Choose **Add files** and use the file selector to choose the PDF file you want to upload.
4. Choose **Open**, then choose **Upload**.
5. Verify that Lambda has saved an encrypted version of your PDF file in your target bucket by doing the following:
1. Navigate back to the [Buckets](https://console.aws.amazon.com/s3/buckets) page of the Amazon S3 console and choose your destination bucket.
2. In the **Objects** pane, you should now see a file with name format `filename\_encrypted.pdf` (where
`filename.pdf` was the name of the file you uploaded to your source bucket).
To download your encrypted PDF, select the file, then choose **Download**.
3. Confirm that you can open the downloaded file with the password your Lambda function protected it with (`my-secret-password`).
AWS CLI
###### To test your app by uploading a file (AWS CLI)
1. From the directory containing the PDF file you want to upload, run the following CLI command. Replace the `--bucket`
parameter with the name of your source bucket. For the `--key` and `--body` parameters, use the filename of
your test file.
```
``aws s3api put-object --bucket `amzn-s3-demo-bucket` --key `test.pdf` --body `./test.pdf```
```
2. Verify that your function has created an encrypted version of your file and saved it to your target S3 bucket. Run the
following CLI command, replacing `amzn-s3-demo-bucket-encrypted` with the name of your own destination bucket.
```
``aws s3api list-objects-v2 --bucket `amzn-s3-demo-bucket-encrypted```
```
If your function runs successfully, you’ll see output similar to the following. Your target bucket should contain a file with the
name format ``&lt;&lt;your\_test\_file&gt;&gt;`\_encrypted.pdf`, where `&lt;&lt;your\_test\_file&gt;&gt;`
is the name of the file you uploaded.
```
`{
"Contents": [
{
"Key": "test\_encrypted.pdf",
"LastModified": "2023-06-07T00:15:50+00:00",
"ETag": "\\"7781a43e765a8301713f533d70968a1e\\"",
"Size": 2763,
"StorageClass": "STANDARD"
}
]
}`
```
3. To download the file that Lambda saved in your destination bucket, run the following CLI command. Replace the `--bucket`
parameter with the name of your destination bucket. For the `--key` parameter, use the filename ``&lt;&lt;your\_test\_file&gt;&gt;`\_encrypted.pdf`,
where `&lt;&lt;your\_test\_file&gt;&gt;` is the name of the the test file you uploaded.
```
``aws s3api get-object --bucket `amzn-s3-demo-bucket-encrypted` --key `test\_encrypted.pdf` my\_encrypted\_file.pdf``
```
This command downloads the file to your current directory and saves it as `my\_encrypted\_file.pdf`.
4. Confirm the you can open the downloaded file with the password your Lambda function protected it with (`my-secret-password`).
Create the following files in your project directory:
* `test\_pdf\_encrypt.py` - a test script you can use to automatically test your application
* `pytest.ini` - a configuration file for the the test script
Expand the following sections to view the code and to learn more about the role of each file.
Copy and paste the following code into a file named `test\_pdf\_encrypt.py`. Be sure to replace the placeholder bucket names:
* In the `test\_source\_bucket\_available` function, replace `amzn-s3-demo-bucket` with the name of your source bucket.
* In the `test\_encrypted\_file\_in\_bucket` function, replace `amzn-s3-demo-bucket-encrypted` with `*source-bucket*-encrypted`,
where `source-bucket&gt;` is the name of your source bucket.
* In the `cleanup` function, replace `amzn-s3-demo-bucket` with the name of your source bucket, and replace
`amzn-s3-demo-bucket-encrypted` with the name of your destination bucket.
```
`import boto3
import json
import pytest
import time
import os
@pytest.fixture
def lambda\_client():
return boto3.client('lambda')
@pytest.fixture
def s3\_client():
return boto3.client('s3')
@pytest.fixture
def logs\_client():
return boto3.client('logs')
@pytest.fixture(scope='session')
def cleanup():
# Create a new S3 client for cleanup
s3\_client = boto3.client('s3')
yield
# Cleanup code will be executed after all tests have finished
# Delete test.pdf from the source bucket
source\_bucket = 'amzn-s3-demo-bucket'
source\_file\_key = 'test.pdf'
s3\_client.delete\_object(Bucket=source\_bucket, Key=source\_file\_key)
print(f"\\nDeleted {source\_file\_key} from {source\_bucket}")
# Delete test\_encrypted.pdf from the destination bucket
destination\_bucket = 'amzn-s3-demo-bucket-encrypted'
destination\_file\_key = 'test\_encrypted.pdf'
s3\_client.delete\_object(Bucket=destination\_bucket, Key=destination\_file\_key)
print(f"Deleted {destination\_file\_key} from {destination\_bucket}")
@pytest.mark.order(1)
def test\_source\_bucket\_available(s3\_client):
s3\_bucket\_name = 'amzn-s3-demo-bucket'
file\_name = 'test.pdf'
file\_path = os.path.join(os.path.dirname(\_\_file\_\_), file\_name)
file\_uploaded = False
try:
s3\_client.upload\_file(file\_path, s3\_bucket\_name, file\_name)
file\_uploaded = True
except:
print("Error: couldn't upload file")
assert file\_uploaded, "Could not upload file to S3 bucket"
@pytest.mark.order(2)
def test\_lambda\_invoked(logs\_client):
# Wait for a few seconds to make sure the logs are available
time.sleep(5)
# Get the latest log stream for the specified log group
log\_streams = logs\_client.describe\_log\_streams(
logGroupName='/aws/lambda/EncryptPDF',
orderBy='LastEventTime',
descending=True,
limit=1
)
latest\_log\_stream\_name = log\_streams['logStreams'][0]['logStreamName']
# Retrieve the log events from the latest log stream
log\_events = logs\_client.get\_log\_events(
logGroupName='/aws/lambda/EncryptPDF',
logStreamName=latest\_log\_stream\_name
)
success\_found = False
for event in log\_events['events']:
message = json.loads(event['message'])
status = message.get('record', {}).get('status')
if status == 'success':
success\_found = True
break
assert success\_found, "Lambda function execution did not report 'success' status in logs."
@pytest.mark.order(3)
def test\_encrypted\_file\_in\_bucket(s3\_client):
# Specify the destination S3 bucket and the expected converted file key
destination\_bucket = 'amzn-s3-demo-bucket-encrypted'
converted\_file\_key = 'test\_encrypted.pdf'
try:
# Attempt to retrieve the metadata of the converted file from the destination S3 bucket
s3\_client.head\_object(Bucket=destination\_bucket, Key=converted\_file\_key)
except s3\_client.exceptions.ClientError as e:
# If the file is not found, the test will fail
pytest.fail(f"Converted file '{converted\_file\_key}' not found in the destination bucket: {str(e)}")
def test\_cleanup(cleanup):
# This test uses the cleanup fixture and will be executed last
pass`
```
The automated test script executes three test functions to confirm correct operation of your app:
* The test `test\_source\_bucket\_available` confirms that your source bucket has been successfully created
by uploading a test PDF file to the bucket.
* The test `test\_lambda\_invoked` interrogates the latest CloudWatch Logs log stream for your function to confirm that
when you uploaded the test file, your Lambda function ran and reported success.
* The test `test\_encrypted\_file\_in\_bucket` confirms that your destination bucket contains the encrypted `test\_encrypted.pdf`
file.
After all these tests have run, the script runs an additional cleanup step to delete the `test.pdf` and `test\_encrypted.pdf` files from
both your source and destination buckets.
As with the AWS SAM template, the bucket names specified in this file are placeholders. Before running the test, you need to edit this file
with your app's real bucket names. This step is explained further in [Testing the app with the automated script](#file-processing-app-test-auto)
Copy and paste the following code into a file named `pytest.ini`.
```
`[pytest]
markers =
order: specify test execution order`
```
This is needed to specify the order in which the tests in the `test\_pdf\_encrypt.py` script run.
To run the tests do the following:
1. Ensure that the `pytest` module is installed in your local environment. You can install
`pytest` by running the following command:
```
``pip install pytest``
```
2. Save a PDF file named `test.pdf` in the directory containing the `test\_pdf\_encrypt.py` and `pytest.ini` files.
3. Open a terminal or shell program and run the following command from the directory containing the test files.
```
``pytest -s -v``
```
When the test completes, you should see output like the following:
```
`
============================================================== test session starts =========================================================
platform linux -- Python 3.12.2, pytest-7.2.2, pluggy-1.0.0 -- /usr/bin/python3
cachedir: .pytest\_cache
hypothesis profile 'default' -&gt;&gt; database=DirectoryBasedExampleDatabase('/home/pdf\_encrypt\_app/.hypothesis/examples')
Test order randomisation NOT enabled. Enable with --random-order or --random-order-bucket=&lt;&lt;bucket\_type&gt;&gt;
rootdir: /home/pdf\_encrypt\_app, configfile: pytest.ini
plugins: anyio-3.7.1, hypothesis-6.70.0, localserver-0.7.1, random-order-1.1.0
collected 4 items
test\_pdf\_encrypt.py::test\_source\_bucket\_available PASSED
test\_pdf\_encrypt.py::test\_lambda\_invoked PASSED
test\_pdf\_encrypt.py::test\_encrypted\_file\_in\_bucket PASSED
test\_pdf\_encrypt.py::test\_cleanup PASSED
Deleted test.pdf from amzn-s3-demo-bucket
Deleted test\_encrypted.pdf from amzn-s3-demo-bucket-encrypted
=============================================================== 4 passed in 7.32s ==========================================================`
```
## Next steps
Now you've created this example app, you can use the provided code as a basis to create other types of file-processing application. Modify the
code in the `lambda\_function.py` file to implement the file-processing logic for your use case.
Many typical file-processing use cases involve image processing. When using Python, the most popular image-processing libraries like
[pillow](https://pypi.org/project/pillow/) typically contain C or C++ components. In order to ensure that your function's deployment package is
compatible with the Lambda execution environment, it's important to use the correct source distribution binary.
When deploying your resources with AWS SAM, you need to take some extra steps to include the right source distribution in your deployment package. Because AWS SAM won't install dependencies
for a different platform than your build machine, specifying the correct source distribution (`.whl` file) in your `requirements.txt`
file won't work if your build machine uses an operating system or architecture that's different from the Lambda execution environment. Instead, you should do one of the following:
* Use the `--use-container` option when running `sam build`. When you specify this option, AWS SAM downloads a container base image that's
compatible with the Lambda execution environment and builds your function's deployment package in a Docker container using that image. To learn more, see
[Building a Lambda
function inside of a provided container](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/using-sam-cli-build.html#using-sam-cli-build-options-container).
* Build your function's .zip deployment package yourself using the correct source distribution binary and save the .zip file in the directory you specify as the
`CodeUri` in the AWS SAM template. To learn more about building .zip deployment packages for Python using binary distributions, see
[Creating a .zip deployment package with dependencies](./python-package.html#python-package-create-dependencies) and [Creating .zip deployment packages with native libraries](./python-package.html#python-package-native-libraries).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Example apps and patterns
Scheduled-maintenance app
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.