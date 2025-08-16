#[derive(Clone)]
pub enum GlobalAttribute {
    Accesskey(String),
    Autocapitalize(Autocapitalize),
    Autofocus,
    Class(Vec<String>),
    Contenteditable(Contenteditable),
    Data(Vec<(String, String)>),
    Dir(Dir),
    Draggable(bool),
    Enterkeyhint(Enterkeyhint),
    Exportparts(Vec<String>),
    Hidden,
    /// MUST be document unique
    Id(String),
    Inert,
    Inputmode(Inputmode),
    Is(String),
    Itemid(String),
    Itemprop(String),
    Itemref(Vec<String>),
    Itemscope,
    Itemtype(String),
    Lang(String),
    Nonce(String),
    Part(Vec<String>),
    Popover(Popover),
    Role(String),
    Slot(String),
    Spellcheck(bool),
    /// Do not use if you are using `CSS` files
    Style(String),
    /// It is discuraged by the doc to use values > 0
    Tabindex(i16),
    Title(String),
    Translate(Translate),
    Virtualkeyboardpolicy(Virtualkeyboardpolicy),
    Writingsuggestions(bool),
}

#[derive(Clone)]
pub enum Virtualkeyboardpolicy {
    Auto,
    Manual,
}

#[derive(Clone)]
pub enum Translate {
    Yes,
    No,
}

#[derive(Clone)]
pub enum Popover {
    Auto,
    Manual,
}

#[derive(Clone)]
pub enum Inputmode {
    None,
    Text,
    Decimal,
    Numeric,
    Tel,
    Search,
    Email,
    Url,
}

#[derive(Clone)]
pub enum Enterkeyhint {
    Enter,
    Done,
    Go,
    Next,
    Previous,
    Search,
    Send,
}

#[derive(Clone)]
pub enum Dir {
    Auto,
    Ltr,
    Rtl,
}

#[derive(Clone)]
pub enum Contenteditable {
    True,
    False,
    PlaintextOnly,
}

#[derive(Clone)]
pub enum Autocapitalize {
    Off,
    None,
    On,
    Sentences,
    Words,
    Characters,
}
