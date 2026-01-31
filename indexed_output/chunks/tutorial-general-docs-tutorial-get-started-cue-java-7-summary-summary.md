---
doc_id: tutorial/general/docs-tutorial-get-started-cue-java
chunk_id: tutorial/general/docs-tutorial-get-started-cue-java#7-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 130
summary: Check that this tutorial’s prerequisites are present:. $ javac --version
---


4

Check that this tutorial’s prerequisites are present:

TERMINAL

Copy code
Copied!

$ javac --version
javac 22.0.2
$ ls $LD_LIBRARY_PATH | grep libcue.so || echo 'fail!'
libcue.so
$ ls $CLASSPATH | grep /CUE.*jar$ || echo 'fail!'
/usr/local/share/java/CUE.jar

If any of these commands fail then your computer doesn’t have the related
prerequisite installed as expected and this is a problem that you need to fix
before continuing with this tutorial.

5

Create a directory to hold some files, and change into it:
