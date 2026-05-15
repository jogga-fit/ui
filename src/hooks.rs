use dioxus::prelude::*;

/// Returns a signal that is `false` on both SSR and the initial WASM render,
/// then flips to `true` after the component mounts on the client.
pub fn use_client_only() -> Signal<bool> {
    let mut ready = use_signal(|| false);
    use_effect(move || {
        ready.set(true);
    });
    ready
}
