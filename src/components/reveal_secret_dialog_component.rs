use dioxus::prelude::*;

use lucide_dioxus::Eye;

use crate::ui::{
    card::{CardDescription, CardHeader, CardTitle},
    modal::{Modal, ModalBackground, ModalContent, ModalTrigger},
};

#[derive(PartialEq, Props, Clone)]
pub struct RevealSecretDialogProps {
    pub decrypted_messages: Vec<String>,
}

#[component]
pub fn RevealSecretDialog(
    RevealSecretDialogProps { decrypted_messages }: RevealSecretDialogProps,
) -> Element {
    let decrypted_message = decrypted_messages.first();

    rsx! {
        Modal {
            ModalTrigger { class: "px-0 pr-0 flex items-center justify-center border-none shadow-none",
                Eye { class: "stroke-primary animate-fade-in" }
            }

            ModalBackground {}

            ModalContent {
                if let Some(decrypted_message) = decrypted_message {
                    CardHeader { class: "mb-2",
                        CardTitle { "Secret unlocked" }
                        CardDescription { "You can finally read it !" }
                    }

                    div { class: "max-h-[50vh] rounded-md border border-dashed p-2 text-sm",
                        "{decrypted_message}"
                    }
                } else {
                    CardHeader { class: "mb-2",
                        CardTitle { "Failed to decrypt your secret" }
                        CardDescription { "This should never happen, please contact support." }
                    }
                }
            }
        }
    }
}
