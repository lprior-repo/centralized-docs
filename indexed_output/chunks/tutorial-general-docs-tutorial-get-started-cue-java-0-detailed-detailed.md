---
doc_id: tutorial/general/docs-tutorial-get-started-cue-java
chunk_id: tutorial/general/docs-tutorial-get-started-cue-java#0-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1033
summary: # Getting started using CUE in Java | CUE. **Source:** https://cuelang
---

# Getting started using CUE in Java | CUE

**Source:** https://cuelang.org/docs/tutorial/get-started-cue-java/

Skip to content

Homepage of CUE [/]
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]

Install
[/docs/introduction/installation/]

Search [/search]

What are you looking for?

Menu

 1. Tutorials [https://cuelang.org/docs/tutorial/]


 2. GETTING STARTED USING CUE IN JAVA

jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews
jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews

Github profile

[https://github.com/jpluscplusm]

Search all content by this author

[/search/?q=author:jpluscplusm]
4ad [https://github.com/4ad.png]
Aram Hăvărneanu
4ad [https://github.com/4ad.png]
Aram Hăvărneanu

Github profile

[https://github.com/4ad]

Search all content by this author

[/search/?q=author:4ad]
 * java api [/search?q=tag:%22java%20api%22]

The library cue-api-java provides a way to use CUE from Java programs.
This tutorial helps you get started using cue-api-java,
after you have installed it successfully.

cue-api-java is an experimental technology preview. This means that it’s
under development and its behaviour might change from one release to the next.

PREREQUISITES

 * You need the low-level library
   libcue [https://github.com/cue-lang/libcue]
   to be compiled and available on your computer,
   which is demonstrated in the guide
   “Building libcue as a shared library [/docs/howto/build-libcue-shared-library/]”.
 * You must have the Java library
   cue-api-java [https://github.com/cue-lang/cue-api-java]
   compiled and available on your computer. The guide
   “Building cue-api-java as a JAR file [/docs/howto/build-cue-api-java-jar/]”
   shows you how to compile and install it.
   This tutorial needs you to install the same version as that guide.
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
