---
doc_id: tutorial/general/docs-tutorial-get-started-cue-java
chunk_id: tutorial/general/docs-tutorial-get-started-cue-java#1-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 515
summary:  * Your computer needs to have the Java Development Kit (JDK).    version 22 [https://openjdk
---

 * Your computer needs to have the Java Development Kit (JDK)
   version 22 [https://openjdk.org/projects/jdk/22/]
   (or later) installed. If you need help choosing a distribution of Java,
   the site whichjdk.com [https://whichjdk.com] is a useful guide.
   Note that many operating systems offer a “Long Term Support” version of Java,
   but this often means JDK version 21, which isn’t recent enough.
   Make sure that you have version 22 or later!

SET UP YOUR ENVIRONMENT

1

Tell the operating system (and Java) where to find the library libcue on your
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
