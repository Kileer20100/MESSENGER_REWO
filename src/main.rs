use dioxus::{logger::tracing::info, prelude::*};
use dioxus_elements::{div, img, p, source::src};
use svg_attributes::id;
use dioxus::prelude::*;
fn main() {
    dioxus::launch(App);
}
static CSS: Asset = asset!("/assets/main.css");
#[component]
fn App() -> Element {
    rsx!(
        document::Stylesheet{href: CSS}
        Menu{}  
        ImageHomeHead{}
        Border1{}
        Border2{}
        Border3{}
        Foter{}
    )
}


#[component]
fn Menu() -> Element {
    rsx! {
        div { h1 { id: "title", "WORE"}
        div { id:"Panel", button { id:"Button-menu", "О WORE" }
       ,button { id:"Button-menu", "Documentation" }
       ,button { id:"Button-menu", "Support" }
       ,button { id:"Button-menu", "Guides" }
       ,button { id:"Button-menu", "Contacts" }
       ,button { id:"Button-menu", "Download" }
       ,button { id:"Button-menu", "WEB" }
       } }
    }
}
#[component]
fn ImageHomeHead()->Element{
    rsx!(
        div {  id: "image-mesanger",
        button { id:"button_<","<" },
        button { id:"button_>",">" }
    }

        div { p { id:"text-head", "You can download the app or use the web version on your device" } }
        div { button { id:"button","Download App" },
                button {  id:"button","Use Web Version" } }
    )
}
#[component]
fn Border1()->Element{
    rsx!(
        div { h1 { id:"Info-wore-h2","What is the WORE Messenger?" } }
        div { id:"text-WORE-title",  
        div { id:"text-WORE", p { "The WORE Messenger is a modern platform that has moved away from traditional concepts of a messenger.
This messenger does not have a central server for data storage. Instead, all management is done by users, who create their own networks and share information on their terms.
WORE provides complete freedom of communication and privacy.
Here, every user becomes part of a community where data remains safe and protected. All decisions are made based on decentralized governance, ensuring transparency and independence.
You can easily exchange messages, share files, and create groups that match your interests." } }  
        div { id:"image-wore" }
        }
    )
}

#[component]
fn Border2()->Element{
    rsx!(
        div { h1 { id:"Info-wore-h2","What are networks in the WORE messenger?" } }
        div { id:"text-WORE-title",  
        div { id:"image-wore" },
        div { id:"text-WORE", p { "Networks in the WORE messenger are a key functional component that allows users to interact and share information without intermediaries. Unlike traditional messengers, WORE does not have a central server for data storage. Instead, each network is created by the users themselves. By participating in this network, everyone becomes part of a community that is governed collectively.

Each network can be either private, accessible only to a limited group of people, or public, allowing any user to join and interact." } }
        }

    )
}

#[component]
fn Border3()->Element{
    rsx!(
        div { h1 { id:"Info-wore-h2","How is security achieved in WORE?" } }
        div { id:"text-WORE-title",  
        div { id:"text-WORE-3", p { " Decentralized management — information is not stored on a central server, but is distributed among users."
        ,p { "Data encryption — messages and files are transmitted and stored using modern encryption algorithms, which protects against leaks and unauthorized access." }
        ,p { "Control over data — only users decide who and how to share information with." }
        ,p { "    Creating private networks — users can create private networks that are accessible only to a limited group of people, as well as public ones." }
     } }  
        div { id:"image-wore" }
        }
    )
}


#[component]
fn Foter() -> Element {
    rsx! {
        div { id:"foter-WORE",
        h2 { id:"h2-foter","WORE" }
        div { id:"foter-block-1" }
        }
    }
}