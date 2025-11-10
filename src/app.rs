use cross_storage::{storage_get, storage_set};

use dioxus::prelude::*;

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

        // required for popover components
        div { id: POPOVER_TARGET_ID }

        Toaster {

            main { class: "relative mx-auto max-w-4xl h-screen w-full min-w-0 px-4 sm:px-6 md:px-8",
                div { class: "relative flex h-full items-center justify-center py-8",
                    div { class: "w-full self-start pt-[12vh] sm:pt-[15vh] md:pt-[18vh]",

                        Tabs {
                            default_tab: preferred_role,
                            class: "w-full",
                            on_change: on_tab_change,
                            TabsList { class: "mb-4",
                                TabsTrigger { id: "send",
                                    "Sender"
                                    Send { class: "ml-1.5 size-3.5" }
                                }
                                TabsTrigger { id: "receive",
                                    "Receiver"
                                    ScanEye { class: "ml-1.5 size-3.5" }
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
