#![allow(non_snake_case)]

use dioxus::prelude::*;

#[css_module("/src/components/otp_password_form.css")]
struct Styles;

#[derive(Clone, PartialEq)]
enum OtpPhase {
    Enter,
    SetPassword,
}

#[component]
pub fn OtpPasswordForm(
    mut otp: Signal<String>,
    mut password: Signal<String>,
    mut password2: Signal<String>,
    loading: bool,
    on_submit: EventHandler<Event<MouseData>>,
    submit_label: String,
    loading_label: String,
    password_label: String,
    password2_label: String,
    #[props(optional)] on_resend: Option<EventHandler<()>>,
) -> Element {
    let mut phase = use_signal(|| OtpPhase::Enter);
    let otp_len = otp.read().len();
    let in_enter_phase = *phase.read() == OtpPhase::Enter;

    rsx! {
        if in_enter_phase {
            div { class: "form-group",
                label { r#for: "otp-hidden", "Verification code" }
                label { class: Styles::otp_boxes,
                    {
                        let len = otp.read().len();
                        let active_idx: usize = len.min(5);
                        let is_full = len >= 6;
                        rsx! {
                            for i in 0..6usize {
                                div {
                                    key: "{i}",
                                    class: if i == active_idx && !is_full {
                                        format!("{} {}", Styles::otp_cell, Styles::otp_cell_active)
                                    } else {
                                        Styles::otp_cell.to_string()
                                    },
                                    { otp.read().chars().nth(i).map(|c| c.to_string()).unwrap_or_default() }
                                }
                            }
                        }
                    }
                    input {
                        id: "otp-hidden",
                        class: Styles::otp_hidden_input,
                        r#type: "text",
                        inputmode: "numeric",
                        autocomplete: "one-time-code",
                        value: "{otp}",
                        oninput: move |e| {
                            let filtered: String = e
                                .value()
                                .chars()
                                .filter(|c| c.is_ascii_digit())
                                .take(6)
                                .collect();
                            otp.set(filtered);
                        },
                    }
                }
            }
        }

        if in_enter_phase {
            if let Some(resend) = on_resend {
                div { class: "auth-footer",
                    button {
                        class: "btn-link",
                        onclick: move |_| resend.call(()),
                        "Resend code"
                    }
                }
            }
            button {
                class: "btn btn-primary btn-full",
                disabled: otp_len < 6,
                onclick: move |_| phase.set(OtpPhase::SetPassword),
                "Verify code"
            }
        } else {
            div { class: "form-group",
                label { r#for: "pwd", "{password_label}" }
                input {
                    id: "pwd",
                    r#type: "password",
                    placeholder: "At least 8 characters",
                    autocomplete: "new-password",
                    value: "{password}",
                    oninput: move |e| password.set(e.value()),
                }
                span { class: "form-hint", "Make it hard to guess." }
            }
            div { class: "form-group",
                label { r#for: "pwd2", "{password2_label}" }
                input {
                    id: "pwd2",
                    r#type: "password",
                    placeholder: "Same password again",
                    autocomplete: "new-password",
                    value: "{password2}",
                    oninput: move |e| password2.set(e.value()),
                }
            }
            button {
                class: "btn btn-primary btn-full",
                disabled: loading,
                onclick: on_submit,
                if loading { "{loading_label}" } else { "{submit_label}" }
            }
        }
    }
}
