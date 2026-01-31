---
doc_id: tutorial/general/docs-tutorial-get-started-cue-java
chunk_id: tutorial/general/docs-tutorial-get-started-cue-java#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1040
summary: before continuing with this tutorial. Create a directory to hold some files, and change into it:
---

before continuing with this tutorial.

5

Create a directory to hold some files, and change into it:

TERMINAL

Copy code
Copied!

$ mkdir -p cue-java-api-tutorials
$ cd cue-java-api-tutorials

CREATE A JAVA PROGRAM

6

Place this Java program in the file GetStarted.java:

Copied!
cue-java-api-tutorials/GetStarted.java

Copy code
Copied!

import org.cuelang.cue.*;

public class GetStarted {
    public static void main(String[] args) throws Exception {
        // Begin by creating a context. Every CUE value must come from a
        // context, and every CUE operation must use CUE values that come from
        // the *same* context.
        var ctx = new CueContext();

        // The compile() method loads CUE passed as a string and evaluates it.
        var a1 = ctx.compile("x: true");
        var b1 = ctx.compile("y: 42");

        // The equals() method checks concrete values for equality.
        var a2 = ctx.compile("x: false");
        var b2 = ctx.compile("y: 42");
        System.out.println("a1 equals a2:\t\t\t" + a1.equals(a2));
        System.out.println("b1 equals b2:\t\t\t" + b1.equals(b2));

        // The unify() method unifies CUE values, returning a new CUE value.
        var c1 = ctx.compile("a: int");
        var c2 = ctx.compile("a: 5");
        var c3 = ctx.compile("b: true");
        var c = c1.unify(c2).unify(c3);
        var d = ctx.compile("{ a: 5, b: true }");
        System.out.println("c equals d:\t\t\t\t" + c.equals(d));

        // The lookup() method extracts a CUE value from a struct.
        var s = ctx.compile("""
            A: {
                val: 1
                B: {
                    val: 1
                }
            }
        """);
        var sAval  = s.lookup("A").lookup("val");
        var sABval = s.lookup("A.B.val");
        System.out.println("A.val equals A.B.val:\t" + sAval.equals(sABval));
    }
}

COMPILE THE PROGRAM

7

Compile the Java program:

TERMINAL

Copy code
Copied!

$ javac GetStarted.java

The Java compiler automatically uses the value of the CLASSPATH environment
variable to locate the JAR file containing cue-api-java.

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
