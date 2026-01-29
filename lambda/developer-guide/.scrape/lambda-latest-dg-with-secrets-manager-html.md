---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-secrets-manager.html
title: Use Secrets Manager secrets in Lambda functions
word_count: 2544
filtered: true
elements_removed: 0
density_score: 0.83
---

Use Secrets Manager secrets in Lambda functions - AWS Lambda
Use Secrets Manager secrets in Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-secrets-manager)
[Choosing an approach](#lambda-secrets-manager-choosing-approach)[When to use Secrets Manager](#lambda-secrets-manager-when-to-use)[Using the AWS parameters and secrets Lambda extension](#lambda-secrets-manager-extension-approach)[Using the parameters utility from Powertools for AWS Lambda](#lambda-secrets-manager-powertools-approach)
# Use Secrets Manager secrets in Lambda functions
AWS Secrets Manager helps you manage credentials, API keys, and other secrets that your Lambda functions need. You have two main approaches for retrieving secrets in your Lambda functions, both offering better performance and lower costs compared to retrieving secrets directly using the AWS SDK:
* **AWS parameters and secrets Lambda extension** - A runtime-agnostic solution that provides a simple HTTP interface for retrieving secrets
* **Powertools for AWS Lambda parameters utility** - A code-integrated solution that supports multiple providers (Secrets Manager, Parameter Store, AppConfig) with built-in transformations
Both approaches maintain local caches of secrets, eliminating the need for your function to call Secrets Manager for every invocation. When your function requests a secret, the cache is checked first. If the secret is available and hasn't expired, it's returned immediately. Otherwise, it's retrieved from Secrets Manager, cached, and returned. This caching mechanism results in faster response times and reduced costs by minimizing API calls.
## Choosing an approach
Consider these factors when choosing between the extension and PowerTools:
Use the AWS parameters and secrets Lambda extension when:
* You want a runtime-agnostic solution that works with any Lambda runtime
* You prefer not to add code dependencies to your function
* You only need to retrieve secrets from Secrets Manager or Parameter Store
Use Powertools for AWS Lambda parameters utility when:
* You want an integrated development experience with your application code
* You need support for multiple providers (Secrets Manager, Parameter Store, AppConfig)
* You want built-in data transformations (JSON parsing, base64 decoding)
* You're using Python, TypeScript, Java, or .NET runtimes
## When to use Secrets Manager with Lambda
Common scenarios for using Secrets Manager with Lambda include:
* Storing database credentials that your function uses to connect to Amazon RDS or other databases
* Managing API keys for external services your function calls
* Storing encryption keys or other sensitive configuration data
* Rotating credentials automatically without needing to update your function code
## Using the AWS parameters and secrets Lambda extension
The AWS parameters and secrets Lambda extension uses a simple HTTP interface compatible with any Lambda runtime. By default, it caches secrets for 300 seconds (5 minutes) and can hold up to 1,000 secrets.
You can [customize these settings with environment variables](#lambda-secrets-manager-env-vars).
### Use Secrets Manager in a Lambda function
This section assumes that you already have a Secrets Manager secret. To create a secret, see [Create an AWS Secrets Manager secret](https://docs.aws.amazon.com/secretsmanager/latest/userguide/create_secret.html).
Choose your preferred runtime and follow the steps to create a function that retrieves secrets from Secrets Manager. The example function retrieves a secret from Secrets Manager and can be used to access database credentials, API keys,
or other sensitive configuration data in your applications.
Python
###### To create a Python function
1. Create and navigate to a new project directory. Example:
```
`mkdir my\_function
cd my\_function`
```
2. Create a file named `lambda\_function.py` with the following code. For `secret\_name`, use the name or Amazon Resource Name (ARN) of your secret.
```
`import json
import os
import requests
def lambda\_handler(event, context):
try:
# Replace with the name or ARN of your secret
secret\_name = "`arn:aws:secretsmanager:us-east-1:111122223333:secret:SECRET\_NAME`"
secrets\_extension\_endpoint = f"http://localhost:2773/secretsmanager/get?secretId={secret\_name}"
headers = {"X-Aws-Parameters-Secrets-Token": os.environ.get('AWS\_SESSION\_TOKEN')}
response = requests.get(secrets\_extension\_endpoint, headers=headers)
print(f"Response status code: {response.status\_code}")
secret = json.loads(response.text)["SecretString"]
print(f"Retrieved secret: {secret}")
return {
'statusCode': response.status\_code,
'body': json.dumps({
'message': 'Successfully retrieved secret',
'secretRetrieved': True
})
}
except Exception as e:
print(f"Error: {str(e)}")
return {
'statusCode': 500,
'body': json.dumps({
'message': 'Error retrieving secret',
'error': str(e)
})
}`
```
3. Create a file named `requirements.txt` with this content:
```
`requests`
```
4. Install the dependencies:
```
`pip install -r requirements.txt -t .`
```
5. Create a .zip file containing all files:
```
`zip -r function.zip .`
```
Node.js
###### To create a Node.js function
1. Create and navigate to a new project directory. Example:
```
`mkdir my\_function
cd my\_function`
```
2. Create a file named `index.mjs` with the following code. For `secret\_name`, use the name or Amazon Resource Name (ARN) of your secret.
```
`import http from 'http';
export const handler = async (event) =&gt; {
try {
// Replace with the name or ARN of your secret
const secretName = "`arn:aws:secretsmanager:us-east-1:111122223333:secret:SECRET\_NAME`";
const options = {
hostname: 'localhost',
port: 2773,
path: `/secretsmanager/get?secretId=${secretName}`,
headers: {
'X-Aws-Parameters-Secrets-Token': process.env.AWS\_SESSION\_TOKEN
}
};
const response = await new Promise((resolve, reject) =&gt;&gt; {
http.get(options, (res) =&gt; {
let data = '';
res.on('data', (chunk) =&gt; { data += chunk; });
res.on('end', () =&gt; {
resolve({
statusCode: res.statusCode,
body: data
});
});
}).on('error', reject);
});
const secret = JSON.parse(response.body).SecretString;
console.log('Retrieved secret:', secret);
return {
statusCode: response.statusCode,
body: JSON.stringify({
message: 'Successfully retrieved secret',
secretRetrieved: true
})
};
} catch (error) {
console.error('Error:', error);
return {
statusCode: 500,
body: JSON.stringify({
message: 'Error retrieving secret',
error: error.message
})
};
}
};`
```
3. Create a .zip file containing the `index.mjs` file:
```
`zip -r function.zip index.mjs`
```
Java
###### To create a Java function
1. Create a Maven project:
```
`mvn archetype:generate \\
-DgroupId=example \\
-DartifactId=lambda-secrets-demo \\
-DarchetypeArtifactId=maven-archetype-quickstart \\
-DarchetypeVersion=1.4 \\
-DinteractiveMode=false`
```
2. Navigate to the project directory:
```
`cd lambda-secrets-demo`
```
3. Open the `pom.xml` and replace the contents with the following:
```
`&lt;project xmlns="http://maven.apache.org/POM/4.0.0"
xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd"&gt;
&lt;modelVersion&gt;4.0.0&lt;/modelVersion&gt;
&lt;groupId&gt;example&lt;/groupId&gt;
&lt;artifactId&gt;lambda-secrets-demo&lt;/artifactId&gt;
&lt;version&gt;1.0-SNAPSHOT&lt;/version&gt;
&lt;properties&gt;
&lt;maven.compiler.source&gt;11&lt;/maven.compiler.source&gt;
&lt;maven.compiler.target&gt;11&lt;/maven.compiler.target&gt;
&lt;/properties&gt;
&lt;dependencies&gt;
&lt;dependency&gt;
&lt;groupId&gt;com.amazonaws&lt;/groupId&gt;
&lt;artifactId&gt;aws-lambda-java-core&lt;/artifactId&gt;
&lt;version&gt;1.2.1&lt;/version&gt;
&lt;/dependency&gt;
&lt;/dependencies&gt;
&lt;build&gt;
&lt;plugins&gt;
&lt;plugin&gt;
&lt;groupId&gt;org.apache.maven.plugins&lt;/groupId&gt;
&lt;artifactId&gt;maven-shade-plugin&lt;/artifactId&gt;
&lt;version&gt;3.2.4&lt;/version&gt;
&lt;executions&gt;
&lt;execution&gt;
&lt;phase&gt;package&lt;/phase&gt;
&lt;goals&gt;
&lt;goal&gt;shade&lt;/goal&gt;
&lt;/goals&gt;
&lt;configuration&gt;
&lt;createDependencyReducedPom&gt;false&lt;/createDependencyReducedPom&gt;
&lt;finalName&gt;function&lt;/finalName&gt;
&lt;/configuration&gt;
&lt;/execution&gt;
&lt;/executions&gt;
&lt;/plugin&gt;
&lt;/plugins&gt;
&lt;/build&gt;
&lt;/project&gt;`
```
4. Rename the `/lambda-secrets-demo/src/main/java/example/App.java` to `Hello.java` to match Lambda's default Java handler name (`example.Hello::handleRequest`):
```
`mv src/main/java/example/App.java src/main/java/example/Hello.java`
```
5. Open the `Hello.java` file and replace its contents with the following. For `secretName`, use the name or Amazon Resource Name (ARN) of your secret.
```
`package example;
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
public class Hello implements RequestHandler&lt;Object, String&gt; {
private final HttpClient client = HttpClient.newHttpClient();
@Override
public String handleRequest(Object input, Context context) {
try {
// Replace with the name or ARN of your secret
String secretName = "`arn:aws:secretsmanager:us-east-1:111122223333:secret:SECRET\_NAME`";
String endpoint = "http://localhost:2773/secretsmanager/get?secretId=" + secretName;
HttpRequest request = HttpRequest.newBuilder()
.uri(URI.create(endpoint))
.header("X-Aws-Parameters-Secrets-Token", System.getenv("AWS\_SESSION\_TOKEN"))
.GET()
.build();
HttpResponse&lt;&lt;String&gt;&gt; response = client.send(request,
HttpResponse.BodyHandlers.ofString());
String secret = response.body();
secret = secret.substring(secret.indexOf("SecretString") + 15);
secret = secret.substring(0, secret.indexOf("\\""));
System.out.println("Retrieved secret: " + secret);
return String.format(
"{\\"statusCode\\": %d, \\"body\\": \\"%s\\"}",
response.statusCode(), "Successfully retrieved secret"
);
} catch (Exception e) {
e.printStackTrace();
return String.format(
"{\\"body\\": \\"Error retrieving secret: %s\\"}",
e.getMessage()
);
}
}
}`
```
6. Remove the test directory. Maven creates this by default, but we don't need it for this example.
```
`rm -rf src/test`
```
7. Build the project:
```
`mvn package`
```
8. Download the JAR file (`target/function.jar`) for later use.
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose **Create function**.
3. Select **Author from scratch**.
4. For **Function name**, enter `secret-retrieval-demo`.
5. Choose your preferred **Runtime**.
6. Choose **Create function**.
###### To upload the deployment package
1. In the function's **Code** tab, choose **Upload from** and select **.zip file** (for Python and Node.js) or **.jar file** (for Java).
2. Upload the deployment package you created earlier.
3. Choose **Save**.
###### To add the AWS Parameters and Secrets Lambda extension as a layer
1. In the function's **Code** tab, scroll down to **Layers**.
2. Choose **Add a layer**.
3. Select **AWS layers**.
4. Choose **AWS-Parameters-and-Secrets-Lambda-Extension**.
5. Select the latest version.
6. Choose **Add**.
###### To add Secrets Manager permissions to your execution role
1. Choose the **Configuration** tab, and then choose **Permissions**.
2. Under **Role name**, choose the link to your execution role. This link opens the role in the IAM console.
![Link to execution role](https://docs.aws.amazon.com/images/lambda/latest/dg/images/execution-role-console.png)
3. Choose **Add permissions**, and then choose **Create inline policy**.
![Attach policies in IAM console](https://docs.aws.amazon.com/images/lambda/latest/dg/images/create-inline-policy.png)
4. Choose the **JSON** tab and add the following policy. For `Resource`, enter the ARN of your secret.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": "secretsmanager:GetSecretValue",
"Resource": "`arn:aws:secretsmanager:us-east-1:`111122223333`:secret:SECRET\_NAME`"
}
]
}`
`
```
5. Choose **Next**.
6. Enter a name for the policy.
7. Choose **Create policy**.
###### To test the function
1. Return to the Lambda console.
2. Select the **Test** tab.
3. Choose **Test**. You should see the following response:
![Successful test result](https://docs.aws.amazon.com/images/lambda/latest/dg/images/execution-results-secret.png)
### Environment variables
The AWS Parameters and Secrets Lambda extension uses the following default settings. You can override these settings by creating the corresponding [environment variables](./configuration-envvars.html#create-environment-variables). To view the current settings for a function, set `PARAMETERS\_SECRETS\_EXTENSION\_LOG\_LEVEL` to `DEBUG`. The extension will log its configuration information to CloudWatch Logs at the start of each function invocation.
|Setting|Default value|Valid values|Environment variable|Details|
|HTTP port|2773|1 - 65535|PARAMETERS\_SECRETS\_EXTENSION\_HTTP\_PORT|Port for the local HTTP server|
|Cache enabled|TRUE|TRUE | FALSE|PARAMETERS\_SECRETS\_EXTENSION\_CACHE\_ENABLED|Enable or disable the cache|
|Cache size|1000|0 - 1000|PARAMETERS\_SECRETS\_EXTENSION\_CACHE\_SIZE|Set to 0 to disable caching|
|Secrets Manager TTL|300 seconds|0 - 300 seconds|SECRETS\_MANAGER\_TTL|Time-to-live for cached secrets. Set to 0 to disable caching. This variable is ignored if the value for PARAMETERS\_SECRETS\_EXTENSION\_CACHE\_SIZE is 0.|
|Parameter Store TTL|300 seconds|0 - 300 seconds|SSM\_PARAMETER\_STORE\_TTL|Time-to-live for cached parameters. Set to 0 to disable caching. This variable is ignored if the value for PARAMETERS\_SECRETS\_EXTENSION\_CACHE\_SIZE is 0.|
|Log level|INFO|DEBUG | INFO | WARN | ERROR | NONE|PARAMETERS\_SECRETS\_EXTENSION\_LOG\_LEVEL|The level of detail reported in logs for the extension|
|Max connections|3|1 or greater|PARAMETERS\_SECRETS\_EXTENSION\_MAX\_CONNECTIONS|Maximum number of HTTP connections for requests to Parameter Store or Secrets Manager|
|Secrets Manager timeout|0 (no timeout)|All whole numbers|SECRETS\_MANAGER\_TIMEOUT\_MILLIS|Timeout for requests to Secrets Manager (in milliseconds)|
|Parameter Store timeout|0 (no timeout)|All whole numbers|SSM\_PARAMETER\_STORE\_TIMEOUT\_MILLIS|Timeout for requests to Parameter Store (in milliseconds)|
### Working with secret rotation
If you rotate secrets frequently, the default 300-second cache duration might cause your function to use outdated secrets. You have two options to ensure your function uses the latest secret value:
* Reduce the cache TTL by setting the `SECRETS\_MANAGER\_TTL` environment variable to a lower value (in seconds). For example, setting it to `60` ensures your function will never use a secret that's more than one minute old.
* Use the `AWSCURRENT` or `AWSPREVIOUS` staging labels in your secret request to ensure you get the specific version you want:
```
`secretsmanager/get?secretId=YOUR\_SECRET\_NAME&amp;&amp;versionStage=AWSCURRENT`
```
Choose the approach that best balances your needs for performance and freshness. A lower TTL means more frequent calls to Secrets Manager but ensures you're working with the most recent secret values.
## Using the parameters utility from Powertools for AWS Lambda
The parameters utility from Powertools for AWS Lambda provides a unified interface for retrieving secrets from multiple providers including Secrets Manager, parameter store, and AppConfig.
It handles caching, transformations, and provides a more integrated development experience compared to the extension approach.
### Benefits of the parameters utility
* **Multiple providers** - Retrieve parameters from Secrets Manager, Parameter Store, and AppConfig using the same interface
* **Built-in transformations** - Automatic JSON parsing, base64 decoding, and other data transformations
* **Integrated caching** - Configurable caching with TTL support to reduce API calls
* **Type safety** - Strong typing support in TypeScript and other supported runtimes
* **Error handling** - Built-in retry logic and error handling
### Code examples
The following examples show how to retrieve secrets using the Parameters utility in different runtimes:
**Python**
###### Note
For complete examples and setup instructions, see the [Parameters utility documentation](https://docs.powertools.aws.dev/lambda/python/latest/utilities/parameters/).
Retrieving secrets from Secrets Manager with Powertools for AWS Lambda Parameters utility.
```
`from aws\_lambda\_powertools import Logger
from aws\_lambda\_powertools.utilities import parameters
logger = Logger()
def lambda\_handler(event, context):
try:
# Get secret with caching (default TTL: 5 seconds)
secret\_value = parameters.get\_secret("my-secret-name")
# Get secret with custom TTL
secret\_with\_ttl = parameters.get\_secret("my-secret-name", max\_age=300)
# Get secret and transform JSON
secret\_json = parameters.get\_secret("my-json-secret", transform="json")
logger.info("Successfully retrieved secrets")
return {
'statusCode': 200,
'body': 'Successfully retrieved secrets'
}
except Exception as e:
logger.error(f"Error retrieving secret: {str(e)}")
return {
'statusCode': 500,
'body': f'Error: {str(e)}'
}`
```
**TypeScript**
###### Note
For complete examples and setup instructions, see the [Parameters utility documentation](https://docs.aws.amazon.com/powertools/typescript/2.1.1/utilities/parameters/).
Retrieving secrets from Secrets Manager with Powertools for AWS Lambda Parameters utility.
```
`import { Logger } from '@aws-lambda-powertools/logger';
import { getSecret } from '@aws-lambda-powertools/parameters/secrets';
import type { Context } from 'aws-lambda';
const logger = new Logger();
export const handler = async (event: any, context: Context) =&gt; {
try {
// Get secret with caching (default TTL: 5 seconds)
const secretValue = await getSecret('my-secret-name');
// Get secret with custom TTL
const secretWithTtl = await getSecret('my-secret-name', { maxAge: 300 });
// Get secret and transform JSON
const secretJson = await getSecret('my-json-secret', { transform: 'json' });
logger.info('Successfully retrieved secrets');
return {
statusCode: 200,
body: 'Successfully retrieved secrets'
};
} catch (error) {
logger.error('Error retrieving secret', { error });
return {
statusCode: 500,
body: `Error: ${error}`
};
}
};`
```
**Java**
###### Note
For complete examples and setup instructions, see the [Parameters utility documentation](https://docs.powertools.aws.dev/lambda/java/latest/utilities/parameters/).
Retrieving secrets from Secrets Manager with Powertools for AWS Lambda Parameters utility.
```
`import software.amazon.lambda.powertools.logging.Logging;
import software.amazon.lambda.powertools.parameters.SecretsProvider;
import software.amazon.lambda.powertools.parameters.ParamManager;
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
public class SecretHandler implements RequestHandler&lt;Object, String&gt; {
private final SecretsProvider secretsProvider = ParamManager.getSecretsProvider();
@Logging
@Override
public String handleRequest(Object input, Context context) {
try {
// Get secret with caching (default TTL: 5 seconds)
String secretValue = secretsProvider.get("my-secret-name");
// Get secret with custom TTL (300 seconds)
String secretWithTtl = secretsProvider.withMaxAge(300).get("my-secret-name");
// Get secret and transform JSON
MySecret secretJson = secretsProvider.get("my-json-secret", MySecret.class);
return "Successfully retrieved secrets";
} catch (Exception e) {
return "Error retrieving secret: " + e.getMessage();
}
}
public static class MySecret {
// Define your secret structure here
}
}`
```
**.NET**
###### Note
For complete examples and setup instructions, see the [Parameters utility documentation](https://docs.aws.amazon.com/powertools/typescript/latest/features/parameters/).
Retrieving secrets from Secrets Manager with Powertools for AWS Lambda Parameters utility.
```
`using AWS.Lambda.Powertools.Logging;
using AWS.Lambda.Powertools.Parameters;
using Amazon.Lambda.Core;
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
public class Function
{
private readonly ISecretsProvider \_secretsProvider;
public Function()
{
\_secretsProvider = ParametersManager.SecretsProvider;
}
[Logging]
public async Task&lt;&lt;string&gt;&gt; FunctionHandler(object input, ILambdaContext context)
{
try
{
// Get secret with caching (default TTL: 5 seconds)
var secretValue = await \_secretsProvider.GetAsync("my-secret-name");
// Get secret with custom TTL
var secretWithTtl = await \_secretsProvider.WithMaxAge(TimeSpan.FromMinutes(5))
.GetAsync("my-secret-name");
// Get secret and transform JSON
var secretJson = await \_secretsProvider.GetAsync&lt;&lt;MySecret&gt;&gt;("my-json-secret");
return "Successfully retrieved secrets";
}
catch (Exception e)
{
return $"Error retrieving secret: {e.Message}";
}
}
public class MySecret
{
// Define your secret structure here
}
}`
```
### Setup and permissions
To use the Parameters utility, you need to:
1. Install Powertools for AWS Lambda for your runtime. For details, see [Powertools for AWS Lambda](./powertools-for-lambda.html).
2. Add the necessary IAM permissions to your function's execution role. Refer to [Managing permissions in AWS Lambda](./lambda-permissions.html) for details.
3. Configure any optional settings through [environment variables](./configuration-envvars.html).
The required IAM permissions are the same as for the extension approach. The utility will automatically handle caching and API calls to Secrets Manager based on your configuration.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Use an Amazon S3 trigger to create thumbnails
SQS
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.