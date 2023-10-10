use crate::components::article_ids::ArticleIds;
use crate::components::common_functions::remove_first_two_lines;
use std::collections::HashMap;
use std::ops::Deref;
use sxd_document::parser;
use sxd_xpath::{evaluate_xpath, Value};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(SearchPage)]
pub fn search_page() -> Html {
    let search_text = use_state(|| "".to_owned());
    let search_text_clone = search_text.clone();
    let ids_list: UseStateHandle<Vec<String>> = use_state(|| vec![]);
    let search_text_clone2 = search_text.clone();
    let ids_list_clone = ids_list.clone();
    let ids_list_clone2 = ids_list.clone();

    let search_query_changed = Callback::from(move |event: Event| {
        let search_query = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        search_text_clone.set(search_query);
    });

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let search_text_state = search_text_clone2.clone();
        let ids_list = ids_list_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let search_text_state = search_text_state.clone();
            let ids_list = ids_list.clone();

            let search_text = format!("{}", search_text_state.deref());
            let url = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi";
            let mut params = HashMap::new();
            params.insert("db", "pubmed");
            params.insert("retmax", "100");
            params.insert("term", &search_text);

            let client = reqwest_wasm::Client::new();
            let response = client.post(url).form(&params).send().await.unwrap();

            let body = response.text().await.unwrap();
            // log!(body);
            let data = remove_first_two_lines(&body);
            let package = parser::parse(&data).expect("failed to parse XML");
            let document = package.as_document();

            let value = evaluate_xpath(&document, "//Id");
            let mut article_ids = vec![];
            match value {
                Ok(Value::Nodeset(nodeset)) => {
                    for node in nodeset.iter() {
                        article_ids.push(node.string_value());
                    }
                }
                _ => {}
            }

            ids_list.set(article_ids);
        });
    });

    html! {
        <>
        <div class="container">
            <h1>{"Pubmed Scihub Search"}</h1>
            <form onsubmit={onsubmit} >
                <div class="mb-3">
                    <label for="search" class="form-label">{"Enter pubmed search query"}</label>
                    <input type="text" class="form-control" name="search" onchange={search_query_changed} />
                    <br/>
                    <p>{"e.g. Evolutionary Game[Title]"}</p>
                </div>
                <button>{"Search"}</button>
            </form>
            <br/>
            <ArticleIds article_list={ids_list_clone2.clone().to_vec()} />
        </div>
        </>
    }
}
