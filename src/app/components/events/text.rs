use crate::app::components::events::{get_sender_displayname, is_new_user};
use linkify::LinkFinder;
use matrix_sdk::{
    events::room::message::{MessageEvent, TextMessageEventContent},
    Room,
};
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;

pub struct Text {
    props: Props,
}

#[derive(Clone, Properties, Debug)]
pub struct Props {
    #[prop_or_default]
    pub prev_event: Option<MessageEvent>,
    #[prop_or_default]
    pub event: Option<MessageEvent>,
    #[prop_or_default]
    pub text_event: Option<TextMessageEventContent>,
    #[prop_or_default]
    pub room: Option<Room>,
}

impl Component for Text {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Text { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        // TODO fix the PartialEq hack
        if format!("{:#?}", self.props) != format!("{:#?}", props) {
            self.props = props;
            true
        } else {
            false
        }
    }

    //noinspection RsTypeCheck
    fn view(&self) -> Html {
        let new_user = is_new_user(
            self.props.prev_event.clone(),
            self.props.event.clone().unwrap(),
        );
        let sender_displayname = if new_user {
            get_sender_displayname(
                self.props.room.clone().unwrap(),
                self.props.event.clone().unwrap(),
            )
        } else {
            "".to_string()
        };

        let mut pure_content = self.props.text_event.clone().unwrap().body;
        let finder = LinkFinder::new();
        let pure_content_clone = pure_content.clone();
        let links: Vec<_> = finder.links(&pure_content_clone).collect();

        let content = if !links.is_empty() {
            for link in links {
                let html_link = format!("<a href={}>{}</a>", link.as_str(), link.as_str());
                pure_content.replace_range(link.start()..link.end(), &html_link);
            }
            pure_content
        } else {
            pure_content
        };

        if self.props.text_event.clone().unwrap().formatted.is_some() {
            let message = if new_user {
                format!(
                    "<displayname>{}:</displayname> {}",
                    sender_displayname,
                    self.props
                        .text_event
                        .clone()
                        .unwrap()
                        .formatted
                        .unwrap()
                        .body
                )
            } else {
                self.props
                    .text_event
                    .clone()
                    .unwrap()
                    .formatted
                    .unwrap()
                    .body
            };
            let js_text_event = {
                let div = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("p")
                    .unwrap();
                div.set_inner_html(message.as_str());
                div
            };
            let node = Node::from(js_text_event);
            VNode::VRef(node)
        } else if new_user {
            let full_html = format!(
                "<p><displayname>{}: </displayname>{}</p>",
                sender_displayname,
                content
            );
            let js_text_event = {
                let div = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("p")
                    .unwrap();
                div.set_inner_html(full_html.as_str());
                div
            };
            let node = Node::from(js_text_event);
            VNode::VRef(node)
        } else {
            let full_html = format!("<p>{}</p>", content);
            let js_text_event = {
                let div = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("p")
                    .unwrap();
                div.set_inner_html(full_html.as_str());
                div
            };
            let node = Node::from(js_text_event);
            VNode::VRef(node)
        }
    }
}
