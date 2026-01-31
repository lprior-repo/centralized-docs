---
doc_id: tutorial/general/docs-tutorial-get-started-cue-java
chunk_id: tutorial/general/docs-tutorial-get-started-cue-java#2-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 521
summary: Create a directory to hold some files, and change into it:. $ mkdir -p cue-java-api-tutorials
---


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
