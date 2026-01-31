---
doc_id: tutorial/general/docs-tutorial-get-started-cue-java
chunk_id: tutorial/general/docs-tutorial-get-started-cue-java#5-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: $ export LD_LIBRARY_PATH=/usr/local/lib/. If you have installed libcue into a different directory than /usr/local/lib/
---

computer:

TERMINAL

Copy code
Copied!

$ export LD_LIBRARY_PATH=/usr/local/lib/

If you have installed libcue into a different directory than /usr/local/lib/
then adapt the command to refer to that directory instead.

2

Tell Java where to find the library cue-api-java on your computer:

TERMINAL

Copy code
Copied!

$ export CLASSPATH='/usr/local/share/java/*'

If you have installed cue-api-java into a different directory than
/usr/local/share/java/ then adapt the command to refer to that directory
instead.
