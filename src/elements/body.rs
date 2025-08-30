use crate::attributes::global::GlobalAttribute;

use super::FlowContent;


#[derive(Clone)]
pub struct Body {
    global_attributes: Vec<GlobalAttribute>,
    children: Vec<FlowContent>,
    // All of these take JavaScript as a string
    on_after_print: Option<String>,
    on_before_print: Option<String>,
    on_before_unload: Option<String>,
    on_hash_change: Option<String>,
    on_language_change: Option<String>,
    on_message: Option<String>,
    on_message_error: Option<String>,
    on_offline: Option<String>,
    on_online: Option<String>,
    on_page_swap: Option<String>,
    on_page_hide: Option<String>,
    on_page_reveal: Option<String>,
    on_page_show: Option<String>,
    on_pop_state: Option<String>,
    on_rejection_handled: Option<String>,
    on_storage: Option<String>,
    on_unhandled_rejection: Option<String>,
    on_unload: Option<String>,
}
