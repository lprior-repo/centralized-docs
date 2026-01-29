---
url: https://docs.aws.amazon.com/lambda/latest/dg/java-package.html
title: Deploy Java Lambda functions with .zip or JAR file archives
word_count: 2441
filtered: true
elements_removed: 0
density_score: 0.86
---

Deploy Java Lambda functions with .zip or JAR file archives - AWS Lambda
Deploy Java Lambda functions with .zip or JAR file archives - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#java-package)
[Prerequisites](#java-package-prereqs)[Tools and libraries](#java-package-libraries)[Building a deployment package with Gradle](#java-package-gradle)[Using layers for dependencies](#java-package-layers)[Building a deployment package with Maven](#java-package-maven)[Uploading a deployment package with the Lambda console](#java-package-console)[Uploading a deployment package with the AWS CLI](#java-package-cli)[Uploading a deployment package with AWS SAM](#java-package-cloudformation)
# Deploy Java Lambda functions with .zip or JAR file archives
Your AWS Lambda function's code consists of scripts or compiled programs and their dependencies.
You use a *deployment package* to deploy your function code to Lambda. Lambda supports two types of deployment packages:
container images and .zip file archives.
This page describes how to create your deployment package as a .zip file or Jar file, and then use the
deployment package to deploy your function code to AWS Lambda using the AWS Command Line Interface (AWS CLI).
###### Important
Java 25 introduced support for Ahead-of-Time (AOT) caches. We strongly recommend not using AOT caches when deploying your functions as .zip or JAR file archives, since the caches may cause unexpected behavior when Lambda updates the managed runtime. For further information, see [Ahead-of-Time (AOT) and CDS caches](./java-customization.html#aot-cds-caches).
###### Sections
* [Prerequisites](#java-package-prereqs)
* [Tools and libraries](#java-package-libraries)
* [Building a deployment package with Gradle](#java-package-gradle)
* [Using layers for dependencies](#java-package-layers)
* [Building a deployment package with Maven](#java-package-maven)
* [Uploading a deployment package with the Lambda console](#java-package-console)
* [Uploading a deployment package with the AWS CLI](#java-package-cli)
* [Uploading a deployment package with AWS SAM](#java-package-cloudformation)
## Prerequisites
The AWS CLI is an open-source tool that enables you to interact with AWS services using commands in your command line shell. To complete the steps in this section, you must have the [AWS CLI version 2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html).
## Tools and libraries
AWS provides the following libraries for Java functions. These libraries are available through [Maven
Central Repository](https://search.maven.org/search?q=g:com.amazonaws).
* [com.amazonaws:aws-lambda-java-core](https://github.com/aws/aws-lambda-java-libs/tree/master/aws-lambda-java-core)
(required) – Defines handler method interfaces and the context object that the runtime passes to the
handler. If you define your own input types, this is the only library that you need.
* [com.amazonaws:aws-lambda-java-events](https://github.com/aws/aws-lambda-java-libs/tree/master/aws-lambda-java-events) – Input types for events from services that invoke Lambda
functions.
* [com.amazonaws:aws-lambda-java-log4j2](https://github.com/aws/aws-lambda-java-libs/tree/master/aws-lambda-java-log4j2) – An appender library for Apache Log4j 2 that you can use
to add the request ID for the current invocation to your [function
logs](./java-logging.html).
* [AWS SDK for Java 2.0](https://github.com/aws/aws-sdk-java-v2) – The official AWS SDK for the Java programming language.
Add these libraries to your build definition as follows:
Gradle
```
`dependencies {
`implementation 'com.amazonaws:aws-lambda-java-core:1.2.2'
implementation 'com.amazonaws:aws-lambda-java-events:3.11.1'
runtimeOnly 'com.amazonaws:aws-lambda-java-log4j2:1.5.1'`
}`
```
Maven
```
` &lt;dependencies&gt;
&lt;dependency&gt;
&lt;groupId&gt;com.amazonaws&lt;/groupId&gt;
&lt;artifactId&gt;aws-lambda-java-core&lt;/artifactId&gt;
&lt;version&gt;1.2.2&lt;/version&gt;
&lt;/dependency&gt;
&lt;dependency&gt;
&lt;groupId&gt;com.amazonaws&lt;/groupId&gt;
&lt;artifactId&gt;aws-lambda-java-events&lt;/artifactId&gt;
&lt;version&gt;3.11.1&lt;/version&gt;
&lt;/dependency&gt;
&lt;dependency&gt;
&lt;groupId&gt;com.amazonaws&lt;/groupId&gt;
&lt;artifactId&gt;aws-lambda-java-log4j2&lt;/artifactId&gt;
&lt;version&gt;1.5.1&lt;/version&gt;
&lt;/dependency&gt;
&lt;/dependencies&gt;`
```
To create a deployment package, compile your function code and dependencies into a single .zip file or Java
Archive (JAR) file. For Gradle, [use the Zip build type](#java-package-gradle). For
Apache Maven, [use the Maven Shade plugin](#java-package-maven). To upload your deployment
package, use the Lambda console, the Lambda API, or AWS Serverless Application Model (AWS SAM).
###### Note
To keep your deployment package size small, package your function's dependencies in layers.
Layers enable you to manage your dependencies independently, can be used by multiple functions, and can be shared with other accounts.
For more information, see [Managing Lambda dependencies with layers](./chapter-layers.html).
## Building a deployment package with Gradle
To create a deployment package with your function's code and dependencies in Gradle, use the `Zip`
build type. Here's an example from a
[complete
sample build.gradle file](https://github.com/awsdocs/aws-lambda-developer-guide/blob/main/sample-apps/s3-java/build.gradle):
###### Example build.gradle – Build task
```
`task buildZip(type: Zip) {
into('lib') {
from(jar)
from(configurations.runtimeClasspath)
}
}`
```
This build configuration produces a deployment package in the `build/distributions` directory.
Within the `into('lib')` statement, the `jar` task assembles a jar archive containing
your main classes into a folder named `lib`. Additionally, the `configurations.runtimeClassPath`
task copies dependency libraries from the build's classpath into the same `lib` folder.
###### Example build.gradle – Dependencies
```
`dependencies {
...
`implementation 'com.amazonaws:aws-lambda-java-core:1.2.2'
implementation 'com.amazonaws:aws-lambda-java-events:3.11.1'`
implementation 'org.apache.logging.log4j:log4j-api:2.17.1'
implementation 'org.apache.logging.log4j:log4j-core:2.17.1'
runtimeOnly 'org.apache.logging.log4j:log4j-slf4j18-impl:2.17.1'
`runtimeOnly 'com.amazonaws:aws-lambda-java-log4j2:1.5.1'`
...
}`
```
Lambda loads JAR files in Unicode alphabetical order. If multiple JAR files in the `lib` directory
contain the same class, the first one is used. You can use the following shell script to identify duplicate
classes:
###### Example test-zip.sh
```
`mkdir -p expanded
unzip path/to/my/function.zip -d expanded
find ./expanded/lib -name '\*.jar' | xargs -n1 zipinfo -1 | grep '.\*.class' | sort | uniq -c | sort`
```
## Using layers for dependencies
You can package your function's dependencies in layers to keep your deployment package small and manage dependencies independently. For more information, see [Working with layers for Java Lambda functions](./java-layers.html).
## Building a deployment package with Maven
To build a deployment package with Maven, use the [Maven Shade plugin](https://maven.apache.org/plugins/maven-shade-plugin/). The plugin creates a JAR
file that contains the compiled function code and all of its dependencies.
###### Example pom.xml – Plugin configuration
```
` &lt;plugin&gt;
&lt;groupId&gt;org.apache.maven.plugins&lt;/groupId&gt;
&lt;artifactId&gt;maven-shade-plugin&lt;/artifactId&gt;
&lt;version&gt;3.2.2&lt;/version&gt;
&lt;configuration&gt;
&lt;createDependencyReducedPom&gt;false&lt;/createDependencyReducedPom&gt;
&lt;/configuration&gt;
&lt;executions&gt;
&lt;execution&gt;
&lt;phase&gt;package&lt;/phase&gt;
&lt;goals&gt;
&lt;goal&gt;shade&lt;/goal&gt;
&lt;/goals&gt;
&lt;/execution&gt;
&lt;/executions&gt;
&lt;/plugin&gt;`
```
To build the deployment package, use the `mvn package` command.
```
`[INFO] Scanning for projects...
[INFO] -----------------------&lt; com.example:java-maven &gt;-----------------------
[INFO] Building java-maven-function 1.0-SNAPSHOT
[INFO] --------------------------------[ jar ]---------------------------------
...
[INFO] --- maven-jar-plugin:2.4:jar (default-jar) @ java-maven ---
[INFO] Building jar: target/java-maven-1.0-SNAPSHOT.jar
[INFO]
[INFO] --- maven-shade-plugin:3.2.2:shade (default) @ java-maven ---
[INFO] Including com.amazonaws:aws-lambda-java-core:jar:1.2.2 in the shaded jar.
[INFO] Including com.amazonaws:aws-lambda-java-events:jar:3.11.1 in the shaded jar.
[INFO] Including joda-time:joda-time:jar:2.6 in the shaded jar.
[INFO] Including com.google.code.gson:gson:jar:2.8.6 in the shaded jar.
[INFO] Replacing original artifact with shaded artifact.
[INFO] Replacing target/java-maven-1.0-SNAPSHOT.jar with target/java-maven-1.0-SNAPSHOT-shaded.jar
[INFO] ------------------------------------------------------------------------
[INFO] BUILD SUCCESS
[INFO] ------------------------------------------------------------------------
[INFO] Total time: 8.321 s
[INFO] Finished at: 2020-03-03T09:07:19Z
[INFO] ------------------------------------------------------------------------`
```
This command generates a JAR file in the `target` directory.
###### Note
If you're working with a [multi-release JAR (MRJAR)](https://openjdk.org/jeps/238),
you must include the MRJAR (i.e. the shaded JAR produced by the Maven Shade plugin) in the `lib`
directory and zip it before uploading your deployment package to Lambda. Otherwise, Lambda may not properly
unpack your JAR file, causing your `MANIFEST.MF` file to be ignored.
If you use the appender library (`aws-lambda-java-log4j2`), you must also configure a transformer
for the Maven Shade plugin. The transformer library combines versions of a cache file that appear in both the
appender library and in Log4j.
###### Example pom.xml – Plugin configuration with Log4j 2 appender
```
` &lt;&lt;plugin&gt;&gt;
&lt;&lt;groupId&gt;&gt;org.apache.maven.plugins&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;maven-shade-plugin&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;3.2.2&lt;&lt;/version&gt;&gt;
&lt;&lt;configuration&gt;&gt;
&lt;&lt;createDependencyReducedPom&gt;&gt;false&lt;&lt;/createDependencyReducedPom&gt;&gt;
&lt;&lt;/configuration&gt;&gt;
&lt;&lt;executions&gt;&gt;
&lt;&lt;execution&gt;&gt;
&lt;&lt;phase&gt;&gt;package&lt;&lt;/phase&gt;&gt;
&lt;&lt;goals&gt;&gt;
&lt;&lt;goal&gt;&gt;shade&lt;&lt;/goal&gt;&gt;
&lt;&lt;/goals&gt;&gt;
&lt;&lt;configuration&gt;&gt;
&lt;&lt;transformers&gt;&gt;
&lt;&lt;transformer implementation="com.github.edwgiz.maven\_shade\_plugin.log4j2\_cache\_transformer.PluginsCacheFileTransformer"&gt;&gt;
&lt;&lt;/transformer&gt;&gt;
&lt;&lt;/transformers&gt;&gt;
&lt;&lt;/configuration&gt;&gt;
&lt;&lt;/execution&gt;&gt;
&lt;&lt;/executions&gt;&gt;
&lt;&lt;dependencies&gt;&gt;
&lt;&lt;dependency&gt;&gt;
&lt;&lt;groupId&gt;&gt;com.github.edwgiz&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;maven-shade-plugin.log4j2-cachefile-transformer&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;2.13.0&lt;&lt;/version&gt;&gt;
&lt;&lt;/dependency&gt;&gt;
&lt;&lt;/dependencies&gt;&gt;
&lt;&lt;/plugin&gt;&gt;`
```
## Uploading a deployment package with the Lambda console
To create a new function, you must first create the function in the console, then upload your .zip or JAR file. To update an existing
function, open the page for your function, then follow the same procedure to add your updated .zip or JAR file.
If your deployment package file is less than 50MB, you can create or update a function by uploading the file directly from your local machine.
For .zip or JAR files greater than 50MB, you must upload your package to an Amazon S3 bucket first. For instructions on how to upload a file to an Amazon S3
bucket using the AWS Management Console, see [Getting started with Amazon S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/GetStartedWithS3.html).
To upload files using the AWS CLI, see [Move objects](https://docs.aws.amazon.com/cli/latest/userguide/cli-services-s3-commands.html#using-s3-commands-managing-objects-move)
in the *AWS CLI User Guide*.
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
###### To upload a .zip or JAR archive from your local machine (console)
1. In the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console, choose the function you want to
upload the .zip or JAR file for.
2. Select the **Code** tab.
3. In the **Code source** pane, choose **Upload from**.
4. Choose **.zip or .jar file**.
5. To upload the .zip or JAR file, do the following:
1. Select **Upload**, then select your .zip or JAR file in the file chooser.
2. Choose **Open**.
3. Choose **Save**.
###### To upload a .zip or JAR archive from an Amazon S3 bucket (console)
1. In the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console, choose the function you want to
upload a new .zip or JAR file for.
2. Select the **Code** tab.
3. In the **Code source** pane, choose **Upload from**.
4. Choose **Amazon S3 location**.
5. Paste the Amazon S3 link URL of your .zip file and choose **Save**.
## Uploading a deployment package with the AWS CLI
You can can use the [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) to create a new function or
to update an existing one using a .zip or JAR file. Use the [create-function](https://docs.aws.amazon.com/cli/latest/reference/lambda/create-function.html)
and [update-function-code](https://docs.aws.amazon.com/cli/latest/reference/lambda/create-function.html) commands to deploy your .zip or JAR
package. If your file is smaller than 50MB, you can upload the package from a file location on your local build machine. For larger files,
you must upload your .zip or JAR package from an Amazon S3 bucket. For instructions on how to upload a file to an Amazon S3 bucket using the AWS CLI,
see [Move objects](https://docs.aws.amazon.com/cli/latest/userguide/cli-services-s3-commands.html#using-s3-commands-managing-objects-move) in the *AWS CLI User Guide*.
###### Note
If you upload your .zip or JAR file from an Amazon S3 bucket using the AWS CLI, the bucket must be located in the same AWS Region as your function.
To create a new function using a .zip or JAR file with the AWS CLI, you must specify the following:
* The name of your function (`--function-name`)
* Your function’s runtime (`--runtime`)
* The Amazon Resource Name (ARN) of your function’s [execution role](https://docs.aws.amazon.com/lambda/latest/dg/lambda-intro-execution-role.html) (`--role`)
* The name of the handler method in your function code (`--handler`)
You must also specify the location of your .zip or JAR file. If your .zip or JAR file is located in a folder on your local build machine, use
the `--zip-file` option to specify the file path, as shown in the following example command.
```
``aws lambda create-function --function-name myFunction \\
--runtime java25 --handler example.handler \\
--role arn:aws:iam::123456789012:role/service-role/my-lambda-role \\
--zip-file fileb://myFunction.zip``
```
To specify the location of .zip file in an Amazon S3 bucket, use the `--code` option as shown in the following example command. You only
need to use the `S3ObjectVersion` parameter for versioned objects.
```
``aws lambda create-function --function-name myFunction \\
--runtime java25 --handler example.handler \\
--role arn:aws:iam::123456789012:role/service-role/my-lambda-role \\
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
## Uploading a deployment package with AWS SAM
You can use AWS SAM to automate deployments of your function code, configuration, and dependencies. AWS SAM is an
extension of CloudFormation that provides a simplified syntax for defining serverless applications. The following example
template defines a function with a deployment package in the `build/distributions` directory that
Gradle uses:
###### Example template.yml
```
`AWSTemplateFormatVersion: '2010-09-09'
Transform: 'AWS::Serverless-2016-10-31'
Description: An AWS Lambda application that calls the Lambda API.
Resources:
function:
Type: [AWS::Serverless::Function](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-function.html)
Properties:
`CodeUri: build/distributions/java-basic.zip`
Handler: example.Handler
Runtime: java25
Description: Java function
MemorySize: 512
Timeout: 10
# Function's execution role
Policies:
- AWSLambdaBasicExecutionRole
- AWSLambda\_ReadOnlyAccess
- AWSXrayWriteOnlyAccess
- AWSLambdaVPCAccessExecutionRole
Tracing: Active`
```
To create the function, use the `package` and `deploy` commands. These commands are
customizations to the AWS CLI. They wrap other commands to upload the deployment package to Amazon S3, rewrite the
template with the object URI, and update the function's code.
The following example script runs a Gradle build and uploads the deployment package that it creates. It
creates an CloudFormation stack the first time you run it. If the stack already exists, the script updates it.
###### Example deploy.sh
```
`#!/bin/bash
set -eo pipefail
aws cloudformation package --template-file template.yml --s3-bucket MY\_BUCKET --output-template-file out.yml
aws cloudformation deploy --template-file out.yml --stack-name java-basic --capabilities CAPABILITY\_NAMED\_IAM`
```
For a complete working example, see the following sample applications:
###### Sample Lambda applications in Java
* [example-java](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/example-java) – A Java function that
demonstrates how you can use Lambda to process orders. This function illustrates how to define and
deserialize a custom input event object, use the AWS SDK, and output logging.
* [java-basic](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/java-basic) – A collection of minimal Java functions
with unit tests and variable logging configuration.
* [java-events](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/java-events) – A collection of Java functions that
contain skeleton code for how to handle events from various services such as Amazon API Gateway, Amazon SQS, and Amazon Kinesis.
These functions use the latest version of the [aws-lambda-java-events](./java-package.html)
library (3.0.0 and newer). These examples do not require the AWS SDK as a dependency.
* [s3-java](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/s3-java) – A Java function that processes
notification events from Amazon S3 and uses the Java Class Library (JCL) to create thumbnails from uploaded image
files.
* [layer-java](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/layer-java) – A Java function that illustrates
how to use a Lambda layer to package dependencies separate from your core function code.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Handler
Deploy container images
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.