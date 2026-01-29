---
url: https://docs.aws.amazon.com/lambda/latest/dg/nodejs-package.html
title: Deploy Node.js Lambda functions with .zip file archives
word_count: 3367
filtered: true
elements_removed: 0
density_score: 0.86
---

Deploy Node.js Lambda functions with .zip file archives - AWS Lambda
Deploy Node.js Lambda functions with .zip file archives - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#nodejs-package)
[Runtime dependencies in Node.js](#nodejs-package-dependencies)[Creating a .zip deployment package with no dependencies](#nodejs-package-create-no-dependencies)[Creating a .zip deployment package with dependencies](#nodejs-package-create-dependencies)[Creating a Node.js layer for your dependencies](#nodejs-package-dependencies-layers)[Dependency search path and runtime-included libraries](#nodejs-package-searchpath)[Creating and updating Node.js Lambda functions using .zip files](#nodejs-package-create-update)
# Deploy Node.js Lambda functions with .zip file archives
Your AWS Lambda function’s code comprises a .js or .mjs file containing your function’s handler code, together with any additional packages and modules
your code depends on. To deploy this function code to Lambda, you use a *deployment package*. This package may either be a
.zip file archive or a container image.
For more information about using container images with Node.js, see [Deploy Node.js Lambda functions with container images](https://docs.aws.amazon.com/lambda/latest/dg/nodejs-image.html).
To create your deployment package as .zip file archive, you can use your command-line tool’s built-in .zip file archive utility, or any other
.zip file utility such as [7zip](https://www.7-zip.org/download.html). The examples shown in the following sections assume
you’re using a command-line `zip` tool in a Linux or MacOS environment. To use the same commands in Windows, you can
[install the Windows Subsystem for Linux](https://docs.microsoft.com/en-us/windows/wsl/install-win10) to get a Windows-integrated
version of Ubuntu and Bash.
Note that Lambda uses POSIX file permissions, so you may need to [set permissions for the deployment package folder](http://aws.amazon.com/premiumsupport/knowledge-center/lambda-deployment-package-errors/) before you create the .zip file archive.
###### Topics
* [Runtime dependencies in Node.js](#nodejs-package-dependencies)
* [Creating a .zip deployment package with no dependencies](#nodejs-package-create-no-dependencies)
* [Creating a .zip deployment package with dependencies](#nodejs-package-create-dependencies)
* [Creating a Node.js layer for your dependencies](#nodejs-package-dependencies-layers)
* [Dependency search path and runtime-included libraries](#nodejs-package-searchpath)
* [Creating and updating Node.js Lambda functions using .zip files](#nodejs-package-create-update)
## Runtime dependencies in Node.js
For Lambda functions that use the Node.js runtime, a dependency can be any Node.js module. The Node.js runtime includes a number of common
libraries, as well as a version of the AWS SDK for JavaScript. All [supported Node.js runtimes](./lambda-nodejs.html#nodejs-supported-runtimes) include version 3 of the SDK. To use version 2 of the SDK, you must add the SDK to your .zip file deployment package. To find the specific version of the SDK that's included in the runtime that you're using, see [Runtime-included SDK versions](./lambda-nodejs.html#nodejs-sdk-included).
Lambda periodically updates the SDK libraries in the Node.js runtime to include the latest features and security upgrades. Lambda also applies
security patches and updates to the other libraries included in the runtime. To have full control of the dependencies in your package, you can
add your preferred version of any runtime-included dependency to your deployment package. For example, if you want to use a particular version of
the SDK for JavaScript, you can include it in your .zip file as a dependency. For more information on adding runtime-included dependencies to your .zip
file, see [Dependency search path and runtime-included libraries](#nodejs-package-searchpath).
Under the [AWS shared responsibility model](./lambda-runtimes.html#runtimes-shared-responsibility),
you are responsible for the management of any dependencies in your functions' deployment packages. This includes applying updates and security
patches. To update dependencies in your function's deployment package, first create a new .zip file and then upload it to Lambda. See [Creating a .zip deployment package with dependencies](#nodejs-package-create-dependencies)
and [Creating and updating Node.js Lambda functions using .zip files](#nodejs-package-create-update) for more information.
## Creating a .zip deployment package with no dependencies
If your function code has no dependencies except for libraries included in the Lambda runtime, your .zip file contains only the `index.js` or
`index.mjs` file with your function’s handler code. Use your preferred zip utility to create a .zip file with your `index.js` or
`index.mjs` file at the root. If the file containing your handler code isn't at the root of your .zip file, Lambda won’t be able to run your code.
To learn how to deploy your .zip file to create a new Lambda function or update an existing one, see [Creating and updating Node.js Lambda functions using .zip files](#nodejs-package-create-update).
## Creating a .zip deployment package with dependencies
If your function code depends on packages or modules that aren't included in the Lambda Node.js runtime,
you can either add these dependencies to your .zip file with your function code or use a
[Lambda layer](./chapter-layers.html). The instructions in this section show you how to
include your dependencies in your .zip deployment package. For instructions on how to include your
dependencies in a layer, see [Creating a Node.js layer for your dependencies](#nodejs-package-dependencies-layers).
The following example CLI commands create a .zip file named `my\_deployment\_package.zip` containing the `index.js`
or `index.mjs` file with your function's handler code and its dependencies. In the example, you install dependencies using the
npm package manager.
###### To create the deployment package
1. Navigate to the project directory containing your `index.js` or `index.mjs` source code file. In this example,
the directory is named `my\_function`.
```
``cd my\_function``
```
2. Install your function's required libraries in the `node\_modules` directory using the `npm install` command. In this
example you install the AWS X-Ray SDK for Node.js.
```
``npm install aws-xray-sdk``
```
This creates a folder structure similar to the following:
```
`\~/my\_function
├── index.mjs
└── node\_modules
├── async
├── async-listener
├── atomic-batcher
├── aws-sdk
├── aws-xray-sdk
├── aws-xray-sdk-core`
```
You can also add custom modules that you create yourself to your deployment package. Create a directory under `node\_modules`
with the name of your module and save your custom written packages there.
3. Create a .zip file that contains the contents of your project folder at the root. Use the `r` (recursive) option to
ensure that zip compresses the subfolders.
```
``zip -r my\_deployment\_package.zip .``
```
## Creating a Node.js layer for your dependencies
The instructions in this section show you how to include your dependencies in a layer. For instructions
on how to include your dependencies in your deployment package, see
[Creating a .zip deployment package with dependencies](#nodejs-package-create-dependencies).
When you add a layer to a function, Lambda loads the layer content into the
`/opt` directory of that execution environment. For each Lambda runtime,
the `PATH` variable already includes specific folder paths within the
`/opt` directory. To ensure that Lambda picks up your layer content,
your layer .zip file should have its dependencies
in one of the following folder paths:
* `nodejs/node\_modules`
* `nodejs/node18/node\_modules (NODE\_PATH)`
* `nodejs/node20/node\_modules (NODE\_PATH)`
* `nodejs/node22/node\_modules (NODE\_PATH)`
For example, your layer .zip file structure might look like the following:
```
`xray-sdk.zip
└ nodejs/node\_modules/aws-xray-sdk`
```
In addition, Lambda automatically detects any libraries in the `/opt/lib`
directory, and any binaries in the `/opt/bin` directory. To ensure
that Lambda properly finds your layer content, you can also create a layer
with the following structure:
```
`custom-layer.zip
└ lib
| lib\_1
| lib\_2
└ bin
| bin\_1
| bin\_2`
```
After you package your layer, see [Creating and deleting layers in Lambda](./creating-deleting-layers.html)
and [Adding layers to functions](./adding-layers.html) to complete your layer setup.
## Dependency search path and runtime-included libraries
The Node.js runtime includes a number of common libraries, as well as a version of the AWS SDK for JavaScript. If you want to use a different version of a runtime-included library, you can do this by bundling it with your function or by adding it as a
dependency in your deployment package. For example, you can use a different version of the SDK by adding it to your .zip deployment package.
You can also include it in a [Lambda layer](./chapter-layers.html) for your function.
When you use an `import` or `require` statement in your code, the Node.js runtime searches the directories in the `NODE\_PATH`
path until it finds the module. By default, the first location the runtime searches is the directory into which your .zip deployment package is
decompressed and mounted (`/var/task`). If you include a version of a runtime-included library in your deployment package, this
version will take precedence over the version included in the runtime. Dependencies in your deployment package also have precedence over
dependencies in layers.
When you add a dependency to a layer, Lambda extracts this to `/opt/nodejs/nodexx/node\_modules` where `nodexx`
represents the version of the runtime you are using. In the search path, this directory has precedence over the directory containing the
runtime-included libraries (`/var/lang/lib/node\_modules`). Libraries in function layers therefore have precedence over versions
included in the runtime.
You can see the full search path for your Lambda function by adding the following line of code.
```
`console.log(process.env.NODE\_PATH)`
```
You can also add dependencies in a separate folder inside your .zip package. For example, you might add a custom module to a folder in
your .zip package called `common`. When your .zip package is decompressed and mounted, this folder is placed inside the `/var/task` directory.
To use a dependency from a folder in your .zip deployment package in your code, use an `import { } from` or `const { } = require()` statement,
depending on whether you are using CJS or ESM module resolution. For example:
```
`import { myModule } from './common'`
```
If you bundle your code with `esbuild`, `rollup`, or similar, the dependencies used by your function are bundled together in one
or more files. We recommend using this method to vend dependencies whenever possible. Compared to adding dependencies to your deployment package,
bundling your code results in improved performance due to the reduction in I/O operations.
## Creating and updating Node.js Lambda functions using .zip files
After you have created your .zip deployment package, you can use it to create a new Lambda function or update an existing one. You can deploy
your .zip package using the Lambda console, the AWS Command Line Interface, and the Lambda API. You can also create and update Lambda functions using
AWS Serverless Application Model (AWS SAM) and CloudFormation.
The maximum size for a .zip deployment package for Lambda is 250 MB (unzipped). Note that this limit applies to the combined size of all the files you upload,
including any Lambda layers.
The Lambda runtime needs permission to read the files in your deployment package. In Linux permissions octal notation, Lambda needs 644 permissions
for non-executable files (rw-r--r--) and 755 permissions (rwxr-xr-x) for directories and executable files.
In Linux and MacOS, use the `chmod` command to change file permissions on files and directories in your deployment package. For
example, to give a non-executable file the correct permissions, run the following command.
```
``chmod 644 &lt;filepath&gt;``
```
To change file permissions in Windows, see [Set, View, Change, or Remove Permissions on an Object](https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2008-R2-and-2008/cc731667(v=ws.10))
in the Microsoft Windows documentation.
###### Note
If you don't grant Lambda the permissions it needs to access directories in your deployment package, Lambda sets the permissions for those directories to 755 (rwxr-xr-x).
### Creating and updating functions with .zip files using the console
To create a new function, you must first create the function in the console, then upload your .zip archive. To update an existing function, open the page for your function, then follow the same procedure to add your updated .zip file.
If your .zip file is less than 50MB, you can create or update a function by uploading the file directly from your local machine. For .zip files greater than 50MB, you must upload your package to an Amazon S3 bucket first. For instructions on how to upload a file to an Amazon S3 bucket using the AWS Management Console, see [Getting started with Amazon S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/GetStartedWithS3.html). To upload files using the AWS CLI, see [Move objects](https://docs.aws.amazon.com/cli/latest/userguide/cli-services-s3-commands.html#using-s3-commands-managing-objects-move) in the *AWS CLI User Guide*.
###### Note
You cannot change the [deployment package type](https://docs.aws.amazon.com/lambda/latest/api/API_CreateFunction.html#lambda-CreateFunction-request-PackageType) (.zip or container image) for an existing function. For example, you cannot convert a container image function to use a .zip file archive. You must create a new function.
###### To create a new function (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console and choose **Create Function**.
2. Choose **Author from scratch**.
3. Under **Basic information**, do the following:
1. For **Function name**, enter the name for your function.
2. For **Runtime**, select the runtime you want to use.
3. (Optional) For **Architecture**, choose the instruction set architecture for your function. The default architecture is x86\_64. Ensure that the .zip deployment package for your function is compatible with the instruction set architecture you select.
4. (Optional) Under **Permissions**, expand **Change default execution role**. You can create a new **Execution role** or use an existing one.
5. Choose **Create function**. Lambda creates a basic 'Hello world' function using your chosen runtime.
###### To upload a .zip archive from your local machine (console)
1. In the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console, choose the function you want to upload the .zip file for.
2. Select the **Code** tab.
3. In the **Code source** pane, choose **Upload from**.
4. Choose **.zip file**.
5. To upload the .zip file, do the following:
1. Select **Upload**, then select your .zip file in the file chooser.
2. Choose **Open**.
3. Choose **Save**.
###### To upload a .zip archive from an Amazon S3 bucket (console)
1. In the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console, choose the function you want to upload a new .zip file for.
2. Select the **Code** tab.
3. In the **Code source** pane, choose **Upload from**.
4. Choose **Amazon S3 location**.
5. Paste the Amazon S3 link URL of your .zip file and choose **Save**.
### Updating .zip file functions using the console code editor
For some functions with .zip deployment packages, you can use the Lambda console’s built-in code editor to update your function code directly. To use this feature, your function must meet the following criteria:
* Your function must use one of the interpreted language runtimes (Python, Node.js, or Ruby)
* Your function’s deployment package must be smaller than 50 MB (unzipped).
Function code for functions with container image deployment packages cannot be edited directly in the console.
###### To update function code using the console code editor
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console and select your function.
2. Select the **Code** tab.
3. In the **Code source** pane, select your source code file and edit it in the integrated code editor.
4. In the **DEPLOY** section, choose **Deploy** to update your function's code:
![Deploy button in the Lambda console code editor](https://docs.aws.amazon.com/images/lambda/latest/dg/images/getting-started-tutorial/deploy-console.png)
### Creating and updating functions with .zip files using the AWS CLI
You can can use the [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) to create a new function or to update an existing one using a .zip file. Use the [create-function](https://docs.aws.amazon.com/cli/latest/reference/lambda/create-function.html)
and [update-function-code](https://docs.aws.amazon.com/cli/latest/reference/lambda/create-function.html) commands to deploy your .zip
package. If your .zip file is smaller than 50MB, you can upload the .zip package from a file location on your local build machine. For larger files,
you must upload your .zip package from an Amazon S3 bucket. For instructions on how to upload a file to an Amazon S3 bucket using the AWS CLI, see [Move objects](https://docs.aws.amazon.com/cli/latest/userguide/cli-services-s3-commands.html#using-s3-commands-managing-objects-move) in the *AWS CLI User Guide*.
###### Note
If you upload your .zip file from an Amazon S3 bucket using the AWS CLI, the bucket must be located in the same AWS Region as your function.
To create a new function using a .zip file with the AWS CLI, you must specify the following:
* The name of your function (`--function-name`)
* Your function’s runtime (`--runtime`)
* The Amazon Resource Name (ARN) of your function’s [execution role](https://docs.aws.amazon.com/lambda/latest/dg/lambda-intro-execution-role.html) (`--role`)
* The name of the handler method in your function code (`--handler`)
You must also specify the location of your .zip file. If your .zip file is located in a folder on your local build machine, use the `--zip-file` option to specify the file path, as shown in the following example command.
```
``aws lambda create-function --function-name myFunction \\
--runtime nodejs24.x --handler index.handler \\
--role arn:aws:iam::111122223333:role/service-role/my-lambda-role \\
--zip-file fileb://myFunction.zip``
```
To specify the location of .zip file in an Amazon S3 bucket, use the `--code` option as shown in the following example command. You only
need to use the `S3ObjectVersion` parameter for versioned objects.
```
``aws lambda create-function --function-name myFunction \\
--runtime nodejs24.x --handler index.handler \\
--role arn:aws:iam::111122223333:role/service-role/my-lambda-role \\
--code S3Bucket=amzn-s3-demo-bucket,S3Key=myFileName.zip,S3ObjectVersion=myObjectVersion``
```
To update an existing function using the CLI, you specify the the name of your function using the `--function-name` parameter. You
must also specify the location of the .zip file you want to use to update your function code. If your .zip file is located in a folder on your
local build machine, use the `--zip-file` option to specify the file path, as shown in the following example command.
```
``aws lambda update-function-code --function-name myFunction \\
--zip-file fileb://myFunction.zip``
```
To specify the location of .zip file in an Amazon S3 bucket, use the `--s3-bucket` and `--s3-key` options as shown in the
following example command. You only need to use the `--s3-object-version` parameter for versioned objects.
```
``aws lambda update-function-code --function-name myFunction \\
--s3-bucket amzn-s3-demo-bucket --s3-key myFileName.zip --s3-object-version myObject Version``
```
### Creating and updating functions with .zip files using the Lambda API
To create and update functions using a .zip file archive, use the following API operations:
* [CreateFunction](https://docs.aws.amazon.com/lambda/latest/api/API_CreateFunction.html)
* [UpdateFunctionCode](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionCode.html)
### Creating and updating functions with .zip files using AWS SAM
The AWS Serverless Application Model (AWS SAM) is a toolkit that helps streamline the process of building and running serverless applications on AWS. You define the resources for your application in a
YAML or JSON template and use the AWS SAM command line interface (AWS SAM CLI) to build, package, and deploy your applications. When you build a Lambda function from an AWS SAM template,
AWS SAM automatically creates a .zip deployment package or container image with your function code and any dependencies you specify. To learn more about using AWS SAM to build and deploy
Lambda functions, see [Getting started with AWS SAM](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-getting-started.html) in the
*AWS Serverless Application Model Developer Guide*.
You can also use AWS SAM to create a Lambda function using an existing .zip file archive. To create a Lambda function using AWS SAM, you can save your .zip file in
an Amazon S3 bucket or in a local folder on your build machine. For instructions on how to upload a file to an Amazon S3 bucket using the AWS CLI,
see [Move objects](https://docs.aws.amazon.com/cli/latest/userguide/cli-services-s3-commands.html#using-s3-commands-managing-objects-move) in the *AWS CLI User Guide*.
In your AWS SAM template, the `AWS::Serverless::Function` resource specifies your Lambda function. In this resource, set the following
properties to create a function using a .zip file archive:
* `PackageType` - set to `Zip`
* `CodeUri` - set to the function code's Amazon S3 URI, path to local folder, or [FunctionCode](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-property-function-functioncode.html) object
* `Runtime` - Set to your chosen runtime
With AWS SAM, if your .zip file is larger than 50MB, you don’t need to upload it to an Amazon S3 bucket first. AWS SAM can upload .zip packages up to
the maximum allowed size of 250MB (unzipped) from a location on your local build machine.
To learn more about deploying functions using .zip file in AWS SAM, see [AWS::Serverless::Function](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-function.html) in the *AWS SAM Developer Guide*.
### Creating and updating functions with .zip files using CloudFormation
You can use CloudFormation to create a Lambda function using a .zip file archive. To create a Lambda function from a .zip file, you must first upload
your file to an Amazon S3 bucket. For instructions on how to upload a file to an Amazon S3 bucket using the AWS CLI, see [Move objects](https://docs.aws.amazon.com/cli/latest/userguide/cli-services-s3-commands.html#using-s3-commands-managing-objects-move)
in the *AWS CLI User Guide. *
In your CloudFormation template, the `AWS::Lambda::Function` resource specifies your Lambda function. In this resource, set the following
properties to create a function using a .zip file archive:
* `PackageType` - Set to `Zip`
* `Code` - Enter the Amazon S3 bucket name and the .zip file name in the `S3Bucket` and `S3Key` fields
* `Runtime` - Set to your chosen runtime
The .zip file that CloudFormation generates cannot exceed 4MB. To learn more about deploying functions using .zip file in CloudFormation, see [AWS::Lambda::Function](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html)
in the *CloudFormation User Guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Handler
Deploy container images
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.