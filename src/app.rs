use cross_storage::{storage_get, storage_set};

use dioxus::prelude::*;

use dioxus_tw_components::prelude::DioxusTwComponentsBootstrap;
use lucide_dioxus::{ScanEye, Send};

use crate::{
    ui::{
        popover::POPOVER_TARGET_ID,
        tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
        toast::Toaster,
    },
    views::{ReceiverView, SenderView},
};

const FAVICON: Asset = asset!("/src/theme/fav.svg");

const CSS_HREF: Asset = asset!("/tmp/styles.css");
const CSS_STYLE: &str = include_str!("../tmp/styles.css");

#[component]
pub fn App() -> Element {
    let preferred_role = storage_get("preferred_role").unwrap_or("send".to_string());

    let on_tab_change = move |tab: String| {
        let _ = storage_set("preferred_role", &tab);
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }

        document::Link { rel: "stylesheet", href: CSS_HREF }
        style { {CSS_STYLE} }

        // launches Dioxus Tailwind Components
        // some components may not work without this
        DioxusTwComponentsBootstrap {}

        // required for popover components
        div { id: POPOVER_TARGET_ID }



        Toaster {

            main { class: "relative mx-auto max-w-200 h-screen w-screen min-w-90 grow px-4",
                div { class: "relative flex h-full items-center justify-center",
                    div { class: "w-full self-start pt-[20vh]",

                        Tabs {
                            default_tab: preferred_role,
                            class: "w-full",
                            on_change: on_tab_change,
                            TabsList {
                                TabsTrigger { id: "send",
                                    "Sender"
                                    Send { class: "ml-2 size-3" }
                                }
                                TabsTrigger { id: "receive",
                                    "Receiver"
                                    ScanEye { class: "ml-2" }
                                }
                            }

                            TabsContent { id: "send", class: "animate-fade-in", SenderView {} }
                            TabsContent { id: "receive", class: "animate-fade-in", ReceiverView {} }
                        }
                    }
                }
            }

        }

    }
}
