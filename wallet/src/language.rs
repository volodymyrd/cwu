use bip39::Language;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub(super) struct Lang {
    pub(super) lang: Language,
}

impl FromStr for Lang {
    type Err = InvalidLanguage;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("en") || s.eq_ignore_ascii_case("english") {
            return Ok(Lang {
                lang: Language::English,
            });
        }
        if s.eq_ignore_ascii_case("zh-cn") || s.eq_ignore_ascii_case("chinese-simplified") {
            return Ok(Lang {
                lang: Language::SimplifiedChinese,
            });
        }
        if s.eq_ignore_ascii_case("zh-tw") || s.eq_ignore_ascii_case("chinese-traditional") {
            return Ok(Lang {
                lang: Language::TraditionalChinese,
            });
        }
        if s.eq_ignore_ascii_case("cs") || s.eq_ignore_ascii_case("czech") {
            return Ok(Lang {
                lang: Language::Czech,
            });
        }
        if s.eq_ignore_ascii_case("fr") || s.eq_ignore_ascii_case("french") {
            return Ok(Lang {
                lang: Language::French,
            });
        }
        if s.eq_ignore_ascii_case("it") || s.eq_ignore_ascii_case("italian") {
            return Ok(Lang {
                lang: Language::Italian,
            });
        }
        if s.eq_ignore_ascii_case("ja") || s.eq_ignore_ascii_case("japanese") {
            return Ok(Lang {
                lang: Language::Japanese,
            });
        }
        if s.eq_ignore_ascii_case("ko") || s.eq_ignore_ascii_case("korean") {
            return Ok(Lang {
                lang: Language::Korean,
            });
        }
        if s.eq_ignore_ascii_case("pt") || s.eq_ignore_ascii_case("portuguese") {
            return Ok(Lang {
                lang: Language::Portuguese,
            });
        }
        if s.eq_ignore_ascii_case("es") || s.eq_ignore_ascii_case("spanish") {
            return Ok(Lang {
                lang: Language::Spanish,
            });
        }
        Err(InvalidLanguage)
    }
}

#[derive(Debug)]
pub struct InvalidLanguage;

impl Display for InvalidLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid language")
    }
}

impl std::error::Error for InvalidLanguage {}
