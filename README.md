# jogga-ui

Component library for jogga.fit built with [Dioxus](https://dioxuslabs.com)

## What's in here

`src/` contains the full component library: feeds, profiles, exercise cards, maps, modals, auth flows, settings, and more. Components are designed for Jogga's UI but are structured to be usable in any Dioxus app.

## Getting Started

Mount styles once near the root of your app:

```rust
use jogga_ui::UiStyles;

rsx! {
    UiStyles {}
    // your app here
}
```

Then import what you need:

```rust
use jogga_ui::{FeedCard, ProfilePageView, ToastProvider};
```

> NOTE: Components that need data (feeds, profiles, routes) receive server functions as props rather than calling them directly. This keeps the library decoupled from any specific backend.

```rust
rsx! {
    HomePageView {
        get_feed: GetFeedFn(my_get_feed_server_fn),
        like: LikeFn(my_like_server_fn),
        // ...
    }
}
```

## Demo

```bash
cd demo
dx serve # install dioxus-cli 
```

Opens at `http://localhost:8080` by default.

## Requirements

- Rust 1.85+
- [Dioxus CLI](https://github.com/DioxusLabs/dioxus) (`cargo install dioxus-cli`)

## License

See [LICENSE](LICENSE).
