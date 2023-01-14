pub mod body;
pub mod global;
pub mod html;
pub mod meta;
pub mod td;
pub mod th;
pub mod li;
pub mod ol;

use body::BodyProperty;
use global::GlobalProperty;
use html::HtmlProperty;
use meta::MetaProperty;
use td::TD;
use th::TH;
use li::LI;
use ol::OL;

pub trait PropertyInfo{
    fn as_str(&self) -> &'static str;
}

impl PropertyInfo for BodyProperty{
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

impl PropertyInfo for GlobalProperty{
    fn as_str(&self) -> &'static str{
        match self{
            GlobalProperty::AccessKey => "accesskey",
            GlobalProperty::AutoCapitalise => "autocapitalise",
            GlobalProperty::AutoFocus => "autofocus",
            GlobalProperty::Class => "class",
            GlobalProperty::ContendedTable => "contendedtable",
            GlobalProperty::Data => "data",
            GlobalProperty::Dir => "dir",
            GlobalProperty::Draggable => "draggable",
            GlobalProperty::EnterKeyHint => "enterkeyhint",
            GlobalProperty::ExportParts => "exportparts",
            GlobalProperty::Hidden => "hidden",
            GlobalProperty::Id => "id",
            GlobalProperty::InputMode => "inputmode",
            GlobalProperty::Is => "is",
            GlobalProperty::ItemProp => "itemprop",
            GlobalProperty::ItemRef => "itemref",
            GlobalProperty::ItemScope => "iterscope",
            GlobalProperty::ItemType => "itemtype",
            GlobalProperty::Lang => "lang",
            GlobalProperty::Nonce => "nonce",
            GlobalProperty::Part => "part",
            GlobalProperty::Role => "role",
            GlobalProperty::Slot => "slot",
            GlobalProperty::SpellCheck => "spellcheck",
            GlobalProperty::Style => "style",
            GlobalProperty::TabIndex => "tabindex",
            GlobalProperty::Title => "title",
            GlobalProperty::Translate => "translate",
        }
    }
}

impl PropertyInfo for HtmlProperty {
    fn as_str(&self) -> &'static str{
        match self{
            HtmlProperty::Xmlns => "xmlns",
        }
    }
}

impl PropertyInfo for LI {
    fn as_str(&self) -> &'static str{
        match self{
            LI::Value => "li",
        }
    }
}

impl PropertyInfo for MetaProperty{
    fn as_str(&self) -> &'static str{
        match self{
            MetaProperty::Charset => "charset",
            MetaProperty::HttpEquiv => "http-equiv",
            MetaProperty::Content => "content",
        }
    }
}

impl PropertyInfo for OL{
    fn as_str(&self) -> &'static str{
        match self{
            OL::Reserved => "reserved",
            OL::Start => "start",
            OL::Type => "type",
        }
    }
}

impl PropertyInfo for TD{
    fn as_str(&self) -> &'static str{
        match self{
            TD::ColSpan => "colspan",
            TD::RowSpan => "rowspan",
            TD::Headers => "headers",
        }
    }
}

impl PropertyInfo for TH{
    fn as_str(&self) ->&'static str{
        match self{
            TH::Abbr => "abbr",
            TH::Headers => "headers",
            TH::RowSpan => "rowspan",
            TH::Scope => "scope",
            TH::ColSpan => "colspan",
        }
    }
}