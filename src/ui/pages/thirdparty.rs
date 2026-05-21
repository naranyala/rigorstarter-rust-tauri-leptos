use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn LeafletDemo() -> impl IntoView {
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            let init = Closure::once_into_js(move || {
                let _ = js_sys::eval(
                    r#"var el=document.getElementById('leaflet-map');
if(el&&typeof L!=='undefined'){
var map=L.map(el).setView([-6.2,106.8],13);
L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png',{
attribution:'&copy; OpenStreetMap contributors',maxZoom:19
}).addTo(map);
}"#,
                );
            });
            let _ = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(init.unchecked_ref(), 300);
        }
    });

    view! {
        <div class="thirdparty-demo">
            <h2>"Leaflet Map"</h2>
            <p>"Interactive map rendered by " <a href="https://leafletjs.com/" target="_blank">"Leaflet"</a> " loaded from " <code>thirdparty/node_modules/leaflet</code> ". Uses OpenStreetMap tiles."</p>
            <div id="leaflet-map" class="leaflet-container"></div>
        </div>
    }
}

#[component]
pub fn MathJaxDemo() -> impl IntoView {
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            let init = Closure::once_into_js(move || {
                let _ = js_sys::eval(
                    r#"var el=document.getElementById('mathjax-content');
if(el&&typeof MathJax!=='undefined'&&MathJax.typesetPromise){
MathJax.typesetPromise([el]);
}"#,
                );
            });
            let _ = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(init.unchecked_ref(), 300);
        }
    });

    view! {
        <div class="thirdparty-demo">
            <h2>"MathJax (LaTeX Typesetting)"</h2>
            <p>"Mathematical formulas rendered by " <a href="https://www.mathjax.org/" target="_blank">"MathJax"</a> " loaded from " <code>thirdparty/node_modules/mathjax</code> "."</p>

            <section class="demo-section">
                <h3>"Quadratic Formula"</h3>
                <p>"When \\(a \\ne 0\\), there are two solutions to \\(ax^2 + bx + c = 0\\):"</p>
                <p>"\\[x = {-b \\pm \\sqrt{b^2-4ac} \\over 2a}\\]"</p>
            </section>

            <section class="demo-section">
                <h3>"Cauchy-Schwarz Inequality"</h3>
                <p>"\\[ \\left( \\sum_{k=1}^n a_k b_k \\right)^2 \\leq \\left( \\sum_{k=1}^n a_k^2 \\right) \\left( \\sum_{k=1}^n b_k^2 \\right) \\]"</p>
            </section>

            <section class="demo-section">
                <h3>"Taylor Series"</h3>
                <p>"\\[ e^x = \\sum_{n=0}^{\\infty} \\frac{x^n}{n!} \\]"</p>
            </section>

            <section class="demo-section">
                <h3>"Matrix"</h3>
                <p>"\\[ \\begin{pmatrix} a & b \\\\ c & d \\end{pmatrix} \\]"</p>
            </section>
        </div>
    }
}

#[component]
pub fn MermaidDemo() -> impl IntoView {
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            let init = Closure::once_into_js(move || {
                let _ = js_sys::eval(
                    r#"mermaid.initialize({startOnLoad:false,theme:'default'});
var els=document.querySelectorAll('.mermaid');
if(els.length>0)mermaid.run({nodes:els});"#,
                );
            });
            let _ = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(init.unchecked_ref(), 300);
        }
    });

    view! {
        <div class="thirdparty-demo">
            <h2>"Mermaid Diagrams"</h2>
            <p>"Diagrams and flowcharts rendered by " <a href="https://mermaid.js.org/" target="_blank">"Mermaid"</a> " loaded from " <code>thirdparty/node_modules/mermaid</code> "."</p>

            <section class="demo-section">
                <h3>"Flowchart"</h3>
                <pre class="mermaid">
"graph TD
    A[Start] --> B{Is it working?}
    B -->|Yes| C[Great!]
    B -->|No| D[Debug]
    D --> B"
                </pre>
            </section>

            <section class="demo-section">
                <h3>"Sequence Diagram"</h3>
                <pre class="mermaid">
"sequenceDiagram
    participant User
    participant App
    participant Server
    User->>App: Click button
    App->>Server: API request
    Server-->>App: JSON response
    App-->>User: Update UI"
                </pre>
            </section>

            <section class="demo-section">
                <h3>"Architecture Diagram"</h3>
                <pre class="mermaid">
"flowchart LR
    A[Leptos WASM] -->|js-sys eval| B(JS Runtime)
    B --> C{3rd Party Libs}
    C -->|L.map| D[Leaflet]
    C -->|MathJax.typeset| E[MathJax]
    C -->|mermaid.run| F[Mermaid]
    D --> G[OpenStreetMap]
    E --> H[TeX Input]
    F --> I[SVG Output]"
                </pre>
            </section>
        </div>
    }
}
