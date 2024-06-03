use strum::{Display, EnumString};

/// LanguageCode
/// LanguageCode based on ISO 639-1.
/// ref: https://www.iso.org/iso-639-language-code
/// ref: https://en.wikipedia.org/wiki/List_of_ISO_639_language_codes
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Display, EnumString)]
pub enum LanguageCode {
    #[strum(serialize = "aa")]
    Aa,
    #[strum(serialize = "ab")]
    Ab,
    #[strum(serialize = "ae")]
    Ae,
    #[strum(serialize = "af")]
    Af,
    #[strum(serialize = "ak")]
    Ak,
    #[strum(serialize = "am")]
    Am,
    #[strum(serialize = "an")]
    An,
    #[strum(serialize = "ar")]
    Ar,
    #[strum(serialize = "as")]
    As,
    #[strum(serialize = "av")]
    Av,
    #[strum(serialize = "ay")]
    Ay,
    #[strum(serialize = "az")]
    Az,
    #[strum(serialize = "ba")]
    Ba,
    #[strum(serialize = "be")]
    Be,
    #[strum(serialize = "bg")]
    Bg,
    #[strum(serialize = "bh")]
    Bh,
    #[strum(serialize = "bi")]
    Bi,
    #[strum(serialize = "bm")]
    Bm,
    #[strum(serialize = "bn")]
    Bn,
    #[strum(serialize = "bo")]
    Bo,
    #[strum(serialize = "br")]
    Br,
    #[strum(serialize = "bs")]
    Bs,
    #[strum(serialize = "ca")]
    Ca,
    #[strum(serialize = "ce")]
    Ce,
    #[strum(serialize = "ch")]
    Ch,
    #[strum(serialize = "co")]
    Co,
    #[strum(serialize = "cr")]
    Cr,
    #[strum(serialize = "cs")]
    Cs,
    #[strum(serialize = "cu")]
    Cu,
    #[strum(serialize = "cv")]
    Cv,
    #[strum(serialize = "cy")]
    Cy,
    #[strum(serialize = "da")]
    Da,
    #[strum(serialize = "de")]
    De,
    #[strum(serialize = "dv")]
    Dv,
    #[strum(serialize = "dz")]
    Dz,
    #[strum(serialize = "ee")]
    Ee,
    #[strum(serialize = "el")]
    El,
    #[strum(serialize = "en")]
    En,
    #[strum(serialize = "eo")]
    Eo,
    #[strum(serialize = "es")]
    Es,
    #[strum(serialize = "et")]
    Et,
    #[strum(serialize = "eu")]
    Eu,
    #[strum(serialize = "fa")]
    Fa,
    #[strum(serialize = "ff")]
    Ff,
    #[strum(serialize = "fi")]
    Fi,
    #[strum(serialize = "fj")]
    Fj,
    #[strum(serialize = "fo")]
    Fo,
    #[strum(serialize = "fr")]
    Fr,
    #[strum(serialize = "fy")]
    Fy,
    #[strum(serialize = "ga")]
    Ga,
    #[strum(serialize = "gd")]
    Gd,
    #[strum(serialize = "gl")]
    Gl,
    #[strum(serialize = "gn")]
    Gn,
    #[strum(serialize = "gu")]
    Gu,
    #[strum(serialize = "gv")]
    Gv,
    #[strum(serialize = "ha")]
    Ha,
    #[strum(serialize = "he")]
    He,
    #[strum(serialize = "hi")]
    Hi,
    #[strum(serialize = "ho")]
    Ho,
    #[strum(serialize = "hr")]
    Hr,
    #[strum(serialize = "ht")]
    Ht,
    #[strum(serialize = "hu")]
    Hu,
    #[strum(serialize = "hy")]
    Hy,
    #[strum(serialize = "hz")]
    Hz,
    #[strum(serialize = "ia")]
    Ia,
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "ie")]
    Ie,
    #[strum(serialize = "ig")]
    Ig,
    #[strum(serialize = "ii")]
    Ii,
    #[strum(serialize = "ik")]
    Ik,
    #[strum(serialize = "in")]
    In,
    #[strum(serialize = "io")]
    Io,
    #[strum(serialize = "is")]
    Is,
    #[strum(serialize = "it")]
    It,
    #[strum(serialize = "iu")]
    Iu,
    #[strum(serialize = "iw")]
    Iw,
    #[strum(serialize = "ja")]
    Ja,
    #[strum(serialize = "ji")]
    Ji,
    #[strum(serialize = "jv")]
    Jv,
    #[strum(serialize = "jw")]
    Jw,
    #[strum(serialize = "ka")]
    Ka,
    #[strum(serialize = "kg")]
    Kg,
    #[strum(serialize = "ki")]
    Ki,
    #[strum(serialize = "kj")]
    Kj,
    #[strum(serialize = "kk")]
    Kk,
    #[strum(serialize = "kl")]
    Kl,
    #[strum(serialize = "km")]
    Km,
    #[strum(serialize = "kn")]
    Kn,
    #[strum(serialize = "ko")]
    Ko,
    #[strum(serialize = "kr")]
    Kr,
    #[strum(serialize = "ks")]
    Ks,
    #[strum(serialize = "ku")]
    Ku,
    #[strum(serialize = "kv")]
    Kv,
    #[strum(serialize = "kw")]
    Kw,
    #[strum(serialize = "ky")]
    Ky,
    #[strum(serialize = "kz")]
    Kz,
    #[strum(serialize = "la")]
    La,
    #[strum(serialize = "lb")]
    Lb,
    #[strum(serialize = "lg")]
    Lg,
    #[strum(serialize = "li")]
    Li,
    #[strum(serialize = "ln")]
    Ln,
    #[strum(serialize = "lo")]
    Lo,
    #[strum(serialize = "ls")]
    Ls,
    #[strum(serialize = "lt")]
    Lt,
    #[strum(serialize = "lu")]
    Lu,
    #[strum(serialize = "lv")]
    Lv,
    #[strum(serialize = "mg")]
    Mg,
    #[strum(serialize = "mh")]
    Mh,
    #[strum(serialize = "mi")]
    Mi,
    #[strum(serialize = "mk")]
    Mk,
    #[strum(serialize = "ml")]
    Ml,
    #[strum(serialize = "mn")]
    Mn,
    #[strum(serialize = "mo")]
    Mo,
    #[strum(serialize = "mr")]
    Mr,
    #[strum(serialize = "ms")]
    Ms,
    #[strum(serialize = "mt")]
    Mt,
    #[strum(serialize = "my")]
    My,
    #[strum(serialize = "na")]
    Na,
    #[strum(serialize = "nb")]
    Nb,
    #[strum(serialize = "nd")]
    Nd,
    #[strum(serialize = "ne")]
    Ne,
    #[strum(serialize = "ng")]
    Ng,
    #[strum(serialize = "nl")]
    Nl,
    #[strum(serialize = "nn")]
    Nn,
    #[strum(serialize = "no")]
    No,
    #[strum(serialize = "nr")]
    Nr,
    #[strum(serialize = "ns")]
    Ns,
    #[strum(serialize = "nv")]
    Nv,
    #[strum(serialize = "ny")]
    Ny,
    #[strum(serialize = "oc")]
    Oc,
    #[strum(serialize = "oj")]
    Oj,
    #[strum(serialize = "om")]
    Om,
    #[strum(serialize = "or")]
    Or,
    #[strum(serialize = "os")]
    Os,
    #[strum(serialize = "pa")]
    Pa,
    #[strum(serialize = "pi")]
    Pi,
    #[strum(serialize = "pl")]
    Pl,
    #[strum(serialize = "ps")]
    Ps,
    #[strum(serialize = "pt")]
    Pt,
    #[strum(serialize = "qu")]
    Qu,
    #[strum(serialize = "rm")]
    Rm,
    #[strum(serialize = "rn")]
    Rn,
    #[strum(serialize = "ro")]
    Ro,
    #[strum(serialize = "ru")]
    Ru,
    #[strum(serialize = "rw")]
    Rw,
    #[strum(serialize = "sa")]
    Sa,
    #[strum(serialize = "sb")]
    Sb,
    #[strum(serialize = "sc")]
    Sc,
    #[strum(serialize = "sd")]
    Sd,
    #[strum(serialize = "se")]
    Se,
    #[strum(serialize = "sg")]
    Sg,
    #[strum(serialize = "sh")]
    Sh,
    #[strum(serialize = "si")]
    Si,
    #[strum(serialize = "sk")]
    Sk,
    #[strum(serialize = "sl")]
    Sl,
    #[strum(serialize = "sm")]
    Sm,
    #[strum(serialize = "sn")]
    Sn,
    #[strum(serialize = "so")]
    So,
    #[strum(serialize = "sq")]
    Sq,
    #[strum(serialize = "sr")]
    Sr,
    #[strum(serialize = "ss")]
    Ss,
    #[strum(serialize = "st")]
    St,
    #[strum(serialize = "su")]
    Su,
    #[strum(serialize = "sv")]
    Sv,
    #[strum(serialize = "sw")]
    Sw,
    #[strum(serialize = "sx")]
    Sx,
    #[strum(serialize = "ta")]
    Ta,
    #[strum(serialize = "te")]
    Te,
    #[strum(serialize = "tg")]
    Tg,
    #[strum(serialize = "th")]
    Th,
    #[strum(serialize = "ti")]
    Ti,
    #[strum(serialize = "tk")]
    Tk,
    #[strum(serialize = "tl")]
    Tl,
    #[strum(serialize = "tn")]
    Tn,
    #[strum(serialize = "to")]
    To,
    #[strum(serialize = "tr")]
    Tr,
    #[strum(serialize = "ts")]
    Ts,
    #[strum(serialize = "tt")]
    Tt,
    #[strum(serialize = "tw")]
    Tw,
    #[strum(serialize = "ty")]
    Ty,
    #[strum(serialize = "ug")]
    Ug,
    #[strum(serialize = "uk")]
    Uk,
    #[strum(serialize = "ur")]
    Ur,
    #[strum(serialize = "us")]
    Us,
    #[strum(serialize = "uz")]
    Uz,
    #[strum(serialize = "ve")]
    Ve,
    #[strum(serialize = "vi")]
    Vi,
    #[strum(serialize = "vo")]
    Vo,
    #[strum(serialize = "wa")]
    Wa,
    #[strum(serialize = "wo")]
    Wo,
    #[strum(serialize = "xh")]
    Xh,
    #[strum(serialize = "yi")]
    Yi,
    #[strum(serialize = "yo")]
    Yo,
    #[strum(serialize = "za")]
    Za,
    #[strum(serialize = "zh")]
    Zh,
    #[strum(serialize = "zu")]
    Zu,
}
