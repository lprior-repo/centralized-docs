---
url: https://docs.aws.amazon.com/lambda/latest/dg/nodejs-layers.html
title: Working with layers for Node.js Lambda functions
word_count: 1290
filtered: true
elements_removed: 0
density_score: 0.89
---

Working with layers for Node.js Lambda functions - AWS Lambda
Working with layers for Node.js Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#nodejs-layers)
[Package your layer content](#nodejs-layers-package)[Create the layer in Lambda](#publishing-layer)[Add the layer to your function](#nodejs-layer-adding)[Sample app](#nodejs-layer-sample-app)
# Working with layers for Node.js Lambda functions
Use [Lambda layers](./chapter-layers.html) to package code and dependencies that you want to reuse across multiple functions. Layers usually contain library dependencies, a [custom runtime](./runtimes-custom.html), or configuration files. Creating a layer involves
three general steps:
1. Package your layer content. This means creating a .zip file archive that contains the dependencies
you want to use in your functions.
2. Create the layer in Lambda.
3. Add the layer to your functions.
###### Topics
* [Package your layer content](#nodejs-layers-package)
* [Create the layer in Lambda](#publishing-layer)
* [Add the layer to your function](#nodejs-layer-adding)
* [Sample app](#nodejs-layer-sample-app)
## Package your layer content
To create a layer, bundle your packages into a .zip file archive that meets the following requirements:
* Build the layer using the same Node.js version that you plan to use for the Lambda function. For example, if you build your layer using Node.js 24, use the Node.js 24 runtime for your function.
* Your layer's .zip file must use one of these directory structures:
* `nodejs/node\_modules`
* `nodejs/node`X`/node\_modules` (where `X` is your Node.js version, for example `node22`)
For more information, see [Layer paths for each Lambda runtime](./packaging-layers.html#packaging-layers-paths).
* The packages in your layer must be compatible with Linux. Lambda functions run on Amazon Linux.
You can create layers that contain either third-party Node.js libraries installed with `npm` (such as `axios` or `lodash`) or your own JavaScript modules.
###### To create a layer using npm packages
1. Create the required directory structure and install packages directly into it:
```
`mkdir -p nodejs
npm install --prefix nodejs lodash axios`
```
This command installs the packages directly into the `nodejs/node\_modules` directory, which is the structure that Lambda requires.
###### Note
For packages with native dependencies or binary components (such as [sharp](https://www.npmjs.com/package/sharp) or [bcrypt](https://www.npmjs.com/package/bcrypt)), ensure that they're compatible with the Lambda Linux environment and your function's [architecture](./foundation-arch.html). You might need to use the `--platform` flag:
```
`npm install --prefix nodejs --platform=linux --arch=x64 sharp`
```
For more complex native dependencies, you might need to compile them in a Linux environment that matches the Lambda runtime. You can use Docker for this purpose.
2. Zip the layer content:
Linux/macOS
```
`zip -r layer.zip nodejs/`
```
PowerShell
```
`Compress-Archive -Path .\\nodejs -DestinationPath .\\layer.zip`
```
The directory structure of your .zip file should look like this:
```
nodejs/
├── package.json
├── package-lock.json
└── node\_modules/
├── lodash/
├── axios/
└── (dependencies of the other packages)
```
###### Note
* Make sure your .zip file includes the `nodejs` directory at the root level with `node\_modules` inside it. This structure ensures that Lambda can locate and import your packages.
* The `package.json` and `package-lock.json` files in the `nodejs/` directory are used by npm for dependency management but are not required by Lambda for layer functionality. Each installed package already contains its own `package.json` file that defines how Lambda imports the package.
###### To create a layer using your own code
1. Create the required directory structure for your layer:
```
`mkdir -p nodejs/node\_modules/validator
cd nodejs/node\_modules/validator`
```
2. Create a `package.json` file for your custom module to define how it should be imported:
###### Example nodejs/node\_modules/validator/package.json
```
`{
"name": "validator",
"version": "1.0.0",
"type": "module",
"main": "index.mjs"
}`
```
3. Create your JavaScript module file:
###### Example nodejs/node\_modules/validator/index.mjs
```
`export function validateOrder(orderData) {
// Validates an order and returns formatted data
const requiredFields = ['productId', 'quantity'];
// Check required fields
const missingFields = requiredFields.filter(field =&gt; !(field in orderData));
if (missingFields.length &gt; 0) {
throw new Error(`Missing required fields: ${missingFields.join(', ')}`);
}
// Validate quantity
const quantity = orderData.quantity;
if (!Number.isInteger(quantity) || quantity &lt; 1) {
throw new Error('Quantity must be a positive integer');
}
// Format and return the validated data
return {
productId: String(orderData.productId),
quantity: quantity,
shippingPriority: orderData.priority || 'standard'
};
}
export function formatResponse(statusCode, body) {
// Formats the API response
return {
statusCode: statusCode,
body: JSON.stringify(body)
};
}`
```
4. Zip the layer content:
Linux/macOS
```
`zip -r layer.zip nodejs/`
```
PowerShell
```
`Compress-Archive -Path .\\nodejs -DestinationPath .\\layer.zip`
```
The directory structure of your .zip file should look like this:
```
nodejs/
└── node\_modules/
└── validator/
├── package.json
└── index.mjs
```
5. In your function, import and use the modules. Example:
```
`import { validateOrder, formatResponse } from 'validator';
export const handler = async (event) =&gt; {
try {
// Parse the order data from the event body
const orderData = JSON.parse(event.body || '{}');
// Validate and format the order
const validatedOrder = validateOrder(orderData);
return formatResponse(200, {
message: 'Order validated successfully',
order: validatedOrder
});
} catch (error) {
if (error instanceof Error &amp;&amp; error.message.includes('Missing required fields')) {
return formatResponse(400, {
error: error.message
});
}
return formatResponse(500, {
error: 'Internal server error'
});
}
};`
```
You can use the following [test event](./testing-functions.html#invoke-with-event) to invoke the function:
```
`{
"body": "{\\"productId\\": \\"ABC123\\", \\"quantity\\": 2, \\"priority\\": \\"express\\"}"
}`
```
Expected response:
```
`{
"statusCode": 200,
"body": "{\\"message\\":\\"Order validated successfully\\",\\"order\\":{\\"productId\\":\\"ABC123\\",\\"quantity\\":2,\\"shippingPriority\\":\\"express\\"}}"
}`
```
## Create the layer in Lambda
You can publish your layer using either the AWS CLI or the Lambda console.
AWS CLI
Run the [publish-layer-version](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/publish-layer-version.html) AWS CLI command to create the Lambda layer:
```
`aws lambda publish-layer-version --layer-name `my-layer` --zip-file fileb://layer.zip --compatible-runtimes `nodejs24.x``
```
The [compatible runtimes](https://docs.aws.amazon.com/lambda/latest/api/API_PublishLayerVersion.html#lambda-PublishLayerVersion-request-CompatibleRuntimes) parameter is optional. When specified, Lambda uses this parameter to filter layers in the Lambda console.
Console
###### To create a layer (console)
1. Open the [Layers page](https://console.aws.amazon.com/lambda/home#/layers)
of the Lambda console.
2. Choose **Create layer**.
3. Choose **Upload a .zip file**, and then upload the .zip archive that you created earlier.
4. (Optional) For **Compatible runtimes**, choose the Node.js runtime that corresponds to the Node.js version you used to build your layer.
5. Choose **Create**.
## Add the layer to your function
AWS CLI
To attach the layer to your function, run the [update-function-configuration](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-function-configuration.html) AWS CLI command. For the `--layers` parameter, use the layer ARN. The ARN must specify the version (for example, `arn:aws:lambda:us-east-1:123456789012:layer:my-layer:`1``). For more information, see [Layers and layer versions](./chapter-layers.html#lambda-layer-versions).
```
`aws lambda update-function-configuration --function-name `my-function` --cli-binary-format raw-in-base64-out --layers "`arn:aws:lambda:us-east-1:123456789012:layer:my-layer:`1``"`
```
The **cli-binary-format** option is required if you're using AWS CLI version 2. To make this the default setting, run `aws configure set cli-binary-format raw-in-base64-out`. For more information, see [AWS CLI supported global command line options](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-options.html#cli-configure-options-list) in the *AWS Command Line Interface User Guide for Version 2*.
Console
###### To add a layer to a function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose the function.
3. Scroll down to the **Layers** section, and then choose **Add a layer**.
4. Under **Choose a layer**, select **Custom layers**, and then choose your layer.
###### Note
If you didn't add a [compatible runtime](https://docs.aws.amazon.com/lambda/latest/api/API_PublishLayerVersion.html#lambda-PublishLayerVersion-request-CompatibleRuntimes) when you created the layer, your layer won't be listed here. You can specify the layer ARN instead.
5. Choose **Add**.
## Sample app
For more examples of how to use Lambda layers, see the [layer-nodejs](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/layer-nodejs) sample application in the AWS Lambda Developer Guide GitHub repository. This application includes a layer that contains the [lodash](https://www.npmjs.com/package/lodash) library. After creating the layer, you can deploy and invoke the corresponding function to confirm that the layer works as expected.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Deploy container images
Context
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.