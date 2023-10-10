use crate::components::query_article::QueryArticle;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ArticleListProps {
    pub article_list: Vec<String>,
}

#[function_component(ArticleIds)]
pub fn article_ids(ArticleListProps { article_list }: &ArticleListProps) -> Html {
    let mut count: u32 = 0;
    let data = article_list
        .iter()
        .map(|article_id| {
            count +=1;
            html! {

                    <QueryArticle key={article_id.clone()} count={count.clone()} article_id={article_id.clone()} />

            }
        })
        .collect();

    data
}
