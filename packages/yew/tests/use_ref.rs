mod common;

use common::obtain_result;
use std::ops::DerefMut;
use wasm_bindgen_test::*;
use yew::functional::{use_ref, use_state, FunctionComponent, FunctionProvider};
use yew::{html, Html};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn use_ref_works() {
    struct UseRefFunction {}
    impl FunctionProvider for UseRefFunction {
        type TProps = ();

        fn run(_: &Self::TProps) -> Html {
            let ref_example = use_ref(|| 0);
            *ref_example.borrow_mut().deref_mut() += 1;
            let counter = use_state(|| 0);
            if *counter < 5 {
                counter.set(*counter + 1)
            }
            return html! {
                <div>
                    {"The test output is: "}
                    <div id="result">{*ref_example.borrow_mut().deref_mut() > 4}</div>
                    {"\n"}
                </div>
            };
        }
    }
    type UseRefComponent = FunctionComponent<UseRefFunction>;
    yew::start_app_in_element::<UseRefComponent>(
        gloo_utils::document().get_element_by_id("output").unwrap(),
    );

    let result = obtain_result();
    assert_eq!(result.as_str(), "true");
}
