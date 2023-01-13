pub enum BodyProperty{
    OnAfterPrint,
    OnBeforeUnload,
    OnBlur,
    OnError,
    OnFocus,
    OnHashChange,
    OnLanguageChange,
    OnLoad,
    OnMessage,
    OnOffline,
    OnOnline,
    OnPopState,
    OnRedo,
    OnResize,
    OnStorage,
    OnUndo,
}

impl BodyProperty{
    fn as_str(&self) -> &'static str{
        match self{
            BodyProperty::OnAfterPrint => "onafterprint",
            BodyProperty::OnBeforeUnload => "onbeforeunload",
            BodyProperty::OnBlur => "onblur",
            BodyProperty::OnError => "onerror",
            BodyProperty::OnFocus => "onfocus",
            BodyProperty::OnHashChange => "onhashchange",
            BodyProperty::OnLanguageChange => "onlanguagechange",
            BodyProperty::OnLoad => "onload",
            BodyProperty::OnMessage => "onmessage",
            BodyProperty::OnOffline => "onoffline",
            BodyProperty::OnOnline => "ononline",
            BodyProperty::OnPopState => "onpopstate",
            BodyProperty::OnRedo => "onredo",
            BodyProperty::OnResize => "onresize",
            BodyProperty::OnStorage => "onstorage",
            BodyProperty::OnUndo => "onundo",
        }
    }
}