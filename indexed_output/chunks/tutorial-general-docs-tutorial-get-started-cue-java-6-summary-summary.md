---
doc_id: tutorial/general/docs-tutorial-get-started-cue-java
chunk_id: tutorial/general/docs-tutorial-get-started-cue-java#6-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: Cross-check the value of this important variable:. $ echo \"$CLASSPATH\"
---


3

Cross-check the value of this important variable:

TERMINAL

Copy code
Copied!

$ echo "$CLASSPATH"
/usr/local/share/java/*

Java’s requirements mean that we need the value of the CLASSPATH variable to
end with an asterisk – make sure you see the trailing *.

If this value ends with a filename instead of an asterisk (for example:
/usr/local/share/java/CUE.jar) then you need to fix this.
Repeat the previous step while making sure that you surround the value of the
variable with quotes (') when you export it.
