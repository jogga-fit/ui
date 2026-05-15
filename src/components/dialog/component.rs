use dioxus::prelude::*;
use dioxus_primitives::dialog::{
    self, DialogContentProps, DialogDescriptionProps, DialogRootProps, DialogTitleProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/dialog/style.css")]
struct Styles;

#[component]
pub fn DialogRoot(props: DialogRootProps) -> Element {
    let base = attributes!(div {
        class: format!("{} modal-backdrop", Styles::dx_dialog_backdrop),
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogRoot {
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
    let default_class = format!("{} modal-card", Styles::dx_dialog);
    let class = props
        .class
        .clone()
        .map(|c| format!("{c} modal-card"))
        .unwrap_or(default_class);
    let base = attributes!(div { class });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogContent {
            class: None,
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let base = attributes!(h2 {
        class: Styles::dx_dialog_title,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogTitle {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: Styles::dx_dialog_description,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogDescription {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}
