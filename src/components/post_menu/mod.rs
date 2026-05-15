use dioxus::prelude::*;

#[css_module("/src/components/post_menu/style.css")]
struct Styles;

#[component]
pub fn PostMenu(
    deleting: bool,
    on_delete: EventHandler<()>,
    #[props(default)] on_edit: Option<EventHandler<()>>,
) -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        div { class: "{Styles::post_menu_wrap} post-menu-wrap",
            button {
                class: "{Styles::post_menu_trigger} post-menu-trigger",
                aria_label: "Post actions",
                title: "Post actions",
                disabled: deleting,
                onclick: move |e: MouseEvent| {
                    e.stop_propagation();
                    menu_open.toggle();
                },
                i { class: "ph ph-dots-three" }
            }
            if *menu_open.read() {
                div {
                    class: "{Styles::post_menu_backdrop} post-menu-backdrop",
                    onclick: move |_| menu_open.set(false),
                }
                div { class: "{Styles::post_menu_dropdown} post-menu-dropdown",
                    if let Some(edit_cb) = on_edit {
                        button {
                            class: "{Styles::post_menu_item} post-menu-item",
                            onclick: move |_| {
                                menu_open.set(false);
                                edit_cb.call(());
                            },
                            i { class: "ph ph-pencil" }
                            "Edit"
                        }
                    }
                    button {
                        class: format!("{} post-menu-item {} post-menu-item-danger", Styles::post_menu_item, Styles::post_menu_item_danger),
                        onclick: move |_| {
                            menu_open.set(false);
                            on_delete.call(());
                        },
                        i { class: "ph ph-trash" }
                        "Delete"
                    }
                }
            }
        }
    }
}
