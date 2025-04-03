(function() {
    var type_impls = Object.fromEntries([["sequencer",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-ApiState%3CD%3E\" class=\"impl\"><a href=\"#impl-Clone-for-ApiState%3CD%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for ApiState&lt;D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; ApiState&lt;D&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.85.1/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.85.1/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","sequencer::api::endpoints::AvailState"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-ApiState%3CD%3E\" class=\"impl\"><a href=\"#impl-Debug-for-ApiState%3CD%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for ApiState&lt;D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.1/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.85.1/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","sequencer::api::endpoints::AvailState"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deref-for-ApiState%3CD%3E\" class=\"impl\"><a href=\"#impl-Deref-for-ApiState%3CD%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for ApiState&lt;D&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Target\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Target\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.85.1/core/ops/deref/trait.Deref.html#associatedtype.Target\" class=\"associatedtype\">Target</a> = <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;D&gt;</h4></section></summary><div class='docblock'>The resulting type after dereferencing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref\" class=\"method trait-impl\"><a href=\"#method.deref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.1/core/ops/deref/trait.Deref.html#tymethod.deref\" class=\"fn\">deref</a>(&amp;self) -&gt; &amp;&lt;ApiState&lt;D&gt; as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/deref/trait.Deref.html#associatedtype.Target\" title=\"type core::ops::deref::Deref::Target\">Target</a></h4></section></summary><div class='docblock'>Dereferences the value.</div></details></div></details>","Deref","sequencer::api::endpoints::AvailState"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CArc%3CD%3E%3E-for-ApiState%3CD%3E\" class=\"impl\"><a href=\"#impl-From%3CArc%3CD%3E%3E-for-ApiState%3CD%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;D&gt;&gt; for ApiState&lt;D&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(value: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;D&gt;) -&gt; ApiState&lt;D&gt;</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Arc<D>>","sequencer::api::endpoints::AvailState"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CD%3E-for-ApiState%3CD%3E\" class=\"impl\"><a href=\"#impl-From%3CD%3E-for-ApiState%3CD%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;D&gt; for ApiState&lt;D&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.1/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(d: D) -&gt; ApiState&lt;D&gt;</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<D>","sequencer::api::endpoints::AvailState"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ReadState-for-ApiState%3CD%3E\" class=\"impl\"><a href=\"#impl-ReadState-for-ApiState%3CD%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;D&gt; ReadState for ApiState&lt;D&gt;<div class=\"where\">where\n    D: 'static + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.State\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.State\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">State</a> = D</h4></section></summary><div class='docblock'>The type of state which this type allows a caller to read.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.read\" class=\"method trait-impl\"><a href=\"#method.read\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">read</a>&lt;'life0, 'async_trait, T&gt;(\n    &amp;'life0 self,\n    op: impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + for&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(&amp;'a &lt;ApiState&lt;D&gt; as ReadState&gt;::State) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/core/pin/struct.Pin.html\" title=\"struct core::pin::Pin\">Pin</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'a&gt;&gt; + 'async_trait,\n) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/core/pin/struct.Pin.html\" title=\"struct core::pin::Pin\">Pin</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'async_trait&gt;&gt;<div class=\"where\">where\n    'life0: 'async_trait,\n    T: 'async_trait,\n    ApiState&lt;D&gt;: 'async_trait,</div></h4></section></summary><div class='docblock'>Do an operation with immutable access to the state. <a>Read more</a></div></details></div></details>","ReadState","sequencer::api::endpoints::AvailState"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[10984]}