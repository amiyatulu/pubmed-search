// use gloo::console::log;
use std::collections::HashMap;
use std::ops::Deref;
use sxd_document::parser;
use sxd_xpath::evaluate_xpath;
use wasm_bindgen_futures;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ArticleProps {
    pub article_id: String,
    pub count: u32,
}

fn remove_first_two_lines(input: &str) -> String {
    let mut lines = input.lines();
    let mut result = String::new();

    // Skip the first two lines
    lines.nth(0);
    lines.nth(0);

    for line in lines {
        result.push_str(line);
        result.push('\n');
    }

    result
}

#[function_component(QueryArticle)]
pub fn query_article(ArticleProps { article_id, count }: &ArticleProps) -> Html {
    let article_id_clone = article_id.clone();
    let doi = use_state(|| "".to_string());
    let doi_clone = doi.clone();
    let title = use_state(|| "".to_string());
    let title_clone = title.clone();

    use_timeout(
        move || {
            let article_id = article_id_clone.clone();
            let doi = doi.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi";
                let mut params = HashMap::new();
                params.insert("db", "pubmed");
                params.insert("id", &article_id);
                params.insert("format", "xml");

                let client = reqwest_wasm::Client::new();
                let response = client.post(url).form(&params).send().await.unwrap();

                let body = response.text().await.unwrap();
                let data = remove_first_two_lines(&body);
                let package = parser::parse(&data).expect("failed to parse XML");
                let document = package.as_document();
                let value = evaluate_xpath(&document, "//ELocationID[@EIdType='doi']")
                    .expect("XPath evaluation failed");
                // log!(body);
                let title_value = evaluate_xpath(&document, "//ArticleTitle").expect("Unable to get title");

                // log!(format!("{}", value.string()));
                doi.set(value.string());
                title.set(title_value.string());
            });
        },
        count * 2000,
    );
    html! {
        <>
        <div class="container">
        if !doi_clone.deref().clone().is_empty() {
            <div>
            <div>
            <a href={format!("https://doi.org/{}", doi_clone.deref().clone())} target="_blank" rel="noopener noreferrer" class="link-primary link-offset-2 link-underline-opacity-25 link-underline-opacity-100-hover"> <h5>{title_clone.deref().clone()}</h5></a>
            </div>
            <div>
            <a href={format!("https://sci-hub.st/{}", doi_clone.deref().clone())} target="_blank" rel="noopener noreferrer" class="link-info link-offset-2 link-underline-opacity-25 link-underline-opacity-100-hover"> <p>{"Sci hub"}</p></a>
            </div>
            </div>
        }
        </div>
        </>
    }
}
