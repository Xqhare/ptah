use crate::attributes::global::GlobalAttribute;


/// Category: Meta + if `itemprop` attribute is present then phrasing + flow
/// Contexts: Meta + if `itemprop` attribute is present then phrasing
/// Content model: Empty
/// Tag omission: No end tag
/// Content attributes: Global, href, crossorigin, rel, media, integrity, hreflang, type,
/// referrerpolicy, sizes, imagesrcset, imagesizes, as, blocking, color, disabled, fetchpriority,
/// title
/// Spec: https://html.spec.whatwg.org/multipage/semantics.html#the-link-element
///
/// The `<link>` element represents a link to an external resource.
/// Either the `href` or the `imagesrcset` attribute must be present or both.
/// Must have either a `rel` attribute or an `itemprop` attribute but not both.
#[derive(Clone, PartialEq)]
pub struct Link {
    global_attributes: Vec<GlobalAttribute>,
    href: Option<String>,
    crossorigin: Option<Crossorigin>,
    rel: Option<Vec<Rel>>,
    media: Option<String>,
    /// May only be specified if `rel` contains `stylesheet`, `preload` or `modulepreload`
    integrity: Option<String>,
    /// Text must be a valid BCP 47 language tag - ptah does not validate this
    hreflang: Option<String>,
    /// Attribute is called `type` in the spec - that is a keyword though
    /// Text must be a valid MIME type - ptah does not validate this
    type_mime: Option<String>,
    referrerpolicy: Option<ReferrerPolicy>,
    sizes: Option<String>,
    /// Must be a `image candidate string` - ptah does not validate this
    imagesrcset: Option<String>,
    imagesizes: Option<String>,
    as_attribute: Option<String>,
    blocking: Option<String>,
    color: Option<String>,
    disabled: Option<String>,
    fetchpriority: Option<String>,
    title: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum Crossorigin {
    Anonymous,
    Usecredentials,
}

#[derive(Clone, PartialEq)]
pub enum Rel {
    Alternate,
    DnsPrefetch,
    Expect,
    Icon,
    Manifest,
    Modulepreload,
    Next,
    Pingback,
    Preconnect,
    Prefetch,
    Preload,
    Search,
    Stylesheet,
}

#[derive(Clone, PartialEq)]
pub enum ReferrerPolicy {
    Blank,
    NoReferrer,
    NoReferrerWhenDowngrade,
    SameOrigin,
    Origin,
    StrictOrigin,
    OriginWhenCrossOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}
