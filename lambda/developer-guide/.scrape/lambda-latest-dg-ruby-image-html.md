---
url: https://docs.aws.amazon.com/lambda/latest/dg/ruby-image.html
title: Deploy Ruby Lambda functions with container images
word_count: 3442
filtered: true
elements_removed: 0
density_score: 0.90
---

Deploy Ruby Lambda functions with container images - AWS Lambda
Deploy Ruby Lambda functions with container images - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#ruby-image)
[AWS base images for Ruby](#ruby-image-base)[Using an AWS base image](#ruby-image-instructions)[Using a non-AWS base image](#ruby-image-clients)
# Deploy Ruby Lambda functions with container images
There are three ways to build a container image for a Ruby Lambda function:
* [Using an AWS base image for Ruby](#ruby-image-instructions)
The [AWS base images](./images-create.html#runtimes-images-lp) are preloaded with a language runtime, a runtime interface client to manage the interaction between Lambda and your function code, and a runtime interface emulator for local testing.
* [Using an AWS OS-only base image](./images-create.html#runtimes-images-provided)
[AWS OS-only base images](https://gallery.ecr.aws/lambda/provided) contain an Amazon Linux distribution and the [runtime interface emulator](https://github.com/aws/aws-lambda-runtime-interface-emulator/). These images are commonly used to create container images for compiled languages, such as [Go](./go-image.html#go-image-provided) and [Rust](./lambda-rust.html), and for a language or language version that Lambda doesn't provide a base image for, such as Node.js 19. You can also use OS-only base images to implement a [custom runtime](./runtimes-custom.html). To make the image compatible with Lambda, you must include the [runtime interface client for Ruby](#ruby-image-clients) in the image.
* [Using a non-AWS base image](#ruby-image-clients)
You can use an alternative base image from another container registry, such as Alpine Linux or Debian. You can also use a custom image created by your organization. To make the image compatible with Lambda, you must include the [runtime interface client for Ruby](#ruby-image-clients) in the image.
###### Tip
To reduce the time it takes for Lambda container functions to become active, see [Use multi-stage builds](https://docs.docker.com/build/building/multi-stage/) in the Docker documentation. To build efficient container images, follow the [Best practices for writing Dockerfiles](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/).
This page explains how to build, test, and deploy container images for Lambda.
###### Topics
* [AWS base images for Ruby](#ruby-image-base)
* [Using an AWS base image for Ruby](#ruby-image-instructions)
* [Using an alternative base image with the runtime interface client](#ruby-image-clients)
## AWS base images for Ruby
AWS provides the following base images for Ruby:
|Tags|Runtime|Operating system|Dockerfile|Deprecation|
|
3.4
|Ruby 3.4|Amazon Linux 2023|[Dockerfile
for Ruby 3.4 on GitHub](https://github.com/aws/aws-lambda-base-images/blob/ruby3.4/Dockerfile.ruby3.4)|
Mar 31, 2028
|
|
3.3
|Ruby 3.3|Amazon Linux 2023|[Dockerfile
for Ruby 3.3 on GitHub](https://github.com/aws/aws-lambda-base-images/blob/ruby3.3/Dockerfile.ruby3.3)|
Mar 31, 2027
|
|
3.2
|Ruby 3.2|Amazon Linux 2|[Dockerfile
for Ruby 3.2 on GitHub](https://github.com/aws/aws-lambda-base-images/blob/ruby3.2/Dockerfile.ruby3.2)|
Mar 31, 2026
|
Amazon ECR repository: [gallery.ecr.aws/lambda/ruby](https://gallery.ecr.aws/lambda/ruby)
## Using an AWS base image for Ruby
To complete the steps in this section, you must have the following:
* [AWS CLI version 2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
* [Docker](https://docs.docker.com/get-docker) (minimum version 25.0.0)
* The Docker [buildx plugin](https://github.com/docker/buildx/blob/master/README.md).
* Ruby
###### To create a container image for Ruby
1. Create a directory for the project, and then switch to that directory.
```
`mkdir example
cd example`
```
2. Create a new file called `Gemfile`. This is where you list your application's required RubyGems packages. The AWS SDK for Ruby is available from RubyGems. You should choose specific AWS service gems to install. For example, to use the [Ruby gem for Lambda](https://rubygems.org/gems/aws-sdk-lambda/), your Gemfile should look like this:
```
`source 'https://rubygems.org'
gem 'aws-sdk-lambda'`
```
Alternatively, the [aws-sdk](https://rubygems.org/gems/aws-sdk/) gem contains every available AWS service gem. This gem is very large. We recommend that you use it only if you depend on many AWS services.
3. Install the dependencies specified in the Gemfile using [bundle install](https://bundler.io/v2.4/man/bundle-install.1.html).
```
`bundle install`
```
4. Create a new file called `lambda\_function.rb`. You can add the following sample function code to the file for testing, or use your own.
###### Example Ruby function
```
`module LambdaFunction
class Handler
def self.process(event:,context:)
"Hello from Lambda!"
end
end
end`
```
5. Create a new Dockerfile. The following is an example Dockerfile that uses an [AWS base image](./images-create.html#runtimes-images-lp). This Dockerfiles uses the following configuration:
* Set the `FROM` property to the URI of the base image.
* Use the COPY command to copy the function code and runtime dependencies to `{LAMBDA\_TASK\_ROOT}`, a [Lambda-defined environment variable](./configuration-envvars.html#configuration-envvars-runtime).
* Set the `CMD` argument to the Lambda function handler.
Note that the example Dockerfile does not include a [USER instruction](https://docs.docker.com/reference/dockerfile/#user). When you deploy a container image to Lambda, Lambda automatically defines a default Linux user with least-privileged permissions. This is different from standard Docker behavior which defaults to the `root` user when no `USER` instruction is provided.
# Install Bundler and the specified gems
RUN gem install bundler:2.4.20 &amp;&amp;&amp;&amp; \\
bundle config set --local path 'vendor/bundle' &amp;&amp;&amp;&amp; \\
bundle install
# Set the CMD to your handler (could also be done as a parameter override outside of the Dockerfile)
CMD [ "`lambda\_function.LambdaFunction::Handler.process`" ]`
```
* Build the Docker image with the [docker build](https://docs.docker.com/engine/reference/commandline/build/) command. The
following example names the image `docker-image` and gives it the `test` [tag](https://docs.docker.com/engine/reference/commandline/build/#tag). To make your
image compatible with Lambda, you must use the `--provenance=false` option.
```
`docker buildx build --platform linux/amd64 --provenance=false -t `docker-image`:`test` .`
```
###### Note
The command specifies the `--platform linux/amd64` option to ensure that your container is compatible with the Lambda execution environment regardless of the
architecture of your build machine. If you intend to create a Lambda function using the ARM64 instruction set architecture, be sure to change the command to use the `--platform linux/arm64`
option instead.
1. Start the Docker image with the **docker run** command. In this example,
`docker-image` is the image name and `test` is the tag.
```
`docker run --platform linux/amd64 -p 9000:8080 `docker-image`:`test``
```
This command runs the image as a container and creates a local endpoint at
`localhost:9000/2015-03-31/functions/function/invocations`.
###### Note
If you built the Docker image for the ARM64 instruction set architecture, be sure to use the `--platform linux/`arm64`` option instead of `--platform linux/`amd64``.
2. From a new terminal window, post an event to the local endpoint.
Linux/macOS
In Linux and macOS, run the following `curl` command:
```
`curl "http://localhost:9000/2015-03-31/functions/function/invocations" -d '{}'`
```
This command invokes the function with an empty event and returns a response. If you're using your own function code rather than the sample function code, you might want to invoke the function with a JSON payload. Example:
```
`curl "http://localhost:9000/2015-03-31/functions/function/invocations" -d '`{"payload":"hello world!"}`'`
```
PowerShell
In PowerShell, run the following `Invoke-WebRequest` command:
```
`Invoke-WebRequest -Uri "http://localhost:9000/2015-03-31/functions/function/invocations" -Method Post -Body '{}' -ContentType "application/json"`
```
This command invokes the function with an empty event and returns a response. If you're using your own function code rather than the sample function code, you might want to invoke the function with a JSON payload. Example:
```
`Invoke-WebRequest -Uri "http://localhost:9000/2015-03-31/functions/function/invocations" -Method Post -Body '`{"payload":"hello world!"}`' -ContentType "application/json"`
```
3. Get the container ID.
```
`docker ps`
```
4. Use the [docker kill](https://docs.docker.com/engine/reference/commandline/kill/) command to stop the container. In this command, replace `3766c4ab331c` with the container ID from the previous step.
```
`docker kill `3766c4ab331c``
```
###### To upload the image to Amazon ECR and create the Lambda function
1. Run the [get-login-password](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/ecr/get-login-password.html) command to authenticate the Docker CLI to your Amazon ECR registry.
* Set the `--region` value to the AWS Region where you want to create the Amazon ECR repository.
* Replace `111122223333` with your AWS account ID.
```
`aws ecr get-login-password --region `us-east-1` | docker login --username AWS --password-stdin `111122223333`.dkr.ecr.`us-east-1`.amazonaws.com`
```
* Create a repository in Amazon ECR using the [create-repository](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/ecr/create-repository.html) command.
```
`aws ecr create-repository --repository-name `hello-world` --region `us-east-1` --image-scanning-configuration scanOnPush=true --image-tag-mutability MUTABLE`
```
###### Note
The Amazon ECR repository must be in the same AWS Region as the Lambda function.
If successful, you see a response like this:
```
`{
"repository": {
"repositoryArn": "arn:aws:ecr:us-east-1:111122223333:repository/hello-world",
"registryId": "111122223333",
"repositoryName": "hello-world",
"repositoryUri": "111122223333.dkr.ecr.us-east-1.amazonaws.com/hello-world",
"createdAt": "2023-03-09T10:39:01+00:00",
"imageTagMutability": "MUTABLE",
"imageScanningConfiguration": {
"scanOnPush": true
},
"encryptionConfiguration": {
"encryptionType": "AES256"
}
}
}`
```
* Copy the `repositoryUri` from the output in the previous step.
* Run the [docker tag](https://docs.docker.com/engine/reference/commandline/tag/) command to tag your local image into your Amazon ECR repository as the latest version. In this command:
* `docker-image:test` is the name and [tag](https://docs.docker.com/engine/reference/commandline/build/#tag) of your Docker image. This is the image name and tag that you specified in the `docker build` command.
* Replace `&lt;ECRrepositoryUri&gt;` with the `repositoryUri` that you copied. Make sure to include `:latest` at the end of the URI.
```
`docker tag docker-image:test `&lt;ECRrepositoryUri&gt;`:latest`
```
Example:
```
`docker tag `docker-image`:`test` `111122223333`.dkr.ecr.`us-east-1`.amazonaws.com/`hello-world`:latest`
```
* Run the [docker push](https://docs.docker.com/engine/reference/commandline/push/) command to deploy your local image to the Amazon ECR repository. Make sure to include `:latest` at the end of the repository URI.
```
`docker push `111122223333`.dkr.ecr.`us-east-1`.amazonaws.com/`hello-world`:latest`
```
* [Create an execution role](./lambda-intro-execution-role.html#permissions-executionrole-api) for the function, if you don't already have one. You need the Amazon Resource Name (ARN) of the role in the next step.
* Create the Lambda function. For `ImageUri`, specify the repository URI from earlier. Make sure to include `:latest` at the end of the URI.
```
`aws lambda create-function \\
--function-name `hello-world` \\
--package-type Image \\
--code ImageUri=`111122223333`.dkr.ecr.`us-east-1`.amazonaws.com/`hello-world`:latest \\
--role `arn:aws:iam::111122223333:role/lambda-ex``
```
###### Note
You can create a function using an image in a different AWS account, as long as the image is in the same Region as the Lambda function. For more information, see [ Amazon ECR cross-account permissions](./images-create.html#configuration-images-xaccount-permissions).
* Invoke the function.
```
`aws lambda invoke --function-name `hello-world` response.json`
```
You should see a response like this:
```
`{
"ExecutedVersion": "$LATEST",
"StatusCode": 200
}`
```
* To see the output of the function, check the `response.json` file.
To update the function code, you must build the image again, upload the new image to the Amazon ECR repository, and then use the [update-function-code](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-function-code.html) command to deploy the image to the Lambda function.
Lambda resolves the image tag to a specific image digest. This means that if you point the image tag that was used to deploy the function to a new image in Amazon ECR, Lambda doesn't automatically update the function to use the new image.
To deploy the new image to the same Lambda function, you must use the [update-function-code](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-function-code.html) command, even if the image tag in Amazon ECR remains the same. In the following example, the `--publish` option creates a new version of the function using the updated container image.
```
`aws lambda update-function-code \\
--function-name `hello-world` \\
--image-uri `111122223333.dkr.ecr.us-east-1.amazonaws.com/hello-world:latest` \\
--publish`
```
## Using an alternative base image with the runtime interface client
If you use an [OS-only base image](./images-create.html#runtimes-images-provided) or an alternative base image, you must include the runtime interface client in your image. The runtime interface client extends the [Runtime API](./runtimes-api.html), which manages the interaction between Lambda and your function code.
Install the [Lambda runtime interface client for Ruby](https://rubygems.org/gems/aws_lambda_ric) using the RubyGems.org package manager:
```
`gem install aws\_lambda\_ric`
```
You can also download the [Ruby runtime interface client](https://github.com/aws/aws-lambda-ruby-runtime-interface-client) from GitHub.
The following example demonstrates how to build a container image for Ruby using a non-AWS base image. The example Dockerfile uses an official Ruby base image. The Dockerfile includes the runtime interface client.
To complete the steps in this section, you must have the following:
* [AWS CLI version 2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
* [Docker](https://docs.docker.com/get-docker) (minimum version 25.0.0)
* The Docker [buildx plugin](https://github.com/docker/buildx/blob/master/README.md).
* Ruby
###### To create a container image for Ruby using an alternative base image
1. Create a directory for the project, and then switch to that directory.
```
`mkdir example
cd example`
```
2. Create a new file called `Gemfile`. This is where you list your application's required RubyGems packages. The AWS SDK for Ruby is available from RubyGems. You should choose specific AWS service gems to install. For example, to use the [Ruby gem for Lambda](https://rubygems.org/gems/aws-sdk-lambda/), your Gemfile should look like this:
```
`source 'https://rubygems.org'
gem 'aws-sdk-lambda'`
```
Alternatively, the [aws-sdk](https://rubygems.org/gems/aws-sdk/) gem contains every available AWS service gem. This gem is very large. We recommend that you use it only if you depend on many AWS services.
3. Install the dependencies specified in the Gemfile using [bundle install](https://bundler.io/v2.4/man/bundle-install.1.html).
```
`bundle install`
```
4. Create a new file called `lambda\_function.rb`. You can add the following sample function code to the file for testing, or use your own.
###### Example Ruby function
```
`module LambdaFunction
class Handler
def self.process(event:,context:)
"Hello from Lambda!"
end
end
end`
```
5. Create a new Dockerfile. The following Dockerfile uses a Ruby base image instead of an [AWS base image](./images-create.html#runtimes-images-lp). The Dockerfile includes the [runtime interface client for Ruby](https://github.com/aws/aws-lambda-ruby-runtime-interface-client), which makes the image compatible with Lambda. Alternatively, you can add the runtime interface client to your application's Gemfile.
* Set the `FROM` property to the Ruby base image.
* Create a directory for the function code and an environment variable that points
to that directory. In this example, the directory is `/var/task`, which
mirrors the Lambda execution environment. However, you can choose any directory for
the function code because the Dockerfile doesn't use an AWS base image.
* Set the `ENTRYPOINT` to the module that you want the Docker container to run when it starts. In this case, the module is the runtime interface client.
* Set the `CMD` argument to the Lambda function handler.
Note that the example Dockerfile does not include a [USER instruction](https://docs.docker.com/reference/dockerfile/#user). When you deploy a container image to Lambda, Lambda automatically defines a default Linux user with least-privileged permissions. This is different from standard Docker behavior which defaults to the `root` user when no `USER` instruction is provided.
# Install the runtime interface client for Ruby
RUN gem install aws\_lambda\_ric
# Add the runtime interface client to the PATH
ENV PATH="/usr/local/bundle/bin:${PATH}"
# Create a directory for the Lambda function
ENV LAMBDA\_TASK\_ROOT=/var/task
RUN mkdir -p ${LAMBDA\_TASK\_ROOT}
WORKDIR ${LAMBDA\_TASK\_ROOT}
# Install Bundler and the specified gems
RUN gem install bundler:2.4.20 &amp;&amp;&amp;&amp; \\
bundle config set --local path 'vendor/bundle' &amp;&amp;&amp;&amp; \\
bundle install
# Set runtime interface client as default command for the container runtime
ENTRYPOINT [ "`aws\_lambda\_ric`" ]
# Set the CMD to your handler (could also be done as a parameter override outside of the Dockerfile)
CMD [ "`lambda\_function.LambdaFunction::Handler.process" ]``
```
* Build the Docker image with the [docker build](https://docs.docker.com/engine/reference/commandline/build/) command. The
following example names the image `docker-image` and gives it the `test` [tag](https://docs.docker.com/engine/reference/commandline/build/#tag). To make your
image compatible with Lambda, you must use the `--provenance=false` option.
```
`docker buildx build --platform linux/amd64 --provenance=false -t `docker-image`:`test` .`
```
###### Note
The command specifies the `--platform linux/amd64` option to ensure that your container is compatible with the Lambda execution environment regardless of the
architecture of your build machine. If you intend to create a Lambda function using the ARM64 instruction set architecture, be sure to change the command to use the `--platform linux/arm64`
option instead.
Use the [runtime interface emulator](https://github.com/aws/aws-lambda-runtime-interface-emulator/) to locally test the image. You can [build the emulator into your image](https://github.com/aws/aws-lambda-runtime-interface-emulator/?tab=readme-ov-file#build-rie-into-your-base-image) or use the following procedure to install it on your local machine.
###### To install and run the runtime interface emulator on your local machine
1. From your project directory, run the following command to download the runtime interface emulator (x86-64 architecture) from GitHub and install it on your local machine.
Linux/macOS
```
`mkdir -p \~/.aws-lambda-rie &amp;&amp;&amp;&amp; \\
curl -Lo \~/.aws-lambda-rie/aws-lambda-rie https://github.com/aws/aws-lambda-runtime-interface-emulator/releases/latest/download/aws-lambda-rie &amp;&amp;&amp;&amp; \\
chmod +x \~/.aws-lambda-rie/aws-lambda-rie`
```
To install the arm64 emulator, replace the GitHub repository URL in the previous command with the following:
```
`https://github.com/aws/aws-lambda-runtime-interface-emulator/releases/latest/download/aws-lambda-rie-arm64`
```
PowerShell
```
`$dirPath = "$HOME\\.aws-lambda-rie"
if (-not (Test-Path $dirPath)) {
New-Item -Path $dirPath -ItemType Directory
}
$downloadLink = "https://github.com/aws/aws-lambda-runtime-interface-emulator/releases/latest/download/aws-lambda-rie"
$destinationPath = "$HOME\\.aws-lambda-rie\\aws-lambda-rie"
Invoke-WebRequest -Uri $downloadLink -OutFile $destinationPath`
```
To install the arm64 emulator, replace the `$downloadLink` with the following:
```
`https://github.com/aws/aws-lambda-runtime-interface-emulator/releases/latest/download/aws-lambda-rie-arm64`
```
2. Start the Docker image with the **docker run** command. Note the following:
* `docker-image` is the image name and `test` is the tag.
* `aws\_lambda\_ric lambda\_function.LambdaFunction::Handler.process` is the `ENTRYPOINT` followed by the `CMD` from your Dockerfile.
Linux/macOS
```
`docker run --platform linux/amd64 -d -v \~/.aws-lambda-rie:/aws-lambda -p 9000:8080 \\
--entrypoint /aws-lambda/aws-lambda-rie \\
`docker-image:test` \\
`aws\_lambda\_ric lambda\_function.LambdaFunction::Handler.process``
```
PowerShell
```
`docker run --platform linux/amd64 -d -v "$HOME\\.aws-lambda-rie:/aws-lambda" -p 9000:8080 `
--entrypoint /aws-lambda/aws-lambda-rie `
`docker-image:test` `
`aws\_lambda\_ric lambda\_function.LambdaFunction::Handler.process``
```
This command runs the image as a container and creates a local endpoint at
`localhost:9000/2015-03-31/functions/function/invocations`.
###### Note
If you built the Docker image for the ARM64 instruction set architecture, be sure to use the `--platform linux/`arm64`` option instead of `--platform linux/`amd64``.
* Post an event to the local endpoint.
Linux/macOS
In Linux and macOS, run the following `curl` command:
```
`curl "http://localhost:9000/2015-03-31/functions/function/invocations" -d '{}'`
```
This command invokes the function with an empty event and returns a response. If you're using your own function code rather than the sample function code, you might want to invoke the function with a JSON payload. Example:
```
`curl "http://localhost:9000/2015-03-31/functions/function/invocations" -d '`{"payload":"hello world!"}`'`
```
PowerShell
In PowerShell, run the following `Invoke-WebRequest` command:
```
`Invoke-WebRequest -Uri "http://localhost:9000/2015-03-31/functions/function/invocations" -Method Post -Body '{}' -ContentType "application/json"`
```
This command invokes the function with an empty event and returns a response. If you're using your own function code rather than the sample function code, you might want to invoke the function with a JSON payload. Example:
```
`Invoke-WebRequest -Uri "http://localhost:9000/2015-03-31/functions/function/invocations" -Method Post -Body '`{"payload":"hello world!"}`' -ContentType "application/json"`
```
* Get the container ID.
```
`docker ps`
```
* Use the [docker kill](https://docs.docker.com/engine/reference/commandline/kill/) command to stop the container. In this command, replace `3766c4ab331c` with the container ID from the previous step.
```
`docker kill `3766c4ab331c``
```
###### To upload the image to Amazon ECR and create the Lambda function
1. Run the [get-login-password](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/ecr/get-login-password.html) command to authenticate the Docker CLI to your Amazon ECR registry.
* Set the `--region` value to the AWS Region where you want to create the Amazon ECR repository.
* Replace `111122223333` with your AWS account ID.
```
`aws ecr get-login-password --region `us-east-1` | docker login --username AWS --password-stdin `111122223333`.dkr.ecr.`us-east-1`.amazonaws.com`
```
* Create a repository in Amazon ECR using the [create-repository](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/ecr/create-repository.html) command.
```
`aws ecr create-repository --repository-name `hello-world` --region `us-east-1` --image-scanning-configuration scanOnPush=true --image-tag-mutability MUTABLE`
```
###### Note
The Amazon ECR repository must be in the same AWS Region as the Lambda function.
If successful, you see a response like this:
```
`{
"repository": {
"repositoryArn": "arn:aws:ecr:us-east-1:111122223333:repository/hello-world",
"registryId": "111122223333",
"repositoryName": "hello-world",
"repositoryUri": "111122223333.dkr.ecr.us-east-1.amazonaws.com/hello-world",
"createdAt": "2023-03-09T10:39:01+00:00",
"imageTagMutability": "MUTABLE",
"imageScanningConfiguration": {
"scanOnPush": true
},
"encryptionConfiguration": {
"encryptionType": "AES256"
}
}
}`
```
* Copy the `repositoryUri` from the output in the previous step.
* Run the [docker tag](https://docs.docker.com/engine/reference/commandline/tag/) command to tag your local image into your Amazon ECR repository as the latest version. In this command:
* `docker-image:test` is the name and [tag](https://docs.docker.com/engine/reference/commandline/build/#tag) of your Docker image. This is the image name and tag that you specified in the `docker build` command.
* Replace `&lt;ECRrepositoryUri&gt;` with the `repositoryUri` that you copied. Make sure to include `:latest` at the end of the URI.
```
`docker tag docker-image:test `&lt;ECRrepositoryUri&gt;`:latest`
```
Example:
```
`docker tag `docker-image`:`test` `111122223333`.dkr.ecr.`us-east-1`.amazonaws.com/`hello-world`:latest`
```
* Run the [docker push](https://docs.docker.com/engine/reference/commandline/push/) command to deploy your local image to the Amazon ECR repository. Make sure to include `:latest` at the end of the repository URI.
```
`docker push `111122223333`.dkr.ecr.`us-east-1`.amazonaws.com/`hello-world`:latest`
```
* [Create an execution role](./lambda-intro-execution-role.html#permissions-executionrole-api) for the function, if you don't already have one. You need the Amazon Resource Name (ARN) of the role in the next step.
* Create the Lambda function. For `ImageUri`, specify the repository URI from earlier. Make sure to include `:latest` at the end of the URI.
```
`aws lambda create-function \\
--function-name `hello-world` \\
--package-type Image \\
--code ImageUri=`111122223333`.dkr.ecr.`us-east-1`.amazonaws.com/`hello-world`:latest \\
--role `arn:aws:iam::111122223333:role/lambda-ex``
```
###### Note
You can create a function using an image in a different AWS account, as long as the image is in the same Region as the Lambda function. For more information, see [ Amazon ECR cross-account permissions](./images-create.html#configuration-images-xaccount-permissions).
* Invoke the function.
```
`aws lambda invoke --function-name `hello-world` response.json`
```
You should see a response like this:
```
`{
"ExecutedVersion": "$LATEST",
"StatusCode": 200
}`
```
* To see the output of the function, check the `response.json` file.
To update the function code, you must build the image again, upload the new image to the Amazon ECR repository, and then use the [update-function-code](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-function-code.html) command to deploy the image to the Lambda function.
Lambda resolves the image tag to a specific image digest. This means that if you point the image tag that was used to deploy the function to a new image in Amazon ECR, Lambda doesn't automatically update the function to use the new image.
To deploy the new image to the same Lambda function, you must use the [update-function-code](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-function-code.html) command, even if the image tag in Amazon ECR remains the same. In the following example, the `--publish` option creates a new version of the function using the updated container image.
```
`aws lambda update-function-code \\
--function-name `hello-world` \\
--image-uri `111122223333.dkr.ecr.us-east-1.amazonaws.com/hello-world:latest` \\
--publish`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Deploy .zip file archives
Layers
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.