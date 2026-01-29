---
url: https://docs.aws.amazon.com/lambda/latest/dg/java-tracing.html
title: Instrumenting Java code in AWS Lambda
word_count: 3686
filtered: true
elements_removed: 0
density_score: 0.82
---

Instrumenting Java code in AWS Lambda - AWS Lambda
Instrumenting Java code in AWS Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#java-tracing)
[Using Powertools for AWS Lambda (Java) and AWS SAM for tracing](#java-tracing-sam)[Using Powertools for AWS Lambda (Java) and the AWS CDK for tracing](#java-tracing-cdk)[Using ADOT to instrument your Java functions](#java-adot)[Using the X-Ray SDK to instrument your Java functions](#java-xray-sdk)[Activating tracing with the Lambda console](#java-tracing-console)[Activating tracing with the Lambda API](#java-tracing-api)[Activating tracing with CloudFormation](#java-tracing-cloudformation)[Interpreting an X-Ray trace](#java-tracing-interpretation)[Storing runtime dependencies in a layer (X-Ray SDK)](#java-tracing-layers)[X-Ray tracing in sample applications (X-Ray SDK)](#java-tracing-samples)
# Instrumenting Java code in AWS Lambda
Lambda integrates with AWS X-Ray to help you trace, debug, and optimize Lambda applications. You can use X-Ray
to trace a request as it traverses resources in your application, which may include Lambda functions and other AWS
services.
To send tracing data to X-Ray, you can use one of two SDK libraries:
* [AWS Distro for OpenTelemetry (ADOT)](https://aws.amazon.com/otel) – A secure, production-ready,
AWS-supported distribution of the OpenTelemetry (OTel) SDK.
* [AWS X-Ray SDK for Java](https://docs.aws.amazon.com/xray/latest/devguide/xray-sdk-java.html) – An SDK
for generating and sending trace data to X-Ray.
* [Powertools for AWS Lambda (Java)](https://docs.aws.amazon.com/powertools/java/latest/) – A developer toolkit to implement Serverless
best practices and increase developer velocity.
Each of the SDKs offer ways to send your telemetry data to the X-Ray service.
You can then use X-Ray to view, filter, and gain insights into your application's performance metrics to identify
issues and opportunities for optimization.
###### Important
The X-Ray and Powertools for AWS Lambda SDKs are part of a tightly integrated instrumentation solution offered by AWS.
The ADOT Lambda Layers are part of an industry-wide standard for tracing instrumentation that collect more data in general, but may not be
suited for all use cases. You can implement end-to-end tracing in X-Ray using either solution. To learn more about choosing between them, see
[Choosing between the AWS
Distro for Open Telemetry and X-Ray SDKs](https://docs.aws.amazon.com/xray/latest/devguide/xray-instrumenting-your-app.html#xray-instrumenting-choosing).
###### Sections
* [Using Powertools for AWS Lambda (Java) and AWS SAM for tracing](#java-tracing-sam)
* [Using Powertools for AWS Lambda (Java) and the AWS CDK for tracing](#java-tracing-cdk)
* [Using ADOT to instrument your Java functions](#java-adot)
* [Using the X-Ray SDK to instrument your Java functions](#java-xray-sdk)
* [Activating tracing with the Lambda console](#java-tracing-console)
* [Activating tracing with the Lambda API](#java-tracing-api)
* [Activating tracing with CloudFormation](#java-tracing-cloudformation)
* [Interpreting an X-Ray trace](#java-tracing-interpretation)
* [Storing runtime dependencies in a layer (X-Ray SDK)](#java-tracing-layers)
* [X-Ray tracing in sample applications (X-Ray SDK)](#java-tracing-samples)
## Using Powertools for AWS Lambda (Java) and AWS SAM for tracing
Follow the steps below to download, build, and deploy a sample Hello World Java application with integrated [Powertools for AWS Lambda (Java)](https://docs.powertools.aws.dev/lambda-java) modules using the AWS SAM. This application implements a
basic API backend and uses Powertools for emitting logs, metrics, and traces. It consists of an Amazon API Gateway endpoint and a Lambda function.
When you send a GET request to the API Gateway endpoint, the Lambda function invokes, sends logs and metrics using Embedded Metric Format to CloudWatch, and
sends traces to AWS X-Ray. The function returns a `hello world` message.
###### Prerequisites
To complete the steps in this section, you must have the following:
* Java 11 or later
* [AWS CLI version 2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
* [AWS SAM CLI version 1.75 or later](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html). If you have an older version of the AWS SAM CLI, see [Upgrading the AWS SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/manage-sam-cli-versions.html#manage-sam-cli-versions-upgrade).
###### Deploy a sample AWS SAM application
1. Initialize the application using the Hello World Java template.
```
`sam init --app-template hello-world-powertools-java --name sam-app --package-type Zip --runtime java11 --no-tracing`
```
2. Build the app.
```
`cd sam-app &amp;&amp; sam build`
```
3. Deploy the app.
```
`sam deploy --guided`
```
4. Follow the on-screen prompts. To accept the default options provided in the interactive experience, press `Enter`.
###### Note
For **HelloWorldFunction may not have authorization defined, Is this okay?**, make sure to enter `y`.
5. Get the URL of the deployed application:
```
`aws cloudformation describe-stacks --stack-name sam-app --query 'Stacks[0].Outputs[?OutputKey==`HelloWorldApi`].OutputValue' --output text`
```
6. Invoke the API endpoint:
```
`curl -X GET `&lt;&lt;URL\_FROM\_PREVIOUS\_STEP&gt;&gt;``
```
If successful, you'll see this response:
```
{"message":"hello world"}
```
7. To get the traces for the function, run [sam traces](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-cli-command-reference-sam-traces.html).
```
`sam traces`
```
The trace output looks like this:
```
`New XRay Service Graph
Start time: 2025-02-03 14:31:48+01:00
End time: 2025-02-03 14:31:48+01:00
Reference Id: 0 - (Root) AWS::Lambda - sam-app-HelloWorldFunction-y9Iu1FLJJBGD - Edges: []
Summary\_statistics:
- total requests: 1
- ok count(2XX): 1
- error count(4XX): 0
- fault count(5XX): 0
- total response time: 5.587
Reference Id: 1 - client - sam-app-HelloWorldFunction-y9Iu1FLJJBGD - Edges: [0]
Summary\_statistics:
- total requests: 0
- ok count(2XX): 0
- error count(4XX): 0
- fault count(5XX): 0
- total response time: 0
XRay Event [revision 3] at (2025-02-03T14:31:48.500000) with id (1-63dd0cc4-3c869dec72a586875da39777) and duration (5.603s)
- 5.587s - sam-app-HelloWorldFunction-y9Iu1FLJJBGD [HTTP: 200]
- 4.053s - sam-app-HelloWorldFunction-y9Iu1FLJJBGD
- 1.181s - Initialization
- 4.037s - Invocation
- 1.981s - ## handleRequest
- 1.840s - ## getPageContents
- 0.000s - Overhead`
```
8. This is a public API endpoint that is accessible over the internet. We recommend that you delete the endpoint after testing.
```
`sam delete`
```
## Using Powertools for AWS Lambda (Java) and the AWS CDK for tracing
Follow the steps below to download, build, and deploy a sample Hello World Java application with integrated [Powertools for AWS Lambda (Java)](https://docs.powertools.aws.dev/lambda-java) modules using the AWS CDK. This application implements a
basic API backend and uses Powertools for emitting logs, metrics, and traces. It consists of an Amazon API Gateway endpoint and a Lambda function.
When you send a GET request to the API Gateway endpoint, the Lambda function invokes, sends logs and metrics using Embedded Metric Format to CloudWatch, and
sends traces to AWS X-Ray. The function returns a hello world message.
###### Prerequisites
To complete the steps in this section, you must have the following:
* Java 11 or later
* [AWS CLI version 2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
* [AWS CDK version 2](https://docs.aws.amazon.com/cdk/v2/guide/getting_started.html#getting_started_prerequisites)
* [AWS SAM CLI version 1.75 or later](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html). If you have an older version of the AWS SAM CLI, see [Upgrading the AWS SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/manage-sam-cli-versions.html#manage-sam-cli-versions-upgrade).
###### Deploy a sample AWS CDK application
1. Create a project directory for your new application.
```
`mkdir hello-world
cd hello-world`
```
2. Initialize the app.
```
`cdk init app --language java`
```
3. Create a maven project with the following command:
```
`mkdir app
cd app
mvn archetype:generate -DgroupId=helloworld -DartifactId=Function -DarchetypeArtifactId=maven-archetype-quickstart -DinteractiveMode=false`
```
4. Open `pom.xml` in the `hello-world\\app\\Function` directory and replace the existing code with the following code that includes dependencies and maven plugins for Powertools.
```
`&lt;&lt;project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/maven-v4\_0\_0.xsd"&gt;&gt;
&lt;&lt;modelVersion&gt;&gt;4.0.0&lt;&lt;/modelVersion&gt;&gt;
&lt;&lt;groupId&gt;&gt;helloworld&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;Function&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;packaging&gt;&gt;jar&lt;&lt;/packaging&gt;&gt;
&lt;&lt;version&gt;&gt;1.0-SNAPSHOT&lt;&lt;/version&gt;&gt;
&lt;&lt;name&gt;&gt;Function&lt;&lt;/name&gt;&gt;
&lt;&lt;url&gt;&gt;http://maven.apache.org&lt;&lt;/url&gt;&gt;
&lt;&lt;properties&gt;&gt;
&lt;&lt;maven.compiler.source&gt;&gt;11&lt;&lt;/maven.compiler.source&gt;&gt;
&lt;&lt;maven.compiler.target&gt;&gt;11&lt;&lt;/maven.compiler.target&gt;&gt;
&lt;&lt;log4j.version&gt;&gt;2.17.2&lt;&lt;/log4j.version&gt;&gt;
&lt;&lt;/properties&gt;&gt;
&lt;&lt;dependencies&gt;&gt;
&lt;&lt;dependency&gt;&gt;
&lt;&lt;groupId&gt;&gt;junit&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;junit&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;3.8.1&lt;&lt;/version&gt;&gt;
&lt;&lt;scope&gt;&gt;test&lt;&lt;/scope&gt;&gt;
&lt;&lt;/dependency&gt;&gt;
&lt;&lt;dependency&gt;&gt;
&lt;&lt;groupId&gt;&gt;software.amazon.lambda&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;powertools-tracing&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;1.3.0&lt;&lt;/version&gt;&gt;
&lt;&lt;/dependency&gt;&gt;
&lt;&lt;dependency&gt;&gt;
&lt;&lt;groupId&gt;&gt;software.amazon.lambda&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;powertools-metrics&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;1.3.0&lt;&lt;/version&gt;&gt;
&lt;&lt;/dependency&gt;&gt;
&lt;&lt;dependency&gt;&gt;
&lt;&lt;groupId&gt;&gt;software.amazon.lambda&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;powertools-logging&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;1.3.0&lt;&lt;/version&gt;&gt;
&lt;&lt;/dependency&gt;&gt;
&lt;&lt;dependency&gt;&gt;
&lt;&lt;groupId&gt;&gt;com.amazonaws&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;aws-lambda-java-core&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;1.2.2&lt;&lt;/version&gt;&gt;
&lt;&lt;/dependency&gt;&gt;
&lt;&lt;dependency&gt;&gt;
&lt;&lt;groupId&gt;&gt;com.amazonaws&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;aws-lambda-java-events&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;3.11.1&lt;&lt;/version&gt;&gt;
&lt;&lt;/dependency&gt;&gt;
&lt;&lt;/dependencies&gt;&gt;
&lt;&lt;build&gt;&gt;
&lt;&lt;plugins&gt;&gt;
&lt;&lt;plugin&gt;&gt;
&lt;&lt;groupId&gt;&gt;org.codehaus.mojo&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;aspectj-maven-plugin&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;1.14.0&lt;&lt;/version&gt;&gt;
&lt;&lt;configuration&gt;&gt;
&lt;&lt;source&gt;&gt;${maven.compiler.source}&lt;/source&gt;
&lt;target&gt;${maven.compiler.target}&lt;/target&gt;
&lt;complianceLevel&gt;${maven.compiler.target}&lt;&lt;/complianceLevel&gt;&gt;
&lt;&lt;aspectLibraries&gt;&gt;
&lt;&lt;aspectLibrary&gt;&gt;
&lt;&lt;groupId&gt;&gt;software.amazon.lambda&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;powertools-tracing&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;/aspectLibrary&gt;&gt;
&lt;&lt;aspectLibrary&gt;&gt;
&lt;&lt;groupId&gt;&gt;software.amazon.lambda&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;powertools-metrics&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;/aspectLibrary&gt;&gt;
&lt;&lt;aspectLibrary&gt;&gt;
&lt;&lt;groupId&gt;&gt;software.amazon.lambda&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;powertools-logging&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;/aspectLibrary&gt;&gt;
&lt;&lt;/aspectLibraries&gt;&gt;
&lt;&lt;/configuration&gt;&gt;
&lt;&lt;executions&gt;&gt;
&lt;&lt;execution&gt;&gt;
&lt;&lt;goals&gt;&gt;
&lt;&lt;goal&gt;&gt;compile&lt;&lt;/goal&gt;&gt;
&lt;&lt;/goals&gt;&gt;
&lt;&lt;/execution&gt;&gt;
&lt;&lt;/executions&gt;&gt;
&lt;&lt;/plugin&gt;&gt;
&lt;&lt;plugin&gt;&gt;
&lt;&lt;groupId&gt;&gt;org.apache.maven.plugins&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;maven-shade-plugin&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;3.4.1&lt;&lt;/version&gt;&gt;
&lt;&lt;executions&gt;&gt;
&lt;&lt;execution&gt;&gt;
&lt;&lt;phase&gt;&gt;package&lt;&lt;/phase&gt;&gt;
&lt;&lt;goals&gt;&gt;
&lt;&lt;goal&gt;&gt;shade&lt;&lt;/goal&gt;&gt;
&lt;&lt;/goals&gt;&gt;
&lt;&lt;configuration&gt;&gt;
&lt;&lt;transformers&gt;&gt;
&lt;&lt;transformer
implementation="com.github.edwgiz.maven\_shade\_plugin.log4j2\_cache\_transformer.PluginsCacheFileTransformer"&gt;&gt;
&lt;&lt;/transformer&gt;&gt;
&lt;&lt;/transformers&gt;&gt;
&lt;&lt;createDependencyReducedPom&gt;&gt;false&lt;&lt;/createDependencyReducedPom&gt;&gt;
&lt;&lt;finalName&gt;&gt;function&lt;&lt;/finalName&gt;&gt;
&lt;&lt;/configuration&gt;&gt;
&lt;&lt;/execution&gt;&gt;
&lt;&lt;/executions&gt;&gt;
&lt;&lt;dependencies&gt;&gt;
&lt;&lt;dependency&gt;&gt;
&lt;&lt;groupId&gt;&gt;com.github.edwgiz&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;maven-shade-plugin.log4j2-cachefile-transformer&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;2.15&lt;&lt;/version&gt;&gt;
&lt;&lt;/dependency&gt;&gt;
&lt;&lt;/dependencies&gt;&gt;
&lt;&lt;/plugin&gt;&gt;
&lt;&lt;/plugins&gt;&gt;
&lt;&lt;/build&gt;&gt;
&lt;&lt;/project&gt;&gt;
`
```
5. Create the `hello-world\\app\\src\\main\\resource` directory and create `log4j.xml` for the log configuration.
```
`mkdir -p src/main/resource
cd src/main/resource
touch log4j.xml`
```
6. Open `log4j.xml` and add the following code.
```
`&lt;&lt;?xml version="1.0" encoding="UTF-8"?&gt;&gt;
&lt;&lt;Configuration&gt;&gt;
&lt;&lt;Appenders&gt;&gt;
&lt;&lt;Console name="JsonAppender" target="SYSTEM\_OUT"&gt;&gt;
&lt;&lt;JsonTemplateLayout eventTemplateUri="classpath:LambdaJsonLayout.json" /&gt;&gt;
&lt;&lt;/Console&gt;&gt;
&lt;&lt;/Appenders&gt;&gt;
&lt;&lt;Loggers&gt;&gt;
&lt;&lt;Logger name="JsonLogger" level="INFO" additivity="false"&gt;&gt;
&lt;&lt;AppenderRef ref="JsonAppender"/&gt;&gt;
&lt;&lt;/Logger&gt;&gt;
&lt;&lt;Root level="info"&gt;&gt;
&lt;&lt;AppenderRef ref="JsonAppender"/&gt;&gt;
&lt;&lt;/Root&gt;&gt;
&lt;&lt;/Loggers&gt;&gt;
&lt;&lt;/Configuration&gt;&gt;`
```
7. Open `App.java` from the `hello-world\\app\\Function\\src\\main\\java\\helloworld` directory and replace the existing code with the following code.
This is the code for the Lambda function.
```
`package helloworld;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.URL;
import java.util.HashMap;
import java.util.Map;
import java.util.stream.Collectors;
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.APIGatewayProxyRequestEvent;
import com.amazonaws.services.lambda.runtime.events.APIGatewayProxyResponseEvent;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import software.amazon.lambda.powertools.logging.Logging;
import software.amazon.lambda.powertools.metrics.Metrics;
import software.amazon.lambda.powertools.tracing.CaptureMode;
import software.amazon.lambda.powertools.tracing.Tracing;
import static software.amazon.lambda.powertools.tracing.CaptureMode.\*;
/\*\*
\* Handler for requests to Lambda function.
\*/
public class App implements RequestHandler&lt;&lt;APIGatewayProxyRequestEvent, APIGatewayProxyResponseEvent&gt;&gt; {
Logger log = LogManager.getLogger(App.class);
@Logging(logEvent = true)
@Tracing(captureMode = DISABLED)
@Metrics(captureColdStart = true)
public APIGatewayProxyResponseEvent handleRequest(final APIGatewayProxyRequestEvent input, final Context context) {
Map&lt;String, String&gt; headers = new HashMap&lt;&gt;();
headers.put("Content-Type", "application/json");
headers.put("X-Custom-Header", "application/json");
APIGatewayProxyResponseEvent response = new APIGatewayProxyResponseEvent()
.withHeaders(headers);
try {
final String pageContents = this.getPageContents("https://checkip.amazonaws.com");
String output = String.format("{ \\"message\\": \\"hello world\\", \\"location\\": \\"%s\\" }", pageContents);
return response
.withStatusCode(200)
.withBody(output);
} catch (IOException e) {
return response
.withBody("{}")
.withStatusCode(500);
}
}
@Tracing(namespace = "getPageContents")
private String getPageContents(String address) throws IOException {
log.info("Retrieving {}", address);
URL url = new URL(address);
try (BufferedReader br = new BufferedReader(new InputStreamReader(url.openStream()))) {
return br.lines().collect(Collectors.joining(System.lineSeparator()));
}
}
}`
```
8. Open `HelloWorldStack.java` from the `hello-world\\src\\main\\java\\com\\myorg` directory and replace the existing code with the following code. This code uses
[Lambda Constructor](https://docs.aws.amazon.com/cdk/api/v1/java/aws_cdk.aws_lambda.html)and the
[ApiGatewayv2 Constructor](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.aws_apigatewayv2-readme.html)
to create a REST API and a Lambda function.
```
`package com.myorg;
import software.amazon.awscdk.\*;
import software.amazon.awscdk.services.apigatewayv2.alpha.\*;
import software.amazon.awscdk.services.apigatewayv2.integrations.alpha.HttpLambdaIntegration;
import software.amazon.awscdk.services.apigatewayv2.integrations.alpha.HttpLambdaIntegrationProps;
import software.amazon.awscdk.services.lambda.Code;
import software.amazon.awscdk.services.lambda.Function;
import software.amazon.awscdk.services.lambda.FunctionProps;
import software.amazon.awscdk.services.lambda.Runtime;
import software.amazon.awscdk.services.lambda.Tracing;
import software.amazon.awscdk.services.logs.RetentionDays;
import software.amazon.awscdk.services.s3.assets.AssetOptions;
import software.constructs.Construct;
import java.util.Arrays;
import java.util.List;
import static java.util.Collections.singletonList;
import static software.amazon.awscdk.BundlingOutput.ARCHIVED;
public class HelloWorldStack extends Stack {
public HelloWorldStack(final Construct scope, final String id) {
this(scope, id, null);
}
public HelloWorldStack(final Construct scope, final String id, final StackProps props) {
super(scope, id, props);
List&lt;&lt;String&gt;&gt; functionPackagingInstructions = Arrays.asList(
"/bin/sh",
"-c",
"cd Function " +
"&amp;&amp;&amp;&amp; mvn clean install " +
"&amp;&amp;&amp;&amp; cp /asset-input/Function/target/function.jar /asset-output/"
);
BundlingOptions.Builder builderOptions = BundlingOptions.builder()
.command(functionPackagingInstructions)
.image(Runtime.JAVA\_11.getBundlingImage())
.volumes(singletonList(
// Mount local .m2 repo to avoid download all the dependencies again inside the container
DockerVolume.builder()
.hostPath(System.getProperty("user.home") + "/.m2/")
.containerPath("/root/.m2/")
.build()
))
.user("root")
.outputType(ARCHIVED);
Function function = new Function(this, "Function", FunctionProps.builder()
.runtime(Runtime.JAVA\_11)
.code(Code.fromAsset("app", AssetOptions.builder()
.bundling(builderOptions
.command(functionPackagingInstructions)
.build())
.build()))
.handler("helloworld.App::handleRequest")
.memorySize(1024)
.tracing(Tracing.ACTIVE)
.timeout(Duration.seconds(10))
.logRetention(RetentionDays.ONE\_WEEK)
.build());
HttpApi httpApi = new HttpApi(this, "sample-api", HttpApiProps.builder()
.apiName("sample-api")
.build());
httpApi.addRoutes(AddRoutesOptions.builder()
.path("/")
.methods(singletonList(HttpMethod.GET))
.integration(new HttpLambdaIntegration("function", function, HttpLambdaIntegrationProps.builder()
.payloadFormatVersion(PayloadFormatVersion.VERSION\_2\_0)
.build()))
.build());
new CfnOutput(this, "HttpApi", CfnOutputProps.builder()
.description("Url for Http Api")
.value(httpApi.getApiEndpoint())
.build());
}
}`
```
9. Open `pom.xml` from the `hello-world` directory and replace the existing code with the following code.
```
`&lt;?xml version="1.0" encoding="UTF-8"?&gt;
&lt;project xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd"
xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"&gt;
&lt;modelVersion&gt;4.0.0&lt;/modelVersion&gt;
&lt;groupId&gt;com.myorg&lt;/groupId&gt;
&lt;artifactId&gt;hello-world&lt;/artifactId&gt;
&lt;version&gt;0.1&lt;/version&gt;
&lt;properties&gt;
&lt;project.build.sourceEncoding&gt;UTF-8&lt;/project.build.sourceEncoding&gt;
&lt;cdk.version&gt;2.70.0&lt;/cdk.version&gt;
&lt;constructs.version&gt;[10.0.0,11.0.0)&lt;/constructs.version&gt;
&lt;junit.version&gt;5.7.1&lt;/junit.version&gt;
&lt;/properties&gt;
&lt;build&gt;
&lt;plugins&gt;
&lt;plugin&gt;
&lt;groupId&gt;org.apache.maven.plugins&lt;/groupId&gt;
&lt;artifactId&gt;maven-compiler-plugin&lt;/artifactId&gt;
&lt;version&gt;3.8.1&lt;/version&gt;
&lt;configuration&gt;
&lt;source&gt;1.8&lt;/source&gt;
&lt;target&gt;1.8&lt;/target&gt;
&lt;/configuration&gt;
&lt;/plugin&gt;
&lt;plugin&gt;
&lt;groupId&gt;org.codehaus.mojo&lt;/groupId&gt;
&lt;artifactId&gt;exec-maven-plugin&lt;/artifactId&gt;
&lt;version&gt;3.0.0&lt;/version&gt;
&lt;configuration&gt;
&lt;mainClass&gt;com.myorg.HelloWorldApp&lt;/mainClass&gt;
&lt;/configuration&gt;
&lt;/plugin&gt;
&lt;/plugins&gt;
&lt;/build&gt;
&lt;dependencies&gt;
&lt;!-- AWS Cloud Development Kit --&gt;
&lt;dependency&gt;
&lt;groupId&gt;software.amazon.awscdk&lt;/groupId&gt;
&lt;artifactId&gt;aws-cdk-lib&lt;/artifactId&gt;
&lt;version&gt;${cdk.version}&lt;/version&gt;
&lt;/dependency&gt;
&lt;dependency&gt;
&lt;groupId&gt;software.constructs&lt;/groupId&gt;
&lt;artifactId&gt;constructs&lt;/artifactId&gt;
&lt;version&gt;${constructs.version}&lt;/version&gt;
&lt;/dependency&gt;
&lt;dependency&gt;
&lt;groupId&gt;org.junit.jupiter&lt;/groupId&gt;
&lt;artifactId&gt;junit-jupiter&lt;/artifactId&gt;
&lt;version&gt;${junit.version}&lt;/version&gt;
&lt;scope&gt;test&lt;/scope&gt;
&lt;/dependency&gt;
&lt;dependency&gt;
&lt;groupId&gt;software.amazon.awscdk&lt;/groupId&gt;
&lt;artifactId&gt;apigatewayv2-alpha&lt;/artifactId&gt;
&lt;version&gt;${cdk.version}-alpha.0&lt;/version&gt;
&lt;/dependency&gt;
&lt;dependency&gt;
&lt;groupId&gt;software.amazon.awscdk&lt;/groupId&gt;
&lt;artifactId&gt;apigatewayv2-integrations-alpha&lt;/artifactId&gt;
&lt;version&gt;${cdk.version}-alpha.0&lt;/version&gt;
&lt;/dependency&gt;
&lt;/dependencies&gt;
&lt;/project&gt;`
```
10. Make sure you’re in the `hello-world` directory and deploy your application.
```
`cdk deploy`
```
11. Get the URL of the deployed application:
```
`aws cloudformation describe-stacks --stack-name HelloWorldStack --query 'Stacks[0].Outputs[?OutputKey==`HttpApi`].OutputValue' --output text`
```
12. Invoke the API endpoint:
```
`curl -X GET `&lt;&lt;URL\_FROM\_PREVIOUS\_STEP&gt;&gt;``
```
If successful, you'll see this response:
```
{"message":"hello world"}
```
13. To get the traces for the function, run [sam traces](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-cli-command-reference-sam-traces.html).
```
`sam traces`
```
The trace output looks like this:
```
New XRay Service Graph
Start time: 2025-02-03 14:59:50+00:00
End time: 2025-02-03 14:59:50+00:00
Reference Id: 0 - (Root) AWS::Lambda - sam-app-HelloWorldFunction-YBg8yfYtOc9j - Edges: [1]
Summary\_statistics:
- total requests: 1
- ok count(2XX): 1
- error count(4XX): 0
- fault count(5XX): 0
- total response time: 0.924
Reference Id: 1 - AWS::Lambda::Function - sam-app-HelloWorldFunction-YBg8yfYtOc9j - Edges: []
Summary\_statistics:
- total requests: 1
- ok count(2XX): 1
- error count(4XX): 0
- fault count(5XX): 0
- total response time: 0.016
Reference Id: 2 - client - sam-app-HelloWorldFunction-YBg8yfYtOc9j - Edges: [0]
Summary\_statistics:
- total requests: 0
- ok count(2XX): 0
- error count(4XX): 0
- fault count(5XX): 0
- total response time: 0
XRay Event [revision 1] at (2025-02-03T14:59:50.204000) with id (1-63dd2166-434a12c22e1307ff2114f299) and duration (0.924s)
- 0.924s - sam-app-HelloWorldFunction-YBg8yfYtOc9j [HTTP: 200]
- 0.016s - sam-app-HelloWorldFunction-YBg8yfYtOc9j
- 0.739s - Initialization
- 0.016s - Invocation
- 0.013s - ## lambda\_handler
- 0.000s - ## app.hello
- 0.000s - Overhead
```
14. This is a public API endpoint that is accessible over the internet. We recommend that you delete the endpoint after testing.
```
`cdk destroy`
```
## Using ADOT to instrument your Java functions
ADOT provides fully managed Lambda [layers](./chapter-layers.html) that package
everything you need to collect telemetry data using the OTel SDK. By consuming this layer, you can instrument your
Lambda functions without having to modify any function code. You can also configure your layer to do custom
initialization of OTel. For more information, see [Custom configuration for the ADOT Collector on Lambda](https://aws-otel.github.io/docs/getting-started/lambda#custom-configuration-for-the-adot-collector-on-lambda) in the ADOT documentation.
For Java runtimes, you can choose between two layers to consume:
* **AWS managed Lambda layer for ADOT Java (Auto-instrumentation Agent)**
– This layer automatically transforms your function code at startup to collect tracing data. For
detailed instructions on how to consume this layer together with the ADOT Java agent, see [AWS Distro for
OpenTelemetry Lambda Support for Java (Auto-instrumentation Agent)](https://aws-otel.github.io/docs/getting-started/lambda/lambda-java-auto-instr) in the ADOT
documentation.
* **AWS managed Lambda layer for ADOT Java** – This layer also
provides built-in instrumentation for Lambda functions, but it requires a few manual code changes to initialize
the OTel SDK. For detailed instructions on how to consume this layer, see [AWS Distro for OpenTelemetry
Lambda Support for Java](https://aws-otel.github.io/docs/getting-started/lambda/lambda-java) in the ADOT documentation.
## Using the X-Ray SDK to instrument your Java functions
To record data about calls that your function makes to other resources and services in your application, you
can add the X-Ray SDK for Java to your build configuration. The following example shows a Gradle build
configuration that includes the libraries that activate automatic instrumentation of AWS SDK for Java 2.x clients.
###### Example [build.gradle](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/s3-java/build.gradle) – Tracing
dependencies
```
`dependencies {
implementation platform('software.amazon.awssdk:bom:2.16.1')
`implementation platform('com.amazonaws:aws-xray-recorder-sdk-bom:2.11.0')`
...
`implementation 'com.amazonaws:aws-xray-recorder-sdk-core'
implementation 'com.amazonaws:aws-xray-recorder-sdk-aws-sdk'
implementation 'com.amazonaws:aws-xray-recorder-sdk-aws-sdk-v2-instrumentor'`
...
}`
```
After you add the correct dependencies and make the necessary code changes, activate tracing in your
function's configuration via the Lambda console or the API.
## Activating tracing with the Lambda console
To toggle active tracing on your Lambda function with the console, follow these steps:
###### To turn on active tracing
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Choose **Configuration** and then choose **Monitoring and operations tools**.
4. Under **Additional monitoring tools**, choose **Edit**.
5. Under **CloudWatch Application Signals and AWS X-Ray**, choose **Enable** for **Lambda service traces**.
6. Choose **Save**.
## Activating tracing with the Lambda API
Configure tracing on your Lambda function with the AWS CLI or AWS SDK, use the following API operations:
* [UpdateFunctionConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionConfiguration.html)
* [GetFunctionConfiguration](https://docs.aws.amazon.com/lambda/latest/api/API_GetFunctionConfiguration.html)
* [CreateFunction](https://docs.aws.amazon.com/lambda/latest/api/API_CreateFunction.html)
The following example AWS CLI command enables active tracing on a function named
**my-function**.
```
``aws lambda update-function-configuration --function-name my-function \\
--tracing-config Mode=Active``
```
Tracing mode is part of the version-specific configuration when you publish a version of your function.
You can't change the tracing mode on a published version.
## Activating tracing with CloudFormation
To activate tracing on an `AWS::Lambda::Function` resource in an CloudFormation template, use the
`TracingConfig` property.
###### Example [function-inline.yml](https://github.com/awsdocs/aws-lambda-developer-guide/blob/master/templates/function-inline.yml) –
Tracing configuration
```
`Resources:
function:
Type: [AWS::Lambda::Function](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html)
Properties:
`TracingConfig:
Mode: Active`
...`
```
For an AWS Serverless Application Model (AWS SAM) `AWS::Serverless::Function` resource, use the `Tracing`
property.
###### Example [template.yml](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/blank-nodejs/template.yml) – Tracing
configuration
```
`Resources:
function:
Type: [AWS::Serverless::Function](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-function.html)
Properties:
`Tracing: Active`
...`
```
## Interpreting an X-Ray trace
Your function needs permission to upload trace data to X-Ray. When you activate tracing in the Lambda
console, Lambda adds the required permissions to your function's [execution role](./lambda-intro-execution-role.html). Otherwise, add the [AWSXRayDaemonWriteAccess](https://console.aws.amazon.com/iam/home#/policies/arn:aws:iam::aws:policy/AWSXRayDaemonWriteAccess) policy to the execution role.
After you've configured active tracing, you can observe specific requests
through your application. The [
X-Ray service graph](https://docs.aws.amazon.com/xray/latest/devguide/aws-xray.html#xray-concepts-servicegraph) shows information about your application and all its components. The following example
shows an application with two functions.
The primary function processes events and sometimes returns errors. The second function at the top processes errors that appear
in the first's log group and uses the AWS SDK to call X-Ray, Amazon Simple Storage Service (Amazon S3), and Amazon CloudWatch Logs.
![A diagram that shows two separate applications and their respective service maps in X-Ray](https://docs.aws.amazon.com/images/lambda/latest/dg/images/sample-errorprocessor-servicemap.png)
X-Ray doesn't trace all requests to your application. X-Ray applies a sampling algorithm
to ensure that tracing is efficient, while still providing a representative sample of all requests. The sampling rate is
1 request per second and 5 percent of additional requests. You can't configure the X-Ray sampling rate for your functions.
In X-Ray, a *trace* records information about a request that is processed by one or more
*services*. Lambda records 2 segments per trace, which creates
two nodes on the service graph. The following image highlights these two nodes:
![An X-Ray service map with a single function.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/xray-servicemap-function.png)
The first node on the left represents the Lambda service, which receives the invocation request. The second
node represents your specific Lambda function. The following example shows a trace with these two segments. Both
are named **my-function**, but one has an origin of `AWS::Lambda` and the other has
an origin of `AWS::Lambda::Function`. If the `AWS::Lambda` segment shows an error, the Lambda service had an issue. If the `AWS::Lambda::Function` segment shows an error, your function had an issue.
![An X-Ray trace that shows latency across each subsegment of a specific Lambda invocation.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/V2_sandbox_images/my-function-2-v1.png)
This example expands the `AWS::Lambda::Function` segment to show its three subsegments.
###### Note
AWS is currently implementing changes to the Lambda service. Due to these changes, you may see minor differences between the structure and content
of system log messages and trace segments emitted by different Lambda functions in your AWS account.
The example trace shown here illustrates the old-style function segment. The differences between the old- and new-style segments are described in the following paragraphs.
These changes will be implemented during the coming weeks, and all functions in all
AWS Regions except the China and GovCloud regions will transition to use the new-format log messages and trace segments.
The old-style function segment contains the following subsegments:
* **Initialization** – Represents time spent loading your function and
running [initialization code](./foundation-progmodel.html). This subsegment
only appears for the first event that each instance of your function processes.
* **Invocation** – Represents the time spent running your handler code.
* **Overhead** – Represents the time the Lambda runtime spends preparing
to handle the next event.
The new-style function segment doesn't contain an `Invocation` subsegment. Instead,
customer subsegments are attached directly to the function segment. For more information about the structure of the
old- and new-style function segments, see [Understanding X-Ray traces](./services-xray.html#services-xray-traces).
###### Note
[Lambda SnapStart](./snapstart.html) functions also include a `Restore` subsegment. The `Restore` subsegment shows the time it takes for Lambda to restore a snapshot, load the runtime, and run any after-restore [ runtime hooks](./snapstart-runtime-hooks.html). The process of restoring snapshots can include time spent on activities outside the MicroVM. This time is reported in the `Restore` subsegment. You aren't charged for the time spent outside the microVM to restore a snapshot.
You can also instrument HTTP clients, record SQL queries, and create custom subsegments with annotations and
metadata. For more information, see [AWS X-Ray SDK for Java](https://docs.aws.amazon.com/xray/latest/devguide/xray-sdk-java.html) in the *AWS X-Ray Developer Guide*.
###### Pricing
You can use X-Ray tracing for free each month up to a certain limit as part of the AWS Free Tier. Beyond that threshold, X-Ray charges for trace storage and
retrieval. For more information, see [AWS X-Ray pricing](https://aws.amazon.com/xray/pricing/).
## Storing runtime dependencies in a layer (X-Ray SDK)
If you use the X-Ray SDK to instrument AWS SDK clients your function code, your deployment package can become
quite large. To avoid uploading runtime dependencies every time you update your function code, package the X-Ray SDK in a
[Lambda layer](./chapter-layers.html).
The following example shows an `AWS::Serverless::LayerVersion` resource that stores the AWS SDK for Java
and X-Ray SDK for Java.
###### Example [template.yml](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/blank-java/template.yml) – Dependencies
layer
```
`Resources:
function:
Type: [AWS::Serverless::Function](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-function.html)
Properties:
CodeUri: build/distributions/blank-java.zip
Tracing: Active
`Layers:
- !Ref libs`
...
`libs:
Type: [AWS::Serverless::LayerVersion](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-layerversion.html)
Properties:
LayerName: blank-java-lib
Description: Dependencies for the blank-java sample app.
ContentUri: build/blank-java-lib.zip
CompatibleRuntimes:
- java25``
```
With this configuration, you update the library layer only if you change your runtime dependencies.
Since the function deployment package contains only your code, this can help reduce upload times.
Creating a layer for dependencies requires build configuration changes to generate the layer archive prior to
deployment. For a working example, see the [java-basic](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/java-basic) sample
application on GitHub.
## X-Ray tracing in sample applications (X-Ray SDK)
The GitHub repository for this guide includes sample applications that demonstrate the use of X-Ray tracing.
Each sample application includes scripts for easy deployment and cleanup, an AWS SAM template, and supporting
resources.
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
All of the sample applications have active tracing enabled for Lambda functions. For example, the
`s3-java` application shows automatic instrumentation of AWS SDK for Java 2.x clients, segment
management for tests, custom subsegments, and the use of Lambda layers to store runtime dependencies.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Logging
Sample apps
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.