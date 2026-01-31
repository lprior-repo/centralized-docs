---
doc_id: tutorial/general/docs-tutorial-get-started-cue-java
chunk_id: tutorial/general/docs-tutorial-get-started-cue-java#8-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: $ mkdir -p cue-java-api-tutorials. $ cd cue-java-api-tutorials
---


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
