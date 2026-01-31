---
doc_id: tutorial/general/docs-tutorial-get-started-cue-java
chunk_id: tutorial/general/docs-tutorial-get-started-cue-java#3-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 515
summary: RUN THE PROGRAM. Run the Java program:
---


RUN THE PROGRAM

8

Run the Java program:

TERMINAL

Copy code
Copied!

$ java --enable-native-access=ALL-UNNAMED -cp .:$CLASSPATH GetStarted
a1 equals a2:			false
b1 equals b2:			true
c equals d:				true
A.val equals A.B.val:	true

The Java runtime must be told about a slightly different classpath from the
compiler, through the -cp flag, because it needs to locate both the
cue-api-java JAR and your compiled code. The --enable-native-access flag
avoids a runtime warning that the
Foreign Function & Memory API [https://openjdk.org/jeps/454] is being used by
cue-api-java.

CONCLUSION

Congratulations - you’ve successfully used CUE in a Java program using the
library cue-api-java.

See Related content, below, for tutorials and guides that explain more about
using CUE in Java.

RELATED CONTENT

 * How-to Guide: Building cue-api-java as a JAR file [/docs/howto/build-cue-api-java-jar/]
 * Tutorial: Converting values between Java and CUE [/docs/tutorial/convert-values-java-cue/]
 * Tutorial: Handling errors in the Java API [/docs/tutorial/handle-errors-java-api/]
 * Tutorial: Validating data against a schema in Java [/docs/tutorial/validate-data-schema-java/]
 * java api
   [/search?q=tag:%22java%20api%22]– all pages exploring the CUE Java API

Last modified September 4, 2025 [https://github.com/cue-lang/cuelang.org/commit/c675de963f4124145b48e2681dab7b4aacab71e2]

 * java api [/search?q=tag:%22java%20api%22]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/tutorial/get-started-cue-java/&text=The%20library%20cue-api-java%20provides%20a%20way%20to%20use%20CUE%20from%20Java%20programs.%20This%20tutorial%20helps%20you%20get%20started%20using%20cue-api-java,%20after%20you%20have%20installed%20it%20successfully.%0acue-api-java%20is%20an%20experimental%20technology%20preview.%20This%20means%20that%20it&rsquo;s%20under%20development%20and%20its%20behaviour%20might%20change%20from%20one%20release%20to%20the%20next.%20]

 * Share on Linkedin
