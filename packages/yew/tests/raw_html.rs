mod common;

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
use wasm_bindgen::JsCast;
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
use wasm_bindgen_test::wasm_bindgen_test as test;
use yew::prelude::*;
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
use tokio::test;

macro_rules! create_test {
    ($name:ident, $html:expr) => {
        create_test!($name, $html, $html);
    };
    ($name:ident, $raw:expr, $expected:expr) => {
        #[test]
        async fn $name() {
            #[component]
            fn App() -> Html {
                let raw = Html::from_html_unchecked(AttrValue::from($raw));
                html! {
                    <div id="raw-container">
                        {raw}
                    </div>
                }
            }

            #[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
            {
                use std::time::Duration;

                use yew::platform::time::sleep;

                yew::Renderer::<App>::with_root(
                    gloo::utils::document().get_element_by_id("output").unwrap(),
                )
                .render();

                // wait for render to finish
                sleep(Duration::from_millis(100)).await;

                let e = gloo::utils::document()
                    .get_element_by_id("raw-container")
                    .unwrap();
                assert_eq!(e.inner_html(), $expected);
            }
            #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
            {
                let actual = yew::LocalServerRenderer::<App>::new()
                    .hydratable(false)
                    .render()
                    .await;
                assert_eq!(
                    actual,
                    format!(r#"<div id="raw-container">{}</div>"#, $expected)
                );
            }
        }
    };
}

create_test!(empty_string, "");
create_test!(one_node, "<span>text</span>");
create_test!(
    one_but_nested_node,
    r#"<p>one <a href="https://yew.rs">link</a> more paragraph</p>"#
);
create_test!(
    multi_node,
    r#"<p>paragraph</p><a href="https://yew.rs">link</a>"#
);

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
macro_rules! create_update_html_test {
    ($name:ident, $initial:expr, $updated:expr) => {
        #[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
        #[test]
        async fn $name() {
            #[component]
            fn App() -> Html {
                let raw_html = use_state(|| ($initial));
                let onclick = {
                    let raw_html = raw_html.clone();
                    move |_| raw_html.set($updated)
                };
                let raw = Html::from_html_unchecked(AttrValue::from(*raw_html));
                html! {
                    <>
                        <div id="raw-container">
                            {raw}
                        </div>
                        <button id="click-me-btn" {onclick}>{"Click me"}</button>
                    </>
                }
            }
            use std::time::Duration;

            use yew::platform::time::sleep;

            yew::Renderer::<App>::with_root(
                gloo::utils::document().get_element_by_id("output").unwrap(),
            )
            .render();

            // wait for render to finish
            sleep(Duration::from_millis(100)).await;

            let e = gloo::utils::document()
                .get_element_by_id("raw-container")
                .unwrap();
            assert_eq!(e.inner_html(), $initial);

            gloo::utils::document()
                .get_element_by_id("click-me-btn")
                .unwrap()
                .unchecked_into::<web_sys::HtmlButtonElement>()
                .click();

            sleep(Duration::from_millis(100)).await;

            let e = gloo::utils::document()
                .get_element_by_id("raw-container")
                .unwrap();
            assert_eq!(e.inner_html(), $updated);
        }
    };
}

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
create_update_html_test!(
    set_new_html_string,
    "<span>first</span>",
    "<span>second</span>"
);

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
create_update_html_test!(
    set_new_html_string_multiple_children,
    "<span>first</span><span>second</span>",
    "<span>second</span>"
);

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
create_update_html_test!(
    clear_html_string_multiple_children,
    "<span>first</span><span>second</span>",
    ""
);

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
create_update_html_test!(
    nothing_changes,
    "<span>first</span><span>second</span>",
    "<span>first</span><span>second</span>"
);

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
#[test]
async fn change_vnode_types_from_other_to_vraw() {
    #[component]
    fn App() -> Html {
        let node = use_state(|| html!("text"));
        let onclick = {
            let node = node.clone();
            move |_| {
                node.set(Html::from_html_unchecked(AttrValue::from(
                    "<span>second</span>",
                )))
            }
        };
        html! {
            <>
                <div id="raw-container">
                    {(*node).clone()}
                </div>
                <button id="click-me-btn" {onclick}>{"Click me"}</button>
            </>
        }
    }
    use std::time::Duration;

    use yew::platform::time::sleep;

    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();

    // wait for render to finish
    sleep(Duration::from_millis(100)).await;

    let e = gloo::utils::document()
        .get_element_by_id("raw-container")
        .unwrap();
    assert_eq!(e.inner_html(), "text");

    gloo::utils::document()
        .get_element_by_id("click-me-btn")
        .unwrap()
        .unchecked_into::<web_sys::HtmlButtonElement>()
        .click();

    sleep(Duration::from_millis(100)).await;

    let e = gloo::utils::document()
        .get_element_by_id("raw-container")
        .unwrap();
    assert_eq!(e.inner_html(), "<span>second</span>");
}

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
#[test]
async fn change_vnode_types_from_vraw_to_other() {
    #[component]
    fn App() -> Html {
        let node = use_state(|| Html::from_html_unchecked(AttrValue::from("<span>second</span>")));
        let onclick = {
            let node = node.clone();
            move |_| node.set(html!("text"))
        };
        html! {
            <>
                <div id="raw-container">
                    {(*node).clone()}
                </div>
                <button id="click-me-btn" {onclick}>{"Click me"}</button>
            </>
        }
    }
    use std::time::Duration;

    use yew::platform::time::sleep;

    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();

    // wait for render to finish
    sleep(Duration::from_millis(100)).await;

    let e = gloo::utils::document()
        .get_element_by_id("raw-container")
        .unwrap();
    assert_eq!(e.inner_html(), "<span>second</span>");

    gloo::utils::document()
        .get_element_by_id("click-me-btn")
        .unwrap()
        .unchecked_into::<web_sys::HtmlButtonElement>()
        .click();

    sleep(Duration::from_millis(100)).await;

    let e = gloo::utils::document()
        .get_element_by_id("raw-container")
        .unwrap();
    assert_eq!(e.inner_html(), "text");
}
