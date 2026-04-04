---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#8-detailed
chunk_level: detailed
chunk_type: table
heading: Shortcodes
token_count: 987
summary: ### Include Statements Shortcodes inside include statements will break the build. You must insert them in the parent document, before and after you call the include. For example: ``` `{{&lt; note...
---

### Include Statements
Shortcodes inside include statements will break the build. You must insert them
in the parent document, before and after you call the include. For example:
```
`{{&lt; note &gt;}}
{{&lt; include "task-tutorial-prereqs.md" &gt;}}
{{&lt; /note &gt;}}
`
```
### Line breaks
Use a single newline to separate block-level content like headings, lists, images,
code blocks, and others. The exception is second-level headings, where it should
be two newlines. Second-level headings follow the first-level (or the title) without
any preceding paragraphs or texts. A two line spacing helps visualize the overall
structure of content in a code editor better.
Manually wrap paragraphs in the Markdown source when appropriate. Since the git
tool and the GitHub website generate file diffs on a line-by-line basis,
manually wrapping long lines helps the reviewers to easily find out the changes
made in a PR and provide feedback. It also helps the downstream localization
teams where people track the upstream changes on a per-line basis. Line
wrapping can happen at the end of a sentence or a punctuation character, for
example. One exception to this is that a Markdown link or a shortcode is
expected to be in a single line.
### Headings and titles
People accessing this documentation may use a screen reader or other assistive technology (AT).
[Screen readers](https://en.wikipedia.org/wiki/Screen_reader) are linear output devices,
they output items on a page one at a time. If there is a lot of content on a page, you can
use headings to give the page an internal structure. A good page structure helps all readers
to easily navigate the page or filter topics of interest.
Do and Don't - Headings|Do|Don't|
|Update the title in the front matter of the page or blog post.|Use first level heading, as Hugo automatically converts the title in the front matter of the page into a first-level heading.|
|Use ordered headings to provide a meaningful high-level outline of your content.|Use headings level 4 through 6, unless it is absolutely necessary. If your content is that detailed, it may need to be broken into separate articles.|
|Use pound or hash signs (`#`) for non-blog post content.|Use underlines (`---` or `===`) to designate first-level headings.|
|Use sentence case for headings in the page body. For example, **Extend kubectl with plugins**|Use title case for headings in the page body. For example, **Extend Kubectl With Plugins**|
|Use title case for the page title in the front matter. For example, `title: Kubernetes API Server Bypass Risks`|Use sentence case for page titles in the front matter. For example, don't use `title: Kubernetes API server bypass risks`|
|Place relevant links in the body copy.|Include hyperlinks (`&lt;a href=""&gt;&lt;/a&gt;`) in headings.|
|Use pound or hash signs (`#`) to indicate headings.|Use **bold** text or other indicators to split paragraphs.|
### Paragraphs
Do and Don't - Paragraphs|Do|Don't|
|Try to keep paragraphs under 6 sentences.|Indent the first paragraph with space characters. For example, ⋅⋅⋅Three spaces before a paragraph will indent it.|
|Use three hyphens (`---`) to create a horizontal rule. Use horizontal rules for breaks in paragraph content. For example, a change of scene in a story, or a shift of topic within a section.|Use horizontal rules for decoration.|
### Links
Do and Don't - Links|Do|Don't|
|Write hyperlinks that give you context for the content they link to. For example: Certain ports are open on your machines. See [Check required ports](#check-required-ports) for more details.|Use ambiguous terms such as "click here". For example: Certain ports are open on your machines. See [here](#check-required-ports) for more details.|
|Write Markdown-style links: `[link text](URL)`. For example: `[Hugo shortcodes](/docs/contribute/style/hugo-shortcodes/#table-captions)` and the output is [Hugo shortcodes](/docs/contribute/style/hugo-shortcodes/#table-captions).|Write HTML-style links: `&lt;a href="/media/examples/link-element-example.css" target="\_blank"&gt;Visit our tutorial!&lt;/a&gt;`, or create links that open in new tabs or windows. For example: `[example website](https://example.com){target="\_blank"}`|