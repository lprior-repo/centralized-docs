---
url: https://docs.aws.amazon.com/lambda/latest/dg/java-handler.html
title: Define Lambda function handler in Java
word_count: 3325
filtered: true
elements_removed: 0
density_score: 0.82
---

Define Lambda function handler in Java - AWS Lambda
Define Lambda function handler in Java - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#java-handler)
[Setting up your Java handler project](#java-handler-setup)[Example Java Lambda function code](#java-example-code)[Valid class definitions for Java handlers](#java-handler-signatures)[Handler naming conventions](#java-example-naming)[Defining and accessing the input event object](#java-handler-input)[Accessing and using the Lambda context object](#java-example-context)[Using the AWS SDK for Java v2 in your handler](#java-example-sdk-usage)[Accessing environment variables](#java-example-envvars)[Using global state](#java-handler-state)[Code best practices for Java Lambda functions](#java-best-practices)
# Define Lambda function handler in Java
The Lambda function *handler* is the method in your function code that processes events. When your function is
invoked, Lambda runs the handler method. Your function runs until the handler returns a response, exits, or times out.
This page describes how to work with Lambda function handlers in Java, including options for project
setup, naming conventions, and best practices. This page also includes an example of a Java Lambda
function that takes in information about an order, produces a text file receipt, and puts this file
in an Amazon Simple Storage Service (Amazon S3) bucket. For information about how to deploy your function after writing it, see
[Deploy Java Lambda functions with .zip or JAR file archives](./java-package.html) or [Deploy Java Lambda functions with container images](./java-image.html).
###### Sections
* [Setting up your Java handler project](#java-handler-setup)
* [Example Java Lambda function code](#java-example-code)
* [Valid class definitions for Java handlers](#java-handler-signatures)
* [Handler naming conventions](#java-example-naming)
* [Defining and accessing the input event object](#java-handler-input)
* [Accessing and using the Lambda context object](#java-example-context)
* [Using the AWS SDK for Java v2 in your handler](#java-example-sdk-usage)
* [Accessing environment variables](#java-example-envvars)
* [Using global state](#java-handler-state)
* [Code best practices for Java Lambda functions](#java-best-practices)
## Setting up your Java handler project
When working with Lambda functions in Java, the process involves writing your code, compiling it,
and deploying the compiled artifacts to Lambda. You can initialize a Java Lambda project in various
ways. For instance, you can use tools like the
[Maven
Archetype for Lambda functions](https://github.com/aws/aws-sdk-java-v2/tree/master/archetypes/archetype-lambda), the AWS SAM CLI
[
sam init command](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-cli-command-reference-sam-init.html), or even a standard Java project setup in your preferred IDE, such as
IntelliJ IDEA or Visual Studio Code. Alternatively, you can create the required file structure
manually.
A typical Java Lambda function project follows this general structure:
```
`/project-root
└ src
└ main
└ java
└ example
└ OrderHandler.java (contains main handler)
└ &lt;&lt;other\_supporting\_classes&gt;&gt;
└ build.gradle OR pom.xml`
```
You can use either Maven or Gradle to build your project and manage dependencies.
The main handler logic for your function resides in a Java file under the
`src/main/java/example` directory. In the example on this page, we name this file
`OrderHandler.java`. Apart from this file, you can include additional Java classes as needed.
When deploying your function to Lambda, make sure you specify the Java class that contains the main
handler method that Lambda should invoke during an invocation.
## Example Java Lambda function code
The following example Java 21 Lambda function code takes in information about an order,
produces a text file receipt, and puts this file in an Amazon S3 bucket.
###### Example `OrderHandler.java` Lambda function
```
`package example;
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import software.amazon.awssdk.core.sync.RequestBody;
import software.amazon.awssdk.services.s3.S3Client;
import software.amazon.awssdk.services.s3.model.PutObjectRequest;
import software.amazon.awssdk.services.s3.model.S3Exception;
import java.nio.charset.StandardCharsets;
/\*\*
\* Lambda handler for processing orders and storing receipts in S3.
\*/
public class OrderHandler implements RequestHandler&lt;&lt;OrderHandler.Order, String&gt;&gt; {
private static final S3Client S3\_CLIENT = S3Client.builder().build();
/\*\*
\* Record to model the input event.
\*/
public record Order(String orderId, double amount, String item) {}
@Override
public String handleRequest(Order event, Context context) {
try {
// Access environment variables
String bucketName = System.getenv("RECEIPT\_BUCKET");
if (bucketName == null || bucketName.isEmpty()) {
throw new IllegalArgumentException("RECEIPT\_BUCKET environment variable is not set");
}
// Create the receipt content and key destination
String receiptContent = String.format("OrderID: %s\\nAmount: $%.2f\\nItem: %s",
event.orderId(), event.amount(), event.item());
String key = "receipts/" + event.orderId() + ".txt";
// Upload the receipt to S3
uploadReceiptToS3(bucketName, key, receiptContent);
context.getLogger().log("Successfully processed order " + event.orderId() +
" and stored receipt in S3 bucket " + bucketName);
return "Success";
} catch (Exception e) {
context.getLogger().log("Failed to process order: " + e.getMessage());
throw new RuntimeException(e);
}
}
private void uploadReceiptToS3(String bucketName, String key, String receiptContent) {
try {
PutObjectRequest putObjectRequest = PutObjectRequest.builder()
.bucket(bucketName)
.key(key)
.build();
// Convert the receipt content to bytes and upload to S3
S3\_CLIENT.putObject(putObjectRequest, RequestBody.fromBytes(receiptContent.getBytes(StandardCharsets.UTF\_8)));
} catch (S3Exception e) {
throw new RuntimeException("Failed to upload receipt to S3: " + e.awsErrorDetails().errorMessage(), e);
}
}
}
`
```
This `OrderHandler.java` file contains the following sections of code:
* `package example`: In Java, this can be anything, but it must match the
directory structure of your project. Here, we use `package example` because
the directory structure is `src/main/java/example`.
* `import` statements: Use these to import Java classes that your Lambda
function requires.
* `public class OrderHandler ...`: This defines your Java class, and must
be a [valid class definition](#java-handler-signatures).
* `private static final S3Client S3\_CLIENT ...`: This initializes an S3
client outside of any of the class’s methods. This causes Lambda to run this code during
the [initialization phase](./lambda-runtime-environment.html#runtimes-lifecycle-ib).
* `public record Order ...`: Define the shape of the expected input event
in this custom Java [record](https://openjdk.org/jeps/395).
* `public String handleRequest(Order event, Context context)`: This is the
**main handler method**, which contains your main
application logic.
* `private void uploadReceiptToS3(...) {}`: This is a helper method that's
referenced by the main `handleRequest` handler method.
The following `build.gradle` or `pom.xml` file accompanies this
function.
build.gradle
```
`plugins {
id 'java'
}
repositories {
mavenCentral()
}
dependencies {
implementation 'com.amazonaws:aws-lambda-java-core:1.2.3'
implementation 'software.amazon.awssdk:s3:2.28.29'
implementation 'org.slf4j:slf4j-nop:2.0.16'
}
task buildZip(type: Zip) {
from compileJava
from processResources
into('lib') {
from configurations.runtimeClasspath
}
}
java {
sourceCompatibility = JavaVersion.VERSION\_21
targetCompatibility = JavaVersion.VERSION\_21
}
build.dependsOn buildZip`
```
pom.xml
```
`&lt;&lt;project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/maven-v4\_0\_0.xsd"&gt;&gt;
&lt;&lt;modelVersion&gt;&gt;4.0.0&lt;&lt;/modelVersion&gt;&gt;
&lt;&lt;groupId&gt;&gt;com.example&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;example-java&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;packaging&gt;&gt;jar&lt;&lt;/packaging&gt;&gt;
&lt;&lt;version&gt;&gt;1.0-SNAPSHOT&lt;&lt;/version&gt;&gt;
&lt;&lt;name&gt;&gt;example-java-function&lt;&lt;/name&gt;&gt;
&lt;&lt;properties&gt;&gt;
&lt;&lt;project.build.sourceEncoding&gt;&gt;UTF-8&lt;&lt;/project.build.sourceEncoding&gt;&gt;
&lt;&lt;maven.compiler.source&gt;&gt;21&lt;&lt;/maven.compiler.source&gt;&gt;
&lt;&lt;maven.compiler.target&gt;&gt;21&lt;&lt;/maven.compiler.target&gt;&gt;
&lt;&lt;/properties&gt;&gt;
&lt;&lt;dependencies&gt;&gt;
&lt;&lt;dependency&gt;&gt;
&lt;&lt;groupId&gt;&gt;com.amazonaws&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;aws-lambda-java-core&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;1.2.3&lt;&lt;/version&gt;&gt;
&lt;&lt;/dependency&gt;&gt;
&lt;&lt;dependency&gt;&gt;
&lt;&lt;groupId&gt;&gt;software.amazon.awssdk&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;s3&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;2.28.29&lt;&lt;/version&gt;&gt;
&lt;&lt;/dependency&gt;&gt;
&lt;&lt;dependency&gt;&gt;
&lt;&lt;groupId&gt;&gt;org.slf4j&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;slf4j-nop&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;2.0.16&lt;&lt;/version&gt;&gt;
&lt;&lt;/dependency&gt;&gt;
&lt;&lt;/dependencies&gt;&gt;
&lt;&lt;build&gt;&gt;
&lt;&lt;plugins&gt;&gt;
&lt;&lt;plugin&gt;&gt;
&lt;&lt;artifactId&gt;&gt;maven-surefire-plugin&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;3.5.2&lt;&lt;/version&gt;&gt;
&lt;&lt;/plugin&gt;&gt;
&lt;&lt;plugin&gt;&gt;
&lt;&lt;groupId&gt;&gt;org.apache.maven.plugins&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;maven-shade-plugin&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;3.4.1&lt;&lt;/version&gt;&gt;
&lt;&lt;configuration&gt;&gt;
&lt;&lt;createDependencyReducedPom&gt;&gt;false&lt;&lt;/createDependencyReducedPom&gt;&gt;
&lt;&lt;filters&gt;&gt;
&lt;&lt;filter&gt;&gt;
&lt;&lt;artifact&gt;&gt;\*:\*&lt;&lt;/artifact&gt;&gt;
&lt;&lt;excludes&gt;&gt;
&lt;&lt;exclude&gt;&gt;META-INF/\*&lt;&lt;/exclude&gt;&gt;
&lt;&lt;exclude&gt;&gt;META-INF/versions/\*\*&lt;&lt;/exclude&gt;&gt;
&lt;&lt;/excludes&gt;&gt;
&lt;&lt;/filter&gt;&gt;
&lt;&lt;/filters&gt;&gt;
&lt;&lt;/configuration&gt;&gt;
&lt;&lt;executions&gt;&gt;
&lt;&lt;execution&gt;&gt;
&lt;&lt;phase&gt;&gt;package&lt;&lt;/phase&gt;&gt;
&lt;&lt;goals&gt;&gt;
&lt;&lt;goal&gt;&gt;shade&lt;&lt;/goal&gt;&gt;
&lt;&lt;/goals&gt;&gt;
&lt;&lt;/execution&gt;&gt;
&lt;&lt;/executions&gt;&gt;
&lt;&lt;/plugin&gt;&gt;
&lt;&lt;plugin&gt;&gt;
&lt;&lt;groupId&gt;&gt;org.apache.maven.plugins&lt;&lt;/groupId&gt;&gt;
&lt;&lt;artifactId&gt;&gt;maven-compiler-plugin&lt;&lt;/artifactId&gt;&gt;
&lt;&lt;version&gt;&gt;3.13.0&lt;&lt;/version&gt;&gt;
&lt;&lt;configuration&gt;&gt;
&lt;&lt;release&gt;&gt;21&lt;&lt;/release&gt;&gt;
&lt;&lt;/configuration&gt;&gt;
&lt;&lt;/plugin&gt;&gt;
&lt;&lt;/plugins&gt;&gt;
&lt;&lt;/build&gt;&gt;
&lt;&lt;/project&gt;&gt;`
```
For this function to work properly, its [
execution role](./lambda-intro-execution-role.html) must allow the `s3:PutObject` action. Also, ensure that you
define the `RECEIPT\_BUCKET` environment variable. After a successful invocation,
the Amazon S3 bucket should contain a receipt file.
###### Note
This function may require additional configuration settings to run successfully without
timing out. We recommend configuring 256 MB of memory, and a 10 second timeout. The first
invocation may take extra time due to a [cold start](./lambda-runtime-environment.html#cold-start-latency).
Subsequent invocations should run much faster due to reuse of the execution environment.
## Valid class definitions for Java handlers
To define your class, the [
aws-lambda-java-core](https://github.com/aws/aws-lambda-java-libs/tree/master/aws-lambda-java-core) library defines two interfaces for handler methods. Use the
provided interfaces to simplify handler configuration and validate the method signature
at compile time.
* [
com.amazonaws.services.lambda.runtime.RequestHandler](https://github.com/aws/aws-lambda-java-libs/blob/master/aws-lambda-java-core/src/main/java/com/amazonaws/services/lambda/runtime/RequestHandler.java)
* [
com.amazonaws.services.lambda.runtime.RequestStreamHandler](https://github.com/aws/aws-lambda-java-libs/blob/master/aws-lambda-java-core/src/main/java/com/amazonaws/services/lambda/runtime/RequestStreamHandler.java)
The `RequestHandler` interface is a generic type that takes two parameters: the
input type and the output type. Both types must be objects. In this example, our
`OrderHandler` class implements `RequestHandler&lt;OrderHandler.Order, String&gt;`.
The input type is the `Order` record we define within the class, and the output type
is `String`.
```
`public class OrderHandler implements RequestHandler&lt;OrderHandler.Order, String&gt; {
...
}`
```
When you use this interface, the Java runtime deserializes the event into the object with the
input type, and serializes the output into text. Use this interface when the built-in serialization
works with your input and output types.
To use your own serialization, you can implement the `RequestStreamHandler` interface.
With this interface, Lambda passes your handler an input stream and output stream. The handler reads
bytes from the input stream, writes to the output stream, and returns void. For an example of this
using the Java 21 runtime, see
[
HandlerStream.java](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/java-basic/src/main/java/example/HandlerStream.java).
If you’re working only with basic and generic types (i.e. `String`, `Integer`,
`List`, or `Map`) in your Java function , you don’t need to implement an interface.
For example, if your function takes in a `Map&lt;String, String&gt;` input and returns a
`String`, your class definition and handler signature may look like the following:
```
`public class ExampleHandler {
public String handleRequest(Map&lt;String, String&gt; input, Context context) {
...
}
}`
```
In addition, when you don’t implement an interface, the
[context](./java-context.html) object is optional. For example, your class definition
and handler signature may look like the following:
```
`public class NoContextHandler {
public String handleRequest(Map&lt;String, String&gt; input) {
...
}
}`
```
## Handler naming conventions
For Lambda functions in Java, if you are implementing either the `RequestHandler` or
`RequestStreamHandler` interface, your main handler method must be named
`handleRequest`. Also, include the `@Override` tag above your
`handleRequest` method. When you deploy your function to Lambda, specify the main handler
in your function’s configuration in the following format:
* `&lt;package&gt;`.`&lt;Class&gt;` –
For example, `example.OrderHandler`.
For Lambda functions in Java that don’t implement the `RequestHandler` or
`RequestStreamHandler` interface, you can use any name for the handler. When you deploy
your function to Lambda, specify the main handler in your function’s configuration in the following
format:
* `&lt;package&gt;`.`&lt;Class&gt;`::`&lt;&lt;handler\_method\_name&gt;&gt;` –
For example, `example.Handler::mainHandler`.
## Defining and accessing the input event object
JSON is the most common and standard input format for Lambda functions. In this example, the
function expects an input similar to the following:
```
`{
"orderId": "12345",
"amount": 199.99,
"item": "Wireless Headphones"
}`
```
When working with Lambda functions in Java 17 or newer, you can define the shape of the expected
input event as a Java record. In this example, we define a record within the `OrderHandler`
class to represent an `Order` object:
```
`public record Order(String orderId, double amount, String item) {}`
```
This record matches the expected input shape. After you define your record, you can write a
handler signature that takes in a JSON input that conforms to the record definition. The Java runtime
automatically deserializes this JSON into a Java object. You can then access the fields of the object.
For example, `event.orderId` retrieves the value of `orderId` from the original
input.
###### Note
Java records are a feature of Java 17 runtimes and newer only. In all Java runtimes, you can
use a class to represent event data. In such cases, you can use a library like
[jackson](https://github.com/FasterXML/jackson) to deserialize JSON inputs.
### Other input event types
There are many possible input events for Lambda functions in Java:
* `Integer`, `Long`, `Double`, etc. – The event is a
number with no additional formatting—for example, `3.5`. The Java runtime converts
the value into an object of the specified type.
* `String` – The event is a JSON string, including quotes—for example,
`“My string”`. The runtime converts the value into a `String` object
without quotes.
* `List&lt;Integer&gt;`, `List&lt;String&gt;`,
`List&lt;Object&gt;`, etc. – The event is a JSON array. The runtime
deserializes it into an object of the specified type or interface.
* `InputStream` – The event is any JSON type. The runtime passes a
byte stream of the document to the handler without modification. You deserialize the input
and write output to an output stream.
* Library type – For events sent by other AWS services, use the types in the
[
aws-lambda-java-events](https://github.com/aws/aws-lambda-java-libs/tree/main/aws-lambda-java-events) library. For example, if your Lambda function is invoked by
Amazon Simple Queue Service (SQS), use the `SQSEvent` object as the input.
## Accessing and using the Lambda context object
The Lambda [context object](./java-context.html) contains information about the
invocation, function, and execution environment. In this example, the context object is of type
`com.amazonaws.services.lambda.runtime.Context`, and is the second argument of the
main handler function.
```
`public String handleRequest(Order event, Context context) {
...
}`
```
If your class implements either the
[
RequestHandler](https://github.com/aws/aws-lambda-java-libs/blob/master/aws-lambda-java-core/src/main/java/com/amazonaws/services/lambda/runtime/RequestHandler.java) or
[
RequestStreamHandler](https://github.com/aws/aws-lambda-java-libs/blob/master/aws-lambda-java-core/src/main/java/com/amazonaws/services/lambda/runtime/RequestStreamHandler.java) interface, the context object is a required argument. Otherwise, the
context object is optional. For more information about valid accepted handler signatures, see
[Valid class definitions for Java handlers](#java-handler-signatures).
If you make calls to other services using the AWS SDK, the context object is required in
a few key areas. For example, to produce function logs for Amazon CloudWatch, you can use the
`context.getLogger()` method to get a `LambdaLogger` object for logging.
In this example, we can use the logger to log an error message if processing fails for any reason:
```
`context.getLogger().log("Failed to process order: " + e.getMessage());`
```
Outside of logging, you can also use the context object for function monitoring. For more
information about the context object, see [Using the Lambda context object to retrieve Java function information](./java-context.html).
## Using the AWS SDK for Java v2 in your handler
Often, you’ll use Lambda functions to interact with or make updates to other AWS resources.
The simplest way to interface with these resources is to use the AWS SDK for Java v2.
###### Note
The AWS SDK for Java (v1) is in maintenance mode, and will reach end-of-support on
December 31, 2025. We recommend that you use only the AWS SDK for Java v2 going forward.
To add SDK dependencies to your function, add them in your `build.gradle` for
Gradle or `pom.xml` file for Maven. We recommend only adding the libraries that you
need for your function. In the example code earlier, we used the
`software.amazon.awssdk.services.s3` library. In Gradle, you can add this dependency
by adding the following line in the dependencies section of your `build.gradle`:
```
`implementation 'software.amazon.awssdk:s3:2.28.29'`
```
In Maven, add the following lines in the `&lt;dependencies&gt;` section of your
`pom.xml`:
```
` &lt;dependency&gt;
&lt;groupId&gt;software.amazon.awssdk&lt;/groupId&gt;
&lt;artifactId&gt;s3&lt;/artifactId&gt;
&lt;version&gt;2.28.29&lt;/version&gt;
&lt;/dependency&gt;`
```
###### Note
This may not be the most recent version of the SDK. Choose the appropriate version of
the SDK for your application.
Then, import the dependencies directly in your Java class:
```
`import software.amazon.awssdk.services.s3.S3Client;
import software.amazon.awssdk.services.s3.model.PutObjectRequest;
import software.amazon.awssdk.services.s3.model.S3Exception;`
```
The example code then initializes an Amazon S3 client as follows:
```
`private static final S3Client S3\_CLIENT = S3Client.builder().build();`
```
In this example, we initialized our Amazon S3 client outside of the main handler function to
avoid having to initialize it every time we invoke our function. After you initialize your SDK
client, you can then use it to interact with other AWS services. The example code calls the
Amazon S3 `PutObject` API as follows:
```
`PutObjectRequest putObjectRequest = PutObjectRequest.builder()
.bucket(bucketName)
.key(key)
.build();
// Convert the receipt content to bytes and upload to S3
S3\_CLIENT.putObject(putObjectRequest, RequestBody.fromBytes(receiptContent.getBytes(StandardCharsets.UTF\_8)));`
```
## Accessing environment variables
In your handler code, you can reference any [
environment variables](./configuration-envvars.html) by using the `System.getenv()` method. In this example,
we reference the defined `RECEIPT\_BUCKET` environment variable using the following
line of code:
```
`String bucketName = System.getenv("RECEIPT\_BUCKET");
if (bucketName == null || bucketName.isEmpty()) {
throw new IllegalArgumentException("RECEIPT\_BUCKET environment variable is not set");
}`
```
## Using global state
Lambda runs your static code and the class constructor during the
[initialization phase](./lambda-runtime-environment.html#runtimes-lifecycle-ib) before invoking your function for the first time. Resources created during initialization stay in memory between invocations, so you can avoid having to create them every time you invoke your function.
In the example code, the S3 client initialization code is outside the main handler method. The runtime initializes the client before the function handles its first event, and the client remains available for reuse across all invocations.
## Code best practices for Java Lambda functions
Adhere to the guidelines in the following list to use best coding practices when building your Lambda functions:
* **Separate the Lambda handler from your core logic.** This allows you to make
a more unit-testable function.
* **Control the dependencies in your function's deployment package. ** The
AWS Lambda execution environment contains a number of libraries.
To enable the latest set of features and security updates, Lambda will periodically update these libraries.
These updates may introduce subtle changes to the behavior of your Lambda function. To have full control of the
dependencies your function uses, package all of your dependencies with your deployment package.
* **Minimize the complexity of your dependencies.** Prefer simpler frameworks
that load quickly on [execution environment](./lambda-runtime-environment.html) startup. For example, prefer
simpler Java dependency injection (IoC) frameworks like [Dagger](https://google.github.io/dagger/) or
[Guice](https://github.com/google/guice), over more complex ones like
[Spring Framework](https://github.com/spring-projects/spring-framework).
* **Minimize your deployment package size to its runtime necessities. ** This
will reduce the amount of time that it takes for your deployment package to be downloaded and unpacked ahead
of invocation. For functions authored in Java, avoid uploading the entire AWS SDK library as part
of your deployment package. Instead, selectively depend on the modules which pick up components of the SDK you
need (e.g. DynamoDB, Amazon S3 SDK modules and [Lambda core
libraries](https://github.com/aws/aws-lambda-java-libs)).
**Take advantage of execution environment reuse to improve the performance of your
function.** Initialize SDK clients and database connections outside of the function handler, and
cache static assets locally in the `/tmp` directory. Subsequent invocations processed by
the same instance of your function can reuse these resources. This saves cost by reducing function run time.
To avoid potential data leaks across invocations, don’t use the execution environment to store user data,
events, or other information with security implications. If your function relies on a mutable state that can’t
be stored in memory within the handler, consider creating a separate function or separate versions of a
function for each user.
**Use a keep-alive directive to maintain persistent connections.** Lambda purges idle connections over time.
Attempting to reuse an idle connection when invoking a function will result in a connection error. To maintain your persistent connection, use the
keep-alive directive associated with your runtime.
For an example, see [Reusing Connections with Keep-Alive in Node.js](https://docs.aws.amazon.com/sdk-for-javascript/v3/developer-guide/node-reusing-connections.html).
**Use [environment variables](./configuration-envvars.html) to pass operational parameters to your function.** For example, if
you are writing to an Amazon S3 bucket, instead of hard-coding the bucket name you are writing to, configure the
bucket name as an environment variable.
**Avoid using recursive invocations** in your Lambda function, where the function
invokes itself or initiates a process that may invoke the function again. This could lead to unintended volume of
function invocations and escalated costs. If you see an unintended volume of invocations, set the function reserved concurrency
to `0` immediately to throttle all invocations to the function, while you update the
code.
**Do not use non-documented, non-public APIs** in your Lambda function code.
For AWS Lambda managed runtimes, Lambda periodically applies security and functional updates to Lambda's internal APIs.
These internal API updates may be backwards-incompatible, leading to unintended consequences such as invocation
failures if your function has a dependency on these non-public APIs. See
[the API reference](https://docs.aws.amazon.com/lambda/latest/api/welcome.html) for a list of
publicly available APIs.
**Write idempotent code.** Writing idempotent code for your functions ensures that
duplicate events are handled the same way. Your code should properly validate events and gracefully handle
duplicate events. For more information, see
[How do I make
my Lambda function idempotent?](https://aws.amazon.com/premiumsupport/knowledge-center/lambda-function-idempotent/).
* **Avoid using the Java DNS cache.** Lambda functions already cache DNS responses. If you use another DNS cache, then you might experience connection timeouts.
The `java.util.logging.Logger` class can indirectly enable the JVM DNS cache.
To override the default settings, set [networkaddress.cache.ttl](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/net/InetAddress.html#inetaddress-caching-heading) to 0 before initializing `logger`.
Example:
```
`public class MyHandler {
// first set TTL property
static{
java.security.Security.setProperty("networkaddress.cache.ttl" , "0");
}
// then instantiate logger
var logger = org.apache.logging.log4j.LogManager.getLogger(MyHandler.class);
}`
```
* **Reduce the time it takes Lambda to unpack deployment packages** authored in
Java by putting your dependency `.jar` files in a separate /lib directory. This is faster than
putting all your function’s code in a single jar with a large number of `.class` files. See [Deploy Java Lambda functions with .zip or JAR file archives](./java-package.html) for instructions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Building with Java
Deploy .zip file archives
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.