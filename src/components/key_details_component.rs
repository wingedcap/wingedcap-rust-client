use dioxus::prelude::*;

use wingedcap::client::{ServerMeta, ServerWithMeta};

use crate::ui::hovercard::{HoverCard, HoverCardContent, HoverCardTrigger};
use crate::ui::separator::Separator;

use lucide_dioxus::{Fingerprint, HatGlasses, IdCard, Link, MapPin, MapPinOff, Server, ServerOff};

#[derive(Props, PartialEq, Clone)]
pub struct KeyDetailsProps {
    #[props(into)]
    key_with_meta: ServerWithMeta,
}

#[component]
pub fn KeyDetails(props: KeyDetailsProps) -> Element {
    let key_with_meta = props.key_with_meta;

    let pk = key_with_meta.pk;
    let host = key_with_meta.host;

    let meta = key_with_meta.meta.unwrap_or_default();

    let ServerMeta {
        provider,
        hoster,
        location,
    } = meta;

    rsx! {
        div { class: "flex items-center w-full gap-2",
            HoverCard {
                HoverCardTrigger { class: "",
                    if provider.is_some() {
                        IdCard { class: "" }
                    } else {
                        HatGlasses { class: "" }
                    }
                }
                HoverCardContent { class: "",
                    h3 { class: "font-semibold mb-1", "Provider" }
                    p { class: if provider.is_some() { "" } else { "italic" },
                        {provider.clone().unwrap_or("unknown".to_string())}
                    }
                }
            }

            Separator { class: "w-auto grow" }

            div { class: "flex items-center",
                HoverCard {
                    HoverCardTrigger {
                        Fingerprint { class: "" }
                    }
                    HoverCardContent { class: "break-all",
                        h3 { class: "font-semibold mb-1", "Fingerprint" }
                        p { class: "", "{pk}" }
                    }
                }

                HoverCard {
                    HoverCardTrigger {
                        Link { class: "" }
                    }
                    HoverCardContent { class: "max-w-60 break-all",
                        h3 { class: "font-semibold mb-1", "Link" }
                        p { class: "", "{host}" }
                    }
                }

                HoverCard {
                    HoverCardTrigger { class: "",
                        if hoster.is_some() {
                            Server { class: "" }
                        } else {
                            ServerOff { class: "" }
                        }
                    }
                    HoverCardContent { class: "",
                        h3 { class: "font-semibold mb-1", "Hoster" }
                        p { class: if hoster.is_some() { "" } else { "italic" },
                            {hoster.clone().unwrap_or("unknown".to_string())}
                        }
                    }
                }

                HoverCard {
                    HoverCardTrigger { class: "",
                        if location.is_some() {
                            MapPin { class: "" }
                        } else {
                            MapPinOff { class: "" }
                        }
                    }
                    HoverCardContent { class: "",
                        h3 { class: "font-semibold mb-1", "Location" }
                        p { class: if location.is_some() { "" } else { "italic" },
                            {location.clone().unwrap_or("unknown".to_string())}
                        }
                    }
                }
            }
        }
    }
}
