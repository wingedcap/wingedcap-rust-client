use dioxus::prelude::*;

use crate::types::*;

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
        div { class: "flex items-center w-full gap-2.5",
            HoverCard {
                HoverCardTrigger { class: "shrink-0",
                    if provider.is_some() {
                        IdCard { class: "" }
                    } else {
                        HatGlasses { class: "text-muted-foreground" }
                    }
                }
                HoverCardContent { class: "", side: Side::Left,
                    h3 { class: "font-semibold mb-1.5 text-xs uppercase tracking-wide",
                        "Provider"
                    }
                    p { class: if provider.is_some() { "" } else { "italic opacity-70" },
                        {provider.clone().unwrap_or("unknown".to_string())}
                    }
                }
            }

            Separator { class: "w-auto grow" }

            div { class: "flex items-center gap-1",
                HoverCard {
                    HoverCardTrigger {
                        Fingerprint { class: "" }
                    }
                    HoverCardContent { class: "break-all", side: Side::Top,
                        h3 { class: "font-semibold mb-1.5 text-xs uppercase tracking-wide",
                            "Fingerprint"
                        }
                        p { class: "font-mono text-xs", "{pk}" }
                    }
                }

                HoverCard {
                    HoverCardTrigger {
                        Link { class: "" }
                    }
                    HoverCardContent { class: "break-all", side: Side::Top,
                        h3 { class: "font-semibold mb-1.5 text-xs uppercase tracking-wide",
                            "Link"
                        }
                        p { class: "text-xs", "{host}" }
                    }
                }

                HoverCard {
                    HoverCardTrigger { class: "",
                        if hoster.is_some() {
                            Server { class: "" }
                        } else {
                            ServerOff { class: "text-muted-foreground" }
                        }
                    }
                    HoverCardContent { class: "",
                        h3 { class: "font-semibold mb-1.5 text-xs uppercase tracking-wide",
                            "Hoster"
                        }
                        p { class: if hoster.is_some() { "" } else { "italic opacity-70" },
                            {hoster.clone().unwrap_or("unknown".to_string())}
                        }
                    }
                }

                HoverCard {
                    HoverCardTrigger { class: "",
                        if location.is_some() {
                            MapPin { class: "" }
                        } else {
                            MapPinOff { class: "text-muted-foreground" }
                        }
                    }
                    HoverCardContent { class: "",
                        h3 { class: "font-semibold mb-1.5 text-xs uppercase tracking-wide",
                            "Location"
                        }
                        p { class: if location.is_some() { "" } else { "italic opacity-70" },
                            {location.clone().unwrap_or("unknown".to_string())}
                        }
                    }
                }
            }
        }
    }
}
