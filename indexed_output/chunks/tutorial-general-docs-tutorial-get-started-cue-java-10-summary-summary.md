---
doc_id: tutorial/general/docs-tutorial-get-started-cue-java
chunk_id: tutorial/general/docs-tutorial-get-started-cue-java#10-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary:         var c1 = ctx. compile(\"a: int\");
---

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
