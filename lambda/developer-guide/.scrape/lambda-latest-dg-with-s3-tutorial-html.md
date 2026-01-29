---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-s3-tutorial.html
title: Tutorial: Using an Amazon S3 trigger to create thumbnail images
word_count: 4901
filtered: true
elements_removed: 0
density_score: 0.84
---

Tutorial: Using an Amazon S3 trigger to create thumbnail images - AWS Lambda
Tutorial: Using an Amazon S3 trigger to create thumbnail images - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-s3-tutorial)
[Prerequisites](#with-s3-example-prereqs)[Create two Amazon S3 buckets](#with-s3-tutorial-prepare-create-buckets)[Upload a test image to your source bucket](#with-s3-tutorial-test-image)[Create a permissions policy](#with-s3-tutorial-create-policy)[Create an execution role](#with-s3-tutorial-create-execution-role)[Create the function deployment package](#with-s3-tutorial-create-function-package)[Create the Lambda function](#with-s3-tutorial-create-function-createfunction)[Configure Amazon S3 to invoke the function](#with-s3-tutorial-configure-s3-trigger)[Test your Lambda function with a dummy event](#with-s3-tutorial-dummy-test)[Test your function using the Amazon S3 trigger](#with-s3-tutorial-test-s3)[Clean up your resources](#s3-tutorial-cleanup)
# Tutorial: Using an Amazon S3 trigger to create thumbnail images
In this tutorial, you create and configure a Lambda function that resizes images added to an Amazon Simple Storage Service (Amazon S3) bucket. When you add an image
file to your bucket, Amazon S3 invokes your Lambda function. The function then creates a thumbnail version of the image and outputs it to a different
Amazon S3 bucket.
![Data flow between an S3 bucket, a Lambda function, and another S3 bucket](https://docs.aws.amazon.com/images/lambda/latest/dg/images/services-s3-tutorial/s3thumb_tut_resources.png)
To complete this tutorial, you carry out the following steps:
1. Create source and destination Amazon S3 buckets and upload a sample image.
2. Create a Lambda function that resizes an image and outputs a thumbnail to an Amazon S3 bucket.
3. Configure a Lambda trigger that invokes your function when objects are uploaded to your source bucket.
4. Test your function, first with a dummy event, and then by uploading an image to your source bucket.
By completing these steps, you’ll learn how to use Lambda to carry out a file processing task on objects added to an Amazon S3 bucket. You can
complete this tutorial using the AWS Command Line Interface (AWS CLI) or the AWS Management Console.
If you're looking for a simpler example to learn how to configure an Amazon S3 trigger for Lambda, you can try [Tutorial: Using an Amazon S3 trigger to invoke a Lambda function](https://docs.aws.amazon.com/lambda/latest/dg/with-s3-example.html).
###### Topics
* [Prerequisites](#with-s3-example-prereqs)
* [Create two Amazon S3 buckets](#with-s3-tutorial-prepare-create-buckets)
* [Upload a test image to your source bucket](#with-s3-tutorial-test-image)
* [Create a permissions policy](#with-s3-tutorial-create-policy)
* [Create an execution role](#with-s3-tutorial-create-execution-role)
* [Create the function deployment package](#with-s3-tutorial-create-function-package)
* [Create the Lambda function](#with-s3-tutorial-create-function-createfunction)
* [Configure Amazon S3 to invoke the function](#with-s3-tutorial-configure-s3-trigger)
* [Test your Lambda function with a dummy event](#with-s3-tutorial-dummy-test)
* [Test your function using the Amazon S3 trigger](#with-s3-tutorial-test-s3)
* [Clean up your resources](#s3-tutorial-cleanup)
## Prerequisites
If you want to use the AWS CLI to complete the tutorial, install the [latest version of the AWS Command Line Interface]().
For your Lambda function code, you can use Python or Node.js. Install the language support tools and a package manager for the
language that you want to use.
If you have not yet installed the AWS Command Line Interface, follow the steps at [Installing or updating the latest version of the AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
to install it.
The tutorial requires a command line terminal or shell to run commands. In Linux and macOS, use your preferred shell and package manager.
###### Note
In Windows, some Bash CLI commands that you commonly use with Lambda (such as `zip`) are not supported by the operating system's built-in terminals.
To get a Windows-integrated version of Ubuntu and Bash, [install the Windows Subsystem for Linux](https://docs.microsoft.com/en-us/windows/wsl/install-win10).
## Create two Amazon S3 buckets
![First step: Create S3 buckets](https://docs.aws.amazon.com/images/lambda/latest/dg/images/services-s3-tutorial/s3thumb_tut_steps1.png)
First create two Amazon S3 buckets. The first bucket is the source bucket you will upload your images to. The second bucket is used by
Lambda to save the resized thumbnail when you invoke your function.
AWS Management Console
###### To create the Amazon S3 buckets (console)
1. Open the [Amazon S3 console](https://console.aws.amazon.com/s3) and select the **General purpose buckets** page.
2. Select the AWS Region closest to your geographical location. You can change your region using the drop-down list at the top of the screen.
Later in the tutorial, you must create your Lambda function in the same Region.
![Image showing drop down region menu in S3 console](https://docs.aws.amazon.com/images/lambda/latest/dg/images/console_region_select.png)
3. Choose **Create bucket**.
4. Under **General configuration**, do the following:
1. For **Bucket type**, ensure **General purpose** is selected.
2. For **Bucket name**, enter a globally unique name that meets the Amazon S3 [Bucket naming rules](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
Bucket names can contain only lower case letters, numbers, dots (.), and hyphens (-).
3. Leave all other options set to their default values and choose **Create bucket**.
4. Repeat steps 1 to 5 to create your destination bucket. For **Bucket name**, enter ``amzn-s3-demo-source-bucket-resized``,
where ``amzn-s3-demo-source-bucket`` is the name of the source bucket you just created.
AWS CLI
###### To create the Amazon S3 buckets (AWS CLI)
1. Run the following CLI command to create your source bucket. The name you choose for your bucket must be globally unique and
follow the Amazon S3 [Bucket naming rules](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
Names can only contain lower case letters, numbers, dots (.), and hyphens (-). For `region` and `LocationConstraint`,
choose the [AWS Region](https://docs.aws.amazon.com/general/latest/gr/lambda-service.html) closest to your geographical
location.
```
``aws s3api create-bucket --bucket `amzn-s3-demo-source-bucket` --region `us-east-1` \\
--create-bucket-configuration LocationConstraint=`us-east-1```
```
Later in the tutorial, you must create your Lambda function in the same AWS Region as your source bucket, so make a note of the
region you chose.
2. Run the following command to create your destination bucket. For the bucket name, you must use ``amzn-s3-demo-source-bucket-resized``,
where ``amzn-s3-demo-source-bucket`` is the name of the source bucket you created in step 1. For `region`
and `LocationConstraint`, choose the same AWS Region you used to create your source bucket.
```
``aws s3api create-bucket --bucket `amzn-s3-demo-source-bucket-resized` --region `us-east-1` \\
--create-bucket-configuration LocationConstraint=`us-east-1```
```
## Upload a test image to your source bucket
![Next step: Upload a test object](https://docs.aws.amazon.com/images/lambda/latest/dg/images/services-s3-tutorial/s3thumb_tut_steps2.png)
Later in the tutorial, you’ll test your Lambda function by invoking it using the AWS CLI or the Lambda console. To confirm that your function
is operating correctly, your source bucket needs to contain a test image. This image can be any JPG or PNG file you choose.
AWS Management Console
###### To upload a test image to your source bucket (console)
1. Open the [Buckets](https://console.aws.amazon.com/s3/buckets) page of the Amazon S3 console.
2. Select the source bucket you created in the previous step.
3. Choose **Upload**.
4. Choose **Add files** and use the file selector to choose the object you want to upload.
5. Choose **Open**, then choose **Upload**.
AWS CLI
###### To upload a test image to your source bucket (AWS CLI)
* From the directory containing the image you want to upload, run the following CLI command. Replace the `--bucket`
parameter with the name of your source bucket. For the `--key` and `--body` parameters, use the filename of
your test image.
```
``aws s3api put-object --bucket `amzn-s3-demo-source-bucket` --key `HappyFace.jpg` --body `./HappyFace.jpg```
```
## Create a permissions policy
![Next step: Create a permissions policy](https://docs.aws.amazon.com/images/lambda/latest/dg/images/services-s3-tutorial/s3thumb_tut_steps3.png)
The first step in creating your Lambda function is to create a permissions policy. This policy gives your function the permissions it needs
to access other AWS resources. For this tutorial, the policy gives Lambda read and write permissions for Amazon S3 buckets and allows it to write
to Amazon CloudWatch Logs.
AWS Management Console
###### To create the policy (console)
1. Open the [Policies](https://console.aws.amazon.com/iamv2/home#policies) page of the AWS Identity and Access Management (IAM) console.
2. Choose **Create policy**.
3. Choose the **JSON** tab, and then paste the following custom policy into the JSON editor.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"logs:PutLogEvents",
"logs:CreateLogGroup",
"logs:CreateLogStream"
],
"Resource": "arn:aws:logs:\*:\*:\*"
},
{
"Effect": "Allow",
"Action": [
"s3:GetObject"
],
"Resource": "arn:aws:s3:::\*/\*"
},
{
"Effect": "Allow",
"Action": [
"s3:PutObject"
],
"Resource": "arn:aws:s3:::\*/\*"
}
]
}`
`
```
4. Choose **Next**.
5. Under **Policy details**, for **Policy name**, enter ``LambdaS3Policy``.
6. Choose **Create policy**.
AWS CLI
###### To create the policy (AWS CLI)
1. Save the following JSON in a file named `policy.json`.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"logs:PutLogEvents",
"logs:CreateLogGroup",
"logs:CreateLogStream"
],
"Resource": "arn:aws:logs:\*:\*:\*"
},
{
"Effect": "Allow",
"Action": [
"s3:GetObject"
],
"Resource": "arn:aws:s3:::\*/\*"
},
{
"Effect": "Allow",
"Action": [
"s3:PutObject"
],
"Resource": "arn:aws:s3:::\*/\*"
}
]
}`
`
```
2. From the directory you saved the JSON policy document in, run the following CLI command.
```
``aws iam create-policy --policy-name LambdaS3Policy --policy-document file://policy.json``
```
## Create an execution role
![Next step: Create an execution role](https://docs.aws.amazon.com/images/lambda/latest/dg/images/services-s3-tutorial/s3thumb_tut_steps4.png)
An execution role is an IAM role that grants a Lambda function permission to access AWS services and resources. To give your function
read and write access to an Amazon S3 bucket, you attach the permissions policy you created in the previous step.
AWS Management Console
###### To create an execution role and attach your permissions policy (console)
1. Open the [Roles](https://console.aws.amazon.com/iamv2/home#roles) page of the (IAM) console.
2. Choose **Create role**.
3. For **Trusted entity type**, select **AWS service**, and for **Use case**,
select **Lambda**.
4. Choose **Next**.
5. Add the permissions policy you created in the previous step by doing the following:
1. In the policy search box, enter ``LambdaS3Policy``.
2. In the search results, select the check box for `LambdaS3Policy`.
3. Choose **Next**.
4. Under **Role details**, for the **Role name** enter ``LambdaS3Role``.
5. Choose **Create role**.
AWS CLI
###### To create an execution role and attach your permissions policy (AWS CLI)
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
3. To attach the permissions policy you created in the previous step, run the following CLI command. Replace the AWS account number
in the policy’s ARN with your own account number.
```
``aws iam attach-role-policy --role-name LambdaS3Role --policy-arn arn:aws:iam::`123456789012`:policy/LambdaS3Policy``
```
## Create the function deployment package
![Next step: Create the deployment package](https://docs.aws.amazon.com/images/lambda/latest/dg/images/services-s3-tutorial/s3thumb_tut_steps5.png)
To create your function, you create a *deployment package* containing your function code and its dependencies. For this
`CreateThumbnail` function, your function code uses a separate library for the image resizing. Follow the instructions for your
chosen language to create a deployment package containing the required library.
Node.js
###### To create the deployment package (Node.js)
1. Create a directory named `lambda-s3` for your function code and dependencies and navigate into it.
```
``mkdir lambda-s3
cd lambda-s3``
```
2. Create a new Node.js project with `npm`. To accept the default options provided in the interactive experience, press `Enter`.
```
`npm init`
```
3. Save the following function code in a file named `index.mjs`. Make sure to replace `us-east-1` with the
AWS Region in which you created your own source and destination buckets.
```
`// dependencies
import { S3Client, GetObjectCommand, PutObjectCommand } from '@aws-sdk/client-s3';
import { Readable } from 'stream';
import sharp from 'sharp';
import util from 'util';
// create S3 client
const s3 = new S3Client({region: `'us-east-1'`});
// define the handler function
export const handler = async (event, context) =&gt; {
// Read options from the event parameter and get the source bucket
console.log("Reading options from event:\\n", util.inspect(event, {depth: 5}));
const srcBucket = event.Records[0].s3.bucket.name;
// Object key may have spaces or unicode non-ASCII characters
const srcKey = decodeURIComponent(event.Records[0].s3.object.key.replace(/\\+/g, " "));
const dstBucket = srcBucket + "-resized";
const dstKey = "resized-" + srcKey;
// Infer the image type from the file suffix
const typeMatch = srcKey.match(/\\.([^.]\*)$/);
if (!typeMatch) {
console.log("Could not determine the image type.");
return;
}
// Check that the image type is supported
const imageType = typeMatch[1].toLowerCase();
if (imageType != "jpg" &amp;&amp; imageType != "png") {
console.log(`Unsupported image type: ${imageType}`);
return;
}
// Get the image from the source bucket. GetObjectCommand returns a stream.
try {
const params = {
Bucket: srcBucket,
Key: srcKey
};
var response = await s3.send(new GetObjectCommand(params));
var stream = response.Body;
// Convert stream to buffer to pass to sharp resize function.
if (stream instanceof Readable) {
var content\_buffer = Buffer.concat(await stream.toArray());
} else {
throw new Error('Unknown object stream type');
}
} catch (error) {
console.log(error);
return;
}
// set thumbnail width. Resize will set the height automatically to maintain aspect ratio.
const width = 200;
// Use the sharp module to resize the image and save in a buffer.
try {
var output\_buffer = await sharp(content\_buffer).resize(width).toBuffer();
} catch (error) {
console.log(error);
return;
}
// Upload the thumbnail image to the destination bucket
try {
const destparams = {
Bucket: dstBucket,
Key: dstKey,
Body: output\_buffer,
ContentType: "image"
};
const putResult = await s3.send(new PutObjectCommand(destparams));
} catch (error) {
console.log(error);
return;
}
console.log('Successfully resized ' + srcBucket + '/' + srcKey +
' and uploaded to ' + dstBucket + '/' + dstKey);
};`
```
4. In your `lambda-s3` directory, install the sharp library using npm. Note that the latest version of sharp (0.33) isn't
compatible with Lambda. Install version 0.32.6 to complete this tutorial.
```
``npm install sharp@0.32.6``
```
The npm `install` command creates a `node\_modules` directory for your modules. After this step, your
directory structure should look like the following.
```
`lambda-s3
|- index.mjs
|- node\_modules
| |- base64js
| |- bl
| |- buffer
...
|- package-lock.json
|- package.json`
```
5. Create a .zip deployment package containing your function code and its dependencies. In MacOS and Linux, run the following
command.
```
``zip -r function.zip .``
```
In Windows, use your preferred zip utility to create a .zip file. Ensure that your `index.mjs`,
`package.json`, and `package-lock.json` files and your `node\_modules` directory are all at the root
of your .zip file.
Python
###### To create the deployment package (Python)
1. Save the example code as a file named
`lambda\_function.py`.
```
`import boto3
import os
import sys
import uuid
from urllib.parse import unquote\_plus
from PIL import Image
import PIL.Image
s3\_client = boto3.client('s3')
def resize\_image(image\_path, resized\_path):
with Image.open(image\_path) as image:
image.thumbnail(tuple(x / 2 for x in image.size))
image.save(resized\_path)
def lambda\_handler(event, context):
for record in event['Records']:
bucket = record['s3']['bucket']['name']
key = unquote\_plus(record['s3']['object']['key'])
tmpkey = key.replace('/', '')
download\_path = '/tmp/{}{}'.format(uuid.uuid4(), tmpkey)
upload\_path = '/tmp/resized-{}'.format(tmpkey)
s3\_client.download\_file(bucket, key, download\_path)
resize\_image(download\_path, upload\_path)
s3\_client.upload\_file(upload\_path, '{}-resized'.format(bucket), 'resized-{}'.format(key))`
```
2. In the same directory in which you created your `lambda\_function.py` file, create a new directory named
`package` and install the [Pillow (PIL)](https://pypi.org/project/Pillow/) library and the
AWS SDK for Python (Boto3). Although the Lambda Python runtime includes a version of the Boto3 SDK, we recommend that you add all of your function's
dependencies to your deployment package, even if they are included in the runtime. For more information, see
[Runtime dependencies in Python](https://docs.aws.amazon.com/lambda/latest/dg/python-package.html#python-package-dependencies).
```
``mkdir package
pip install \\
--platform manylinux2014\_x86\_64 \\
--target=package \\
--implementation cp \\
--python-version 3.12 \\
--only-binary=:all: --upgrade \\
pillow boto3``
```
The Pillow library contains C/C++ code. By using the `--platform manylinux\_2014\_x86\_64` and `--only-binary=:all:`
options, pip will download and install a version of Pillow that contains pre-compiled binaries compatible with the Amazon Linux 2 operating
system. This ensures that your deployment package will work in the Lambda execution environment, regardless of the operating system and
architecture of your local build machine.
3. Create a .zip file containing your application code and the Pillow and Boto3 libraries. In Linux or MacOS, run the following commands from your
command line interface.
```
`cd package
zip -r ../lambda\_function.zip .
cd ..
zip lambda\_function.zip lambda\_function.py`
```
In Windows, use your preferred zip tool to create the `lambda\_function.zip` file. Make sure that your
`lambda\_function.py` file and the folders containing your dependencies are all at the root of the .zip file.
You can also create your deployment package using a Python virtual environment. See [Working with .zip file archives for Python Lambda functions](./python-package.html)
## Create the Lambda function
![Next step: Create the function](https://docs.aws.amazon.com/images/lambda/latest/dg/images/services-s3-tutorial/s3thumb_tut_steps6.png)
You can create your Lambda function using either the AWS CLI or the Lambda console. Follow the instructions for your chosen language to create
the function.
AWS Management Console
###### To create the function (console)
To create your Lambda function using the console, you first create a basic function containing some ‘Hello world’ code. You then
replace this code with your own function code by uploading the.zip or JAR file you created in the previous step.
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Make sure you're working in the same AWS Region you created your Amazon S3 bucket in. You can change your region using the drop-down
list at the top of the screen.
![Image showing drop down region menu in Lambda console](https://docs.aws.amazon.com/images/lambda/latest/dg/images/console_region_select.png)
3. Choose **Create function**.
4. Choose **Author from scratch**.
5. Under **Basic information**, do the following:
1. For **Function name**, enter ``CreateThumbnail``.
2. For **Runtime**, choose either **Node.js 22.x** or **Python 3.12** according to the language you chose for your function.
3. For **Architecture**, choose **x86\_64**.
4. In the **Change default execution role** tab, do the following:
1. Expand the tab, then choose **Use an existing role**.
2. Select the `LambdaS3Role` you created earlier.
3. Choose **Create function**.
###### To upload the function code (console)
1. In the **Code source** pane, choose **Upload from**.
2. Choose **.zip file**.
3. Choose **Upload**.
4. In the file selector, select your .zip file and choose **Open**.
5. Choose **Save**.
AWS CLI
###### To create the function (AWS CLI)
* Run the CLI command for the language you chose. For the `role` parameter, make sure to replace `123456789012`
with your own AWS account ID. For the `region` parameter, replace `us-east-1` with the region you created your
Amazon S3 buckets in.
* For **Node.js**, run the following command from the directory containing your `function.zip`
file.
```
``aws lambda create-function --function-name CreateThumbnail \\
--zip-file fileb://function.zip --handler index.handler --runtime nodejs24.x \\
--timeout 10 --memory-size 1024 \\
--role arn:aws:iam::`123456789012`:role/LambdaS3Role --region `us-east-1```
```
* For **Python**, run the following command from the directory containing your `lambda\_function.zip`
file.
```
``aws lambda create-function --function-name CreateThumbnail \\
--zip-file fileb://lambda\_function.zip --handler lambda\_function.lambda\_handler \\
--runtime python3.14 --timeout 10 --memory-size 1024 \\
--role arn:aws:iam::`123456789012`:role/LambdaS3Role --region `us-east-1```
```
## Configure Amazon S3 to invoke the function
![Next step: Create the Amazon S3 trigger](https://docs.aws.amazon.com/images/lambda/latest/dg/images/services-s3-tutorial/s3thumb_tut_steps7.png)
For your Lambda function to run when you upload an image to your source bucket, you need to configure a trigger for your function. You can
configure the Amazon S3 trigger using either the console or the AWS CLI.
###### Important
This procedure configures the Amazon S3 bucket to invoke your function every time that an object is created in the bucket. Be sure to
configure this only on the source bucket. If your Lambda function creates objects in the same bucket that invokes it, your function can be
[invoked continuously in a loop](https://serverlessland.com/content/service/lambda/guides/aws-lambda-operator-guide/recursive-runaway). This can result
in un expected charges being billed to your AWS account.
AWS Management Console
###### To configure the Amazon S3 trigger (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console and choose your function (`CreateThumbnail`).
2. Choose **Add trigger**.
3. Select **S3**.
4. Under **Bucket**, select your source bucket.
5. Under **Event types**, select **All object create events**.
6. Under **Recursive invocation**, select the check box to acknowledge that using the same Amazon S3 bucket for input
and output is not recommended. You can learn more about recursive invocation patterns in Lambda by reading
[Recursive patterns that cause run-away Lambda functions](https://serverlessland.com/content/service/lambda/guides/aws-lambda-operator-guide/recursive-runaway)
in Serverless Land.
7. Choose **Add**.
When you create a trigger using the Lambda console, Lambda automatically creates a [resource based policy](https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html)
to give the service you select permission to invoke your function.
AWS CLI
###### To configure the Amazon S3 trigger (AWS CLI)
1. For your Amazon S3 source bucket to invoke your function when you add an image file, you first need to configure permissions for your
function using a [resource based policy](https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html).
A resource-based policy statement gives other AWS services permission to invoke your function. To give Amazon S3 permission to invoke
your function, run the following CLI command. Be sure to replace the `source-account` parameter with your own AWS account ID and to
use your own source bucket name.
```
``aws lambda add-permission --function-name CreateThumbnail \\
--principal s3.amazonaws.com --statement-id s3invoke --action "lambda:InvokeFunction" \\
--source-arn arn:aws:s3:::`amzn-s3-demo-source-bucket` \\
--source-account `123456789012```
```
The policy you define with this command allows Amazon S3 to invoke your function only when an action takes place on your source bucket.
###### Note
Although Amazon S3 bucket names are globally unique, when using resource-based policies it is best practice to specify that the
bucket must belong to your account. This is because if you delete a bucket, it is possible for another AWS account to create a
bucket with the same Amazon Resource Name (ARN).
2. Save the following JSON in a file named `notification.json`. When applied to your source bucket, this JSON
configures the bucket to send a notification to your Lambda function every time a new object is added. Replace the AWS account
number and AWS Region in the Lambda function ARN with your own account number and region.
```
`{
"LambdaFunctionConfigurations": [
{
"Id": "CreateThumbnailEventConfiguration",
"LambdaFunctionArn": "arn:aws:lambda:`us-east-1:123456789012`:function:CreateThumbnail",
"Events": [ "s3:ObjectCreated:Put" ]
}
]
}`
```
3. Run the following CLI command to apply the notification settings in the JSON file you created to your source bucket. Replace
`amzn-s3-demo-source-bucket` with the name of your own source bucket.
```
``aws s3api put-bucket-notification-configuration --bucket `amzn-s3-demo-source-bucket` \\
--notification-configuration file://notification.json``
```
To learn more about the `put-bucket-notification-configuration` command and the
`notification-configuration` option, see [put-bucket-notification-configuration](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/s3api/put-bucket-notification-configuration.html)
in the *AWS CLI Command Reference*.
## Test your Lambda function with a dummy event
![Next step: Create a dummy event](https://docs.aws.amazon.com/images/lambda/latest/dg/images/services-s3-tutorial/s3thumb_tut_steps8.png)
Before you test your whole setup by adding an image file to your Amazon S3 source bucket, you test that your Lambda function is working
correctly by invoking it with a dummy event. An event in Lambda is a JSON-formatted document that contains data for your function to process.
When your function is invoked by Amazon S3, the event sent to your function contains information such as the bucket name, bucket ARN, and object
key.
AWS Management Console
###### To test your Lambda function with a dummy event (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console and choose your
function (`CreateThumbnail`).
2. Choose the **Test** tab.
3. To create your test event, in the **Test event** pane, do the following:
1. Under **Test event action**, select **Create new event**.
2. For **Event name**, enter `myTestEvent`.
3. For **Template**, select **S3 Put**.
4. Replace the values for the following parameters with your own values.
* For `awsRegion`, replace `us-east-1` with the AWS Region you created your Amazon S3 buckets in.
* For `name`, replace `amzn-s3-demo-bucket` with the name of your own Amazon S3 source bucket.
* For `key`, replace `test%2Fkey` with the filename of the test object you uploaded to your source
bucket in the step [Upload a test image to your source bucket](#with-s3-tutorial-test-image).
```
`{
"Records": [
{
"eventVersion": "2.0",
"eventSource": "aws:s3",
"awsRegion": `"us-east-1"`,
"eventTime": "1970-01-01T00:00:00.000Z",
"eventName": "ObjectCreated:Put",
"userIdentity": {
"principalId": "EXAMPLE"
},
"requestParameters": {
"sourceIPAddress": "127.0.0.1"
},
"responseElements": {
"x-amz-request-id": "EXAMPLE123456789",
"x-amz-id-2": "EXAMPLE123/5678abcdefghijklambdaisawesome/mnopqrstuvwxyzABCDEFGH"
},
"s3": {
"s3SchemaVersion": "1.0",
"configurationId": "testConfigRule",
"bucket": {
"name": `"amzn-s3-demo-bucket"`,
"ownerIdentity": {
"principalId": "EXAMPLE"
},
"arn": "arn:aws:s3:::amzn-s3-demo-bucket"
},
"object": {
"key": `"test%2Fkey"`,
"size": 1024,
"eTag": "0123456789abcdef0123456789abcdef",
"sequencer": "0A1B2C3D4E5F678901"
}
}
}
]
}`
```
* Choose **Save**.
* In the **Test event** pane, choose **Test**.
* To check the your function has created a resized verison of your image and stored it in your target Amazon S3 bucket, do the following:
1. Open the [Buckets page](https://console.aws.amazon.com/s3/buckets) of the Amazon S3 console.
2. Choose your target bucket and confirm that your resized file is listed in the **Objects** pane.
AWS CLI
###### To test your Lambda function with a dummy event (AWS CLI)
1. Save the following JSON in a file named `dummyS3Event.json`. Replace the values for the following parameters
with your own values:
* For `awsRegion`, replace `us-east-1` with the AWS Region you created your Amazon S3 buckets in.
* For `name`, replace `amzn-s3-demo-bucket` with the name of your own Amazon S3 source bucket.
* For `key`, replace `test%2Fkey` with the filename of the test object you uploaded to your source
bucket in the step [Upload a test image to your source bucket](#with-s3-tutorial-test-image).
```
`{
"Records": [
{
"eventVersion": "2.0",
"eventSource": "aws:s3",
"awsRegion": `"us-east-1"`,
"eventTime": "1970-01-01T00:00:00.000Z",
"eventName": "ObjectCreated:Put",
"userIdentity": {
"principalId": "EXAMPLE"
},
"requestParameters": {
"sourceIPAddress": "127.0.0.1"
},
"responseElements": {
"x-amz-request-id": "EXAMPLE123456789",
"x-amz-id-2": "EXAMPLE123/5678abcdefghijklambdaisawesome/mnopqrstuvwxyzABCDEFGH"
},
"s3": {
"s3SchemaVersion": "1.0",
"configurationId": "testConfigRule",
"bucket": {
"name": `"amzn-s3-demo-bucket"`,
"ownerIdentity": {
"principalId": "EXAMPLE"
},
"arn": "arn:aws:s3:::amzn-s3-demo-bucket"
},
"object": {
"key": `"test%2Fkey"`,
"size": 1024,
"eTag": "0123456789abcdef0123456789abcdef",
"sequencer": "0A1B2C3D4E5F678901"
}
}
}
]
}`
```
* From the directory you saved your `dummyS3Event.json` file in, invoke the function by running the following
CLI command. This command invokes your Lambda function synchronously by specifying `RequestResponse` as the value of the
invocation-type parameter. To learn more about synchronous and asynchronous invocation, see [Invoking Lambda functions](https://docs.aws.amazon.com/lambda/latest/dg/lambda-invocation.html).
```
``aws lambda invoke --function-name CreateThumbnail \\
--invocation-type RequestResponse --cli-binary-format raw-in-base64-out \\
--payload file://dummyS3Event.json outputfile.txt``
```
The cli-binary-format option is required if you are using version 2 of the AWS CLI. To make this the default setting, run
`aws configure set cli-binary-format raw-in-base64-out`. For more information, see [AWS CLI supported global command line options](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-options.html#cli-configure-options-list).
* Verify that your function has created a thumbnail version of your image and saved it to your target Amazon S3 bucket. Run the
following CLI command, replacing `amzn-s3-demo-source-bucket-resized` with the name of your own destination bucket.
```
``aws s3api list-objects-v2 --bucket `amzn-s3-demo-source-bucket-resized```
```
You should see output similar to the following. The `Key` parameter shows the filename of your resized image file.
```
`{
"Contents": [
{
"Key": "resized-HappyFace.jpg",
"LastModified": "2023-06-06T21:40:07+00:00",
"ETag": "\\"d8ca652ffe83ba6b721ffc20d9d7174a\\"",
"Size": 2633,
"StorageClass": "STANDARD"
}
]
}`
```
## Test your function using the Amazon S3 trigger
![Next step: Test the function](https://docs.aws.amazon.com/images/lambda/latest/dg/images/services-s3-tutorial/s3thumb_tut_steps9.png)
Now that you’ve confirmed your Lambda function is operating correctly, you’re ready to test your complete setup by adding an image file to
your Amazon S3 source bucket. When you add your image to the source bucket, your Lambda function should be automatically invoked. Your function
creates a resized version of the file and stores it in your target bucket.
AWS Management Console
###### To test your Lambda function using the Amazon S3 trigger (console)
1. To upload an image to your Amazon S3 bucket, do the following:
1. Open the [Buckets](https://console.aws.amazon.com/s3/buckets) page of the Amazon S3 console and choose your source bucket.
2. Choose **Upload**.
3. Choose **Add files** and use the file selector to choose the image file you want to upload. Your image
object can be any .jpg or .png file.
4. Choose **Open**, then choose **Upload**.
5. Verify that Lambda has saved a resized version of your image file in your target bucket by doing the following:
1. Navigate back to the [Buckets](https://console.aws.amazon.com/s3/buckets) page of the Amazon S3 console and choose your destination bucket.
2. In the **Objects** pane, you should now see two resized image files, one from each test of your Lambda function.
To download your resized image, select the file, then choose **Download**.
AWS CLI
###### To test your Lambda function using the Amazon S3 trigger (AWS CLI)
1. From the directory containing the image you want to upload, run the following CLI command. Replace the `--bucket`
parameter with the name of your source bucket. For the `--key` and `--body` parameters, use the filename of
your test image. Your test image can be any .jpg or .png file.
```
``aws s3api put-object --bucket `amzn-s3-demo-source-bucket` --key `SmileyFace.jpg` --body `./SmileyFace.jpg```
```
2. Verify that your function has created a thumbnail version of your image and saved it to your target Amazon S3 bucket. Run the
following CLI command, replacing `amzn-s3-demo-source-bucket-resized` with the name of your own destination bucket.
```
``aws s3api list-objects-v2 --bucket `amzn-s3-demo-source-bucket-resized```
```
If your function runs successfully, you’ll see output similar to the following. Your target bucket should now contain two resized files.
```
`{
"Contents": [
{
"Key": "resized-HappyFace.jpg",
"LastModified": "2023-06-07T00:15:50+00:00",
"ETag": "\\"7781a43e765a8301713f533d70968a1e\\"",
"Size": 2763,
"StorageClass": "STANDARD"
},
{
"Key": "resized-SmileyFace.jpg",
"LastModified": "2023-06-07T00:13:18+00:00",
"ETag": "\\"ca536e5a1b9e32b22cd549e18792cdbc\\"",
"Size": 1245,
"StorageClass": "STANDARD"
}
]
}`
```
## Clean up your resources
You can now delete the resources that you created for this tutorial, unless you want to retain them. By deleting AWS resources that you're no longer using, you prevent unnecessary charges to your AWS account.
###### To delete the Lambda function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Select the function that you created.
3. Choose **Actions**, **Delete**.
4. Type `confirm` in the text input field and choose **Delete**.
###### To delete the policy that you created
1. Open the [Policies page](https://console.aws.amazon.com/iam/home#/policies) of the IAM console.
2. Select the policy that you created (**AWSLambdaS3Policy**).
3. Choose **Policy actions**, **Delete**.
4. Choose **Delete**.
###### To delete the execution role
1. Open the [Roles page](https://console.aws.amazon.com/iam/home#/roles) of the IAM console.
2. Select the execution role that you created.
3. Choose **Delete**.
4. Enter the name of the role in the text input field and choose **Delete**.
###### To delete the S3 bucket
1. Open the [Amazon S3 console.](https://console.aws.amazon.com//s3/home#)
2. Select the bucket you created.
3. Choose **Delete**.
4. Enter the name of the bucket in the text input field.
5. Choose **Delete bucket**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Use an S3 trigger
Secrets Manager
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.