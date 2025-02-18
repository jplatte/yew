use crate::{content::Author, generator::Generated, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

pub struct AuthorCard {
    author: Author,
}
impl Component for AuthorCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            author: Author::generate_from_seed(ctx.props().seed),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.author = Author::generate_from_seed(ctx.props().seed);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { author } = self;
        html! {
            <div class="card">
                <div class="card-content">
                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-128x128">
                                <img src={author.image_url.clone()} />
                            </figure>
                        </div>
                        <div class="media-content">
                            <p class="title is-3">{ &author.name }</p>
                            <p>
                                { "I like " }
                                <b>{ author.keywords.join(", ") }</b>
                            </p>
                        </div>
                    </div>
                </div>
                <footer class="card-footer">
                    <Link<Route> classes={classes!("card-footer-item")} to={Route::Author { id: author.seed }}>
                        { "Profile" }
                    </Link<Route>>
                </footer>
            </div>
        }
    }
}
