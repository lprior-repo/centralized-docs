---
doc_id: tutorial/general/docs-tutorial-get-started-cue-java
chunk_id: tutorial/general/docs-tutorial-get-started-cue-java#12-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: b1 equals b2:			true. c equals d:				true
---

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
