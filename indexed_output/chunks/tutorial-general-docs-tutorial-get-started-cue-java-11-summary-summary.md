---
doc_id: tutorial/general/docs-tutorial-get-started-cue-java
chunk_id: tutorial/general/docs-tutorial-get-started-cue-java#11-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary:         var sABval = s. val equals A
---

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
