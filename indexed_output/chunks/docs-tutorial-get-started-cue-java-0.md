---
doc_id: docs-tutorial-get-started-cue-java
chunk_id: docs-tutorial-get-started-cue-java#0
heading: Introduction
token_count: 2869
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

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/tutorial/get-started-cue-java/&summary=The%20library%20cue-api-java%20provides%20a%20way%20to%20use%20CUE%20from%20Java%20programs.%20This%20tutorial%20helps%20you%20get%20started%20using%20cue-api-java,%20after%20you%20have%20installed%20it%20successfully.%0acue-api-java%20is%20an%20experimental%20technology%20preview.%20This%20means%20that%20it&rsquo;s%20under%20development%20and%20its%20behaviour%20might%20change%20from%20one%20release%20to%20the%20next.%20]


Converting values between Java and CUE
[/docs/tutorial/convert-values-java-cue/]Handling errors in the Java API
[/docs/tutorial/handle-errors-java-api/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
   * New to CUE? [/docs/tutorial/new-to-cue/]
   * Getting started using CUE in Java [/docs/tutorial/get-started-cue-java/]
      1. Prerequisites
      2. Set up your environment
      3. Create a Java program
      4. Compile the program
      5. Run the program
      6. Conclusion
      7. Related content
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
 * References [/docs/reference/]

Hide side navigation


Show side navigation

Get Started

 * Documentation [/docs/]
 * Language Tour [/docs/tour/]
 * Playground [/play/]
 * Install CUE [/docs/introduction/installation/]

Community

 * The CUE Community [/community]
 * Contributing [https://github.com/cue-lang/cue/blob/master/CONTRIBUTING.md#contribution-guide]
 * Code of Conduct [/docs/reference/code-of-conduct/]
 * Slack Workspace [/s/slack]
 * Discord Server [/s/discord]

Connect

 * GitHub [https://github.com/cue-lang/cue]
 * X (Twitter) [https://twitter.com/cue_lang]
 * Bluesky [https://bsky.app/profile/cuelang.org]
 * YouTube [https://www.youtube.com/@cuelang/videos]

 * © 2025 CUE
 * Privacy policy [/privacy-policy/]
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Ftutorial%2Fget-started-cue-java%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


Homepage of CUE [/]
CUE v0.15 is now available – learn more about its new features and improvements [https://github.com/cue-lang/cue/releases/tag/v0.15.0]
Install CUE

[/docs/introduction/installation/]

Close

Homepage of CUE [/]


Hide menu
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]
 * Install [/docs/introduction/installation/]
 * 

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
