---
url: https://docs.aws.amazon.com/lambda/latest/dg/snapstart-runtime-hooks-java.html
title: Lambda SnapStart runtime hooks for Java
word_count: 912
filtered: true
elements_removed: 0
density_score: 0.86
---

Lambda SnapStart runtime hooks for Java - AWS Lambda
Lambda SnapStart runtime hooks for Java - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#snapstart-runtime-hooks-java)
[Registration and execution](#runtime-hooks-registration-java)[Step 1: Update the build configuration](#runtime-hooks-java-update-build)[Step 2: Update the Lambda handler](#runtime-hooks-java-update-handler)
# Lambda SnapStart runtime hooks for Java
You can use runtime hooks to implement code before Lambda creates a snapshot or after Lambda resumes a function
from a snapshot. Runtime hooks are available as part of the open-source Coordinated Restore at Checkpoint (CRaC)
project. CRaC is in development for the [Open Java Development Kit
(OpenJDK)](https://wiki.openjdk.org/display/crac). For an example of how to use CRaC with a reference application, see the [CRaC](https://github.com/CRaC/docs/blob/master/STEP-BY-STEP.md) repository on GitHub. CRaC uses three
main elements:
* `Resource` – An interface with two methods, `beforeCheckpoint()` and
`afterRestore()`. Use these methods to implement the code that you want to run before a snapshot
and after a restore.
* `Context &lt;R extends Resource&gt;` – To receive notifications for checkpoints and
restores, a `Resource` must be registered with a `Context`.
* `Core` – The coordination service, which provides the default global `Context`
via the static method `Core.getGlobalContext()`.
For more information about `Context` and `Resource`, see [Package org.crac](https://javadoc.io/doc/io.github.crac/org-crac/latest/index.html) in the CRaC
documentation.
Use the following steps to implement runtime hooks with the [org.crac package](https://github.com/CRaC/org.crac). The Lambda runtime contains a customized CRaC context implementation that calls your
runtime hooks before checkpointing and after restoring.
## Runtime hook registration and execution
The order that Lambda executes your runtime hooks is determined by the order of registration. Registration order follows the order of import, definition, or execution in your code.
* `beforeCheckpoint()`: Executed in the reverse order of registration
* `afterRestore()`: Executed in the order of registration
Make sure that all registered hooks are properly imported and included in your function's code. If you register runtime hooks in a separate file or module, you must ensure that the module is imported, either directly or as part of a larger package, in your function's handler file. If the file or module is not imported in the function handler, Lambda ignores the runtime hooks.
###### Note
When Lambda creates a snapshot, your initialization code can run for up to 15 minutes. The time limit is 130 seconds or the [configured function timeout](./configuration-timeout.html) (maximum 900 seconds), whichever is higher. Your `beforeCheckpoint()` runtime hooks count towards the initialization code time limit. When Lambda restores a snapshot, the runtime must load and `afterRestore()` runtime hooks must complete within the timeout limit (10 seconds). Otherwise, you'll get a SnapStartTimeoutException.
## Step 1: Update the build configuration
Add the `org.crac` dependency to the build configuration. The following example uses Gradle. For
examples for other build systems, see the [Apache Maven
documentation](https://search.maven.org/artifact/io.github.crac/org-crac/0.1.3/jar).
```
`dependencies {
compile group: 'com.amazonaws', name: 'aws-lambda-java-core', version: '1.2.1'
# Then, add the org.crac dependency:
implementation group: 'org.crac', name: 'crac', version: '1.4.0'
}`
```
## Step 2: Update the Lambda handler
The Lambda function *handler* is the method in your function code that processes events. When your function is
invoked, Lambda runs the handler method. Your function runs until the handler returns a response, exits, or times out.
For more information, see [Define Lambda function handler in Java](./java-handler.html).
The following example handler shows how to run code before checkpointing (`beforeCheckpoint()`) and
after restoring (`afterRestore()`). This handler also registers the `Resource` to the
runtime-managed global `Context`.
###### Note
When Lambda creates a snapshot, your initialization code can run for up to 15 minutes. The time limit is 130 seconds or the [configured function timeout](./configuration-timeout.html) (maximum 900 seconds), whichever is higher. Your `beforeCheckpoint()` runtime hooks count towards the initialization code time limit. When Lambda restores a snapshot, the runtime (JVM) must load and `afterRestore()` runtime hooks must complete within the timeout limit (10 seconds). Otherwise, you'll get a SnapStartTimeoutException.
```
`...
import org.crac.Resource;
import org.crac.Core;
...
public class CRaCDemo implements RequestStreamHandler, Resource {
public CRaCDemo() {
Core.getGlobalContext().register(this);
}
public String handleRequest(String name, Context context) throws IOException {
System.out.println("Handler execution");
return "Hello " + name;
}
@Override
public void `beforeCheckpoint`(org.crac.Context&lt;? extends Resource&gt; context)
throws Exception {
System.out.println("Before checkpoint");
}
@Override
public void `afterRestore`(org.crac.Context&lt;? extends Resource&gt; context)
throws Exception {
System.out.println("After restore");`
```
`Context` maintains only a [`WeakReference`](https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/lang/ref/WeakReference.html)
to the registered object. If a [`Resource`](https://javadoc.io/static/io.github.crac/org-crac/0.1.3/org/crac/Resource.html)
is garbage collected, runtime hooks do not run. Your code must maintain a strong reference to the
`Resource` to guarantee that the runtime hook runs.
Here are two examples of patterns to avoid:
###### Example – Object without a strong reference
```
`Core.getGlobalContext().register( new MyResource() );`
```
###### Example – Objects of anonymous classes
```
`Core.getGlobalContext().register( new Resource() {
@Override
public void afterRestore(Context&lt;? extends Resource&gt; context) throws Exception {
// ...
}
@Override
public void beforeCheckpoint(Context&lt;? extends Resource&gt; context) throws Exception {
// ...
}
} );`
```
Instead, maintain a strong reference. In the following example, the registered resource isn't garbage
collected and runtime hooks run consistently.
###### Example – Object with a strong reference
```
`Resource myResource = new MyResource(); `// This reference must be maintained to prevent the registered resource from being garbage collected`
Core.getGlobalContext().register( myResource );`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Runtime hooks
Python
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.