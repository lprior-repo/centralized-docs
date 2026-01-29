---
url: https://docs.aws.amazon.com/lambda/latest/dg/extensions-configuration.html
title: Copy and install the app
word_count: 491
filtered: true
elements_removed: 0
density_score: 0.87
---

Configuring Lambda extensions - AWS Lambda
Configuring Lambda extensions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#extensions-configuration)
[Configuring extensions (.zip file archive)](#using-extensions-config)[Using extensions in container images](#invocation-extensions-images)[Next steps](#using-extensions-next)
## Configuring extensions (.zip file archive)
You can add an extension to your function as a [Lambda layer](./chapter-layers.html). Using
layers enables you to share extensions across your organization or to the entire community of Lambda developers.
You can add one or more extensions to a layer. You can register up to 10 extensions for a function.
You add the extension to your function using the same method as you would for any layer. For more information,
see [Managing Lambda dependencies with layers](./chapter-layers.html).
###### Add an extension to your function (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Choose the **Code** tab if it is not already selected.
4. Under **Layers**, choose **Edit**.
5. For **Choose a layer**, choose **Specify an ARN**.
6. For **Specify an ARN**, enter the Amazon Resource Name (ARN) of an extension
layer.
7. Choose **Add**.
## Using extensions in container images
You can add extensions to your [container image](./images-create.html). The ENTRYPOINT container
image setting specifies the main process for the function. Configure the ENTRYPOINT setting in the Dockerfile, or
as an override in the function configuration.
You can run multiple processes within a container. Lambda manages the lifecycle of the main process and any
additional processes. Lambda uses the [Extensions API](./runtimes-extensions-api.html) to manage the
extension lifecycle.
### Example: Adding an external extension
An external extension runs in a separate process from the Lambda function. Lambda starts a process for each
extension in the `/opt/extensions/` directory. Lambda uses the Extensions API to manage the extension
lifecycle. After the function has run to completion, Lambda sends a `Shutdown` event to each external
extension.
###### Example of adding an external extension to a Python base image
```
`FROM public.ecr.aws/lambda/python:3.11
# Copy and install the app
COPY /app /app
WORKDIR /app
RUN pip install -r requirements.txt
# Add an extension from the local directory into /opt/extensions
ADD my-extension.zip /opt/extensions
CMD python ./my-function.py`
```
## Next steps
To learn more about extensions, we recommend the following resources:
* For a basic working example, see [Building
Extensions for AWS Lambda](https://aws.amazon.com/blogs/compute/building-extensions-for-aws-lambda-in-preview/) on the AWS Compute Blog.
* For information about extensions that AWS Lambda Partners provides, see [Introducing
AWS Lambda Extensions](https://aws.amazon.com/blogs/compute/introducing-aws-lambda-extensions-in-preview/) on the AWS Compute Blog.
* To view available example extensions and wrapper scripts, see [AWS Lambda Extensions](https://github.com/aws-samples/aws-lambda-extensions) on the AWS Samples
GitHub repository.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Lambda extensions
Extensions partners
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.