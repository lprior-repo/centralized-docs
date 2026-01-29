---
url: https://docs.aws.amazon.com/lambda/latest/dg/creating-deleting-layers.html
title: Creating and deleting layers in Lambda
word_count: 667
filtered: true
elements_removed: 0
density_score: 0.86
---

Creating and deleting layers in Lambda - AWS Lambda
Creating and deleting layers in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#creating-deleting-layers)
[Creating a layer](#layers-create)[Deleting a layer version](#layers-delete)
# Creating and deleting layers in Lambda
A Lambda layer is a .zip file archive that contains supplementary code or data.
Layers usually contain library dependencies, a [custom runtime](./runtimes-custom.html),
or configuration files.
This section explains how to create and delete layers in Lambda. For more conceptual
information about layers and why you might consider using them, see
[Managing Lambda dependencies with layers](./chapter-layers.html).
After you’ve [packaged your layer content](./packaging-layers.html), the
next step is to create the layer in Lambda. This section demonstrates how to create and
delete layers using the Lambda console or the Lambda API only. To create a layer using
AWS CloudFormation, see [Using AWS CloudFormation with layers](./layers-cfn.html). To create a layer using the
AWS Serverless Application Model (AWS SAM), see [Using AWS SAM with layers](./layers-sam.html).
###### Topics
* [Creating a layer](#layers-create)
* [Deleting a layer version](#layers-delete)
## Creating a layer
To create a layer, you can either upload the .zip file archive from your local
machine or from Amazon Simple Storage Service (Amazon S3). Lambda extracts the layer contents into the
`/opt` directory when setting up the execution environment for the function.
Layers can have one or more [layer versions](./chapter-layers.html#lambda-layer-versions).
When you create a layer, Lambda sets the layer version to version 1. You can change the
permissions on an existing layer version at any time. However, to update the code or make
other configuration changes, you must create a new version of the layer.
###### To create a layer (console)
1. Open the [Layers page](https://console.aws.amazon.com/lambda/home#/layers)
of the Lambda console.
2. Choose **Create layer**.
3. Under **Layer configuration**, for **Name**,
enter a name for your layer.
4. (Optional) For **Description**, enter a description for
your layer.
5. To upload your layer code, do one of the following:
* To upload a .zip file from your computer, choose **Upload a
.zip file**. Then, choose **Upload** to select
your local .zip file.
* To upload a file from Amazon S3, choose **Upload a file from
Amazon S3**. Then, for **Amazon S3 link URL**,
enter a link to the file.
* (Optional) For **Compatible architectures**, choose one
value or both values. For more information, see [Selecting and configuring an instruction set architecture for your Lambda function](./foundation-arch.html).
* (Optional) For **Compatible runtimes**, choose the
runtimes that your layer is compatible with.
* (Optional) For **License**, enter any necessary license information.
* Choose **Create**.
Alternatively, you can run the [publish-layer-version](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/publish-layer-version.html) AWS Command Line Interface (CLI) command. Example:
```
`aws lambda publish-layer-version --layer-name `my-layer` --zip-file fileb://layer.zip --compatible-runtimes `python3.14``
```
Each time that you run `publish-layer-version`, Lambda creates a new [version of the layer](./chapter-layers.html#lambda-layer-versions).
## Deleting a layer version
To delete a layer version, use the [DeleteLayerVersion](https://docs.aws.amazon.com/lambda/latest/api/API_DeleteLayerVersion.html) API operation. For example,
run the [delete-layer-version](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/delete-layer-version.html) AWS CLI command with the layer name and layer version specified.
```
`aws lambda delete-layer-version --layer-name my-layer --version-number 1`
```
When you delete a layer version, you can no longer configure a Lambda function
to use it. However, any function that already uses the version continues to have
access to it. Also, Lambda never reuses version numbers for a layer name.
When calculating [quotas](./gettingstarted-limits.html), deleting
a layer version means it's no longer counted as part of the default 75 GB quota for
storage of functions and layers. However, for functions that consume a deleted layer
version, the layer content still counts towards the function's deployment package size
quota (i.e. 250MB for .zip file archives).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Packaging layers
Adding layers
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.