---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#37-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 518
summary: #place: \"world\". // Outputs \"Hello world!\"
---


#place: "world"

// Outputs "Hello world!"

PACKAGE CLAUSE

A package clause is an optional clause that defines the package to which
a source file the file belongs.


Copy code
Copied!

PackageClause  = "package" PackageName .
PackageName    = identifier .

The PackageName must not be a definition identifier.

If the PackageName is the blank identifier (_), it is treated the same
as if there were no package clause. This can be useful to allow adding
package level attributes or doc comments to a CUE file without a package
name.


Copy code
Copied!

package math

MODULES AND INSTANCES

A module defines a tree of directories, rooted at the module root.

All source files within a module with the same package name belong to the same
package.

A module may define multiple packages.

An instance of a package is any subset of files belonging
to the same package.

It is interpreted as the concatenation of these files.

An implementation may impose conventions on the layout of package files
to determine which files of a package belongs to an instance.
For example, an instance may be defined as the subset of package files
belonging to a directory and all its ancestors.

IMPORT DECLARATIONS

An import declaration states that the source file containing the declaration
depends on definitions of the imported package
and enables access to exported identifiers of that package.
The import names an identifier (PackageName) to be used for access and an
ImportPath that specifies the package to be imported.


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
