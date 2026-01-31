---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#19-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1025
summary: ImportDecl       = \"import\" ( ImportSpec | \"(\" { ImportSpec \",\" } \")\" ) . ImportSpec       = [ PackageName ] ImportPath 
---



Copy code
Copied!

ImportDecl       = "import" ( ImportSpec | "(" { ImportSpec "," } ")" ) .
ImportSpec       = [ PackageName ] ImportPath .
ImportLocation   = { unicode_value } .
ImportPath       = `"` ImportLocation [ ":" identifier ] `"` .

The PackageName is used in qualified identifiers to access
exported identifiers of the package within the importing source file.
It is declared in the file block.
It defaults to the identifier specified in the package clause of the imported
package, which must match either the last path component of ImportLocation
or the identifier following it.

The interpretation of the ImportPath is implementation-dependent but it is
typically either the path of a builtin package or a fully qualifying location
of a package within a source code repository.

An ImportLocation must be a non-empty string using only characters belonging to
Unicode’s L, M, N, P, and S general categories
(the Graphic characters without spaces)
and may not include the characters !"#$%&'()*,:;<=>?[\\]^`{|}
or the Unicode replacement character U+FFFD.

Assume we have package containing the package clause package math,
which exports function Sin at the path identified by lib/math.
This table illustrates how Sin is accessed in files
that import the package after the various types of import declaration.


Copy code
Copied!

Import declaration          Local name of Sin

import   "lib/math"         math.Sin
import   "lib/math:math"    math.Sin
import m "lib/math"         m.Sin

An import declaration declares a dependency relation between the importing and
imported package. It is illegal for a package to import itself, directly or
indirectly, or to directly import a package without referring to any of its
exported identifiers.

AN EXAMPLE PACKAGE

TODO

Last modified November 6, 2025 [https://github.com/cue-lang/cuelang.org/commit/c4d2e727a59f5158a2be4946992d96c4612a9b88]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/reference/spec/&text=%20Note%20to%20implementors%20Notes%20on%20the%20formalism%20underlying%20this%20specification%20can%20be%20found%20here.%0aIntroduction%20This%20is%20a%20reference%20manual%20for%20the%20CUE%20data%20constraint%20language.%20CUE,%20pronounced%20cue%20or%20Q,%20is%20a%20general-purpose%20and%20strongly%20typed%20constraint-based%20language.%20It%20can%20be%20used%20for%20data%20templating,%20data%20validation,%20code%20generation,%20scripting,%20and%20many%20other%20applications%20involving%20structured%20data.%20The%20CUE%20tooling,%20layered%20on%20top%20of%20CUE,%20provides%20a%20general%20purpose%20scripting%20language%20for%20creating%20scripts%20as%20well%20as%20simple%20servers,%20also%20expressed%20in%20CUE.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/reference/spec/&summary=%20Note%20to%20implementors%20Notes%20on%20the%20formalism%20underlying%20this%20specification%20can%20be%20found%20here.%0aIntroduction%20This%20is%20a%20reference%20manual%20for%20the%20CUE%20data%20constraint%20language.%20CUE,%20pronounced%20cue%20or%20Q,%20is%20a%20general-purpose%20and%20strongly%20typed%20constraint-based%20language.%20It%20can%20be%20used%20for%20data%20templating,%20data%20validation,%20code%20generation,%20scripting,%20and%20many%20other%20applications%20involving%20structured%20data.%20The%20CUE%20tooling,%20layered%20on%20top%20of%20CUE,%20provides%20a%20general%20purpose%20scripting%20language%20for%20creating%20scripts%20as%20well%20as%20simple%20servers,%20also%20expressed%20in%20CUE.%0a]


Contribution Guide
[/docs/reference/contribution-guidelines/]Glossary of terms
[/docs/reference/glossary/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
 * References [/docs/reference/]
   * The CUE Language Specification [/docs/reference/spec/]
      1.  Introduction
      2.  Notation
