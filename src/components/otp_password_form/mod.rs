#![allow(non_snake_case)]

use dioxus::prelude::*;

#[css_module("/src/components/otp_password_form/style.css")]
struct Styles;

#[derive(Clone, PartialEq)]
enum OtpPhase {
    Enter,
    SetPassword,
}

#[component]
pub fn OtpPasswordForm(
    loading: bool,
    on_submit: EventHandler<(String, String)>,
    submit_label: String,
    loading_label: String,
    password_label: String,
    password2_label: String,
    #[props(optional)] on_resend: Option<EventHandler<()>>,
    /// If provided, called with the OTP code when "Verify code" is clicked.
    /// The parent is responsible for setting `otp_pre_verified = true` on success.
    /// Without this prop the form advances the phase immediately (old behaviour).
    #[props(optional)]
    on_verify_otp: Option<EventHandler<String>>,
    /// Error to display in the Enter phase (e.g. wrong code from server).
    #[props(optional)]
    otp_verify_error: Option<String>,
    /// When true the form skips the Enter phase and shows the password fields.
    #[props(default)]
    otp_pre_verified: bool,
) -> Element {
    let mut otp = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut password2 = use_signal(String::new);
    let mut phase = use_signal(|| OtpPhase::Enter);
    let otp_len = otp.read().len();
    let in_enter_phase = !otp_pre_verified && *phase.read() == OtpPhase::Enter;

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
                                    class: "{Styles::otp_cell}",
                                    class: if i == active_idx && !is_full { "{Styles::otp_cell_active}" },
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
            if let Some(err) = otp_verify_error {
                div { class: "error-banner", "{err}" }
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
                onclick: move |_| {
                    if let Some(verify) = on_verify_otp.as_ref() {
                        verify.call(otp.read().clone());
                    } else {
                        phase.set(OtpPhase::SetPassword);
                    }
                },
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
                onclick: move |_| on_submit.call((password.read().clone(), password2.read().clone())),
                if loading { "{loading_label}" } else { "{submit_label}" }
            }
        }
    }
}
