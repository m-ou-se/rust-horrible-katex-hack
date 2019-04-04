#![doc(html_favicon_url = "\">
<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/katex@0.10.1/dist/katex.min.css\" integrity=\"sha384-dbVIfZGuN1Yq7/1Ocstc1lUEm+AT+/rCkibIcC/OmWo5f0EA48Vf8CytHzGrSwbQ\" crossorigin=\"anonymous\">
<script defer src=\"https://cdn.jsdelivr.net/npm/katex@0.10.1/dist/katex.min.js\" integrity=\"sha384-2BKqo+exmr9su6dir+qCw08N2ZKRucY4PrGQPPWU1A7FtlCGjmEGFqXCv5nyM5Ij\" crossorigin=\"anonymous\"></script>
<script>
document.addEventListener(\"DOMContentLoaded\", function () {
	for (var e of document.getElementsByClassName(\"language-math\")){
		katex.render(e.innerText, e, {displayMode: true});
	}
	for (var e of document.getElementsByTagName(\"code\")) {
		if (e.classList.contains(\"language-math\")) continue;
		var n = e.nextSibling; var p = e.previousSibling;
		if (/^\\$/.test(n.data) && /\\$$/.test(p.data)) {
			katex.render(e.innerText, e);
			n.splitText(1); n.remove();
			p.splitText(p.data.length - 1).remove();
		}
	}
});
</script>
<meta name=\"")]

//! This crate is an example of using $`\LaTeX`$ math with rustdoc.
//!
//! This demo abuses the `#[doc(html_favicon_url = ..)]` attribute to inject
//! a KaTeX script into the generated documentation.
//!
//! This way, it works both on docs.rs and with `cargo doc` without extra settings.
//!
//! # Usage
//!
//! Look at the source of `lib.rs` of this crate, and copy the doc attribute
//! containing the `<link>` and `<script>` tags.
//!
//! Then, write ``$`\frac 1 2 + 3`$`` for inline math, which renders as $`\frac 1 2 + 3`$.
//!
//! Or, write
//!
//! ````markdown
//! ```math
//! \int_{-\infty}^\infty f(x)\,dx
//! ```
//! ````
//!
//! for display math, which renders as:
//!
//! ```math
//! \int_{-\infty}^\infty f(x)\,dx
//! ```