/// TODO: Attribute macro
macro_rules! declare_enum {
    (NAME: $name:ident, $($item:ident=$lit:literal),* $(,)?) => {
        #[derive(Debug, Clone, Copy)]
        #[repr(u8)]
        pub enum $name {
            $($item,)*
        }

        const LOOKUP_TABLE: &[&str] = &[$($lit,)*];

        impl $name {
            #[inline]
            pub fn raw(&self) -> &str {
                LOOKUP_TABLE[*self as usize]
            }
        }
    };
}

macro_rules! unit {
    (Ident: $ident:expr) => {
        Unit::Ident(Ident::new($ident))
    };
    // Punctuator
    ("!") => {
        Unit::Punc(Punc::Not)
    };
    ("!=") => {
        Unit::Punc(Punc::NotEq)
    };
    ("!==") => {
        Unit::Punc(Punc::StrictNotEq)
    };
    ("#") => {
        Unit::Punc(Punc::Hash)
    };
    ("#!") => {
        Unit::Punc(Punc::Hashbang)
    };
    ("%") => {
        Unit::Punc(Punc::Mod)
    };
    ("%=") => {
        Unit::Punc(Punc::ModAssign)
    };
    ("&") => {
        Unit::Punc(Punc::BitAnd)
    };
    ("&=") => {
        Unit::Punc(Punc::BitAndAssign)
    };
    ("&&") => {
        Unit::Punc(Punc::LogicAnd)
    };
    ("&&=") => {
        Unit::Punc(Punc::LogicAndAssign)
    };
    ("(") => {
        Unit::Punc(Punc::LParen)
    };
    (")") => {
        Unit::Punc(Punc::RParen)
    };
    ("*") => {
        Unit::Punc(Punc::Mul)
    };
    ("*=") => {
        Unit::Punc(Punc::MulAssign)
    };
    ("**") => {
        Unit::Punc(Punc::Pow)
    };
    ("**=") => {
        Unit::Punc(Punc::PowAssign)
    };
    ("+") => {
        Unit::Punc(Punc::Add)
    };
    ("+=") => {
        Unit::Punc(Punc::AddAssign)
    };
    ("++") => {
        Unit::Punc(Punc::BitInc)
    };
    (",") => {
        Unit::Punc(Punc::Comma)
    };
    ("-") => {
        Unit::Punc(Punc::Sub)
    };
    ("-=") => {
        Unit::Punc(Punc::SubAssign)
    };
    ("--") => {
        Unit::Punc(Punc::BitDec)
    };
    (".") => {
        Unit::Punc(Punc::Dot)
    };
    ("...") => {
        Unit::Punc(Punc::Ellipsis)
    };
    ("/") => {
        Unit::Punc(Punc::Div)
    };
    ("/=") => {
        Unit::Punc(Punc::DivAssign)
    };
    (":") => {
        Unit::Punc(Punc::Colon)
    };
    (";") => {
        Unit::Punc(Punc::Semi)
    };
    ("<") => {
        Unit::Punc(Punc::Lt)
    };
    ("<=") => {
        Unit::Punc(Punc::Le)
    };
    ("<<") => {
        Unit::Punc(Punc::BitSal)
    };
    ("<<=") => {
        Unit::Punc(Punc::BitSalAssign)
    };
    ("=") => {
        Unit::Punc(Punc::Assign)
    };
    ("=>") => {
        Unit::Punc(Punc::Arrow)
    };
    ("==") => {
        Unit::Punc(Punc::Eq)
    };
    ("===") => {
        Unit::Punc(Punc::StrictEq)
    };
    (">") => {
        Unit::Punc(Punc::Gt)
    };
    (">=") => {
        Unit::Punc(Punc::Ge)
    };
    (">>") => {
        Unit::Punc(Punc::BitSar)
    };
    (">>=") => {
        Unit::Punc(Punc::BitSarAssign)
    };
    (">>>") => {
        Unit::Punc(Punc::BitShr)
    };
    (">>>=") => {
        Unit::Punc(Punc::BitShrAssign)
    };
    ("?") => {
        Unit::Punc(Punc::Question)
    };
    ("?.") => {
        Unit::Punc(Punc::OptionalChain)
    };
    ("??") => {
        Unit::Punc(Punc::Coalesce)
    };
    ("??=") => {
        Unit::Punc(Punc::CoalesceAssign)
    };
    ("@") => {
        Unit::Punc(Punc::At)
    };
    ("[") => {
        Unit::Punc(Punc::LBracket)
    };
    ("]") => {
        Unit::Punc(Punc::RBracket)
    };
    ("^") => {
        Unit::Punc(Punc::BitXor)
    };
    ("^=") => {
        Unit::Punc(Punc::BitXorAssign)
    };
    ("{") => {
        Unit::Punc(Punc::LBrace)
    };
    ("}") => {
        Unit::Punc(Punc::RBrace)
    };
    ("|") => {
        Unit::Punc(Punc::BitOr)
    };
    ("|=") => {
        Unit::Punc(Punc::BitOrAssign)
    };
    ("||") => {
        Unit::Punc(Punc::LogicOr)
    };
    ("||=") => {
        Unit::Punc(Punc::LogicOrAssign)
    };
    ("~") => {
        Unit::Punc(Punc::BitNot)
    };

    // Keyword
    ("await") => {
        Unit::Keyword(Keyword::Await)
    };
    ("break") => {
        Unit::Keyword(Keyword::Break)
    };
    ("case") => {
        Unit::Keyword(Keyword::Case)
    };
    ("catch") => {
        Unit::Keyword(Keyword::Catch)
    };
    ("class") => {
        Unit::Keyword(Keyword::Class)
    };
    ("const") => {
        Unit::Keyword(Keyword::Const)
    };
    ("continue") => {
        Unit::Keyword(Keyword::Continue)
    };
    ("debugger") => {
        Unit::Keyword(Keyword::Debugger)
    };
    ("default") => {
        Unit::Keyword(Keyword::Default)
    };
    ("delete") => {
        Unit::Keyword(Keyword::Delete)
    };
    ("do") => {
        Unit::Keyword(Keyword::Do)
    };
    ("else") => {
        Unit::Keyword(Keyword::Else)
    };
    ("enum") => {
        Unit::Keyword(Keyword::Enum)
    };
    ("export") => {
        Unit::Keyword(Keyword::Export)
    };
    ("extends") => {
        Unit::Keyword(Keyword::Extends)
    };
    ("false") => {
        Unit::Keyword(Keyword::False)
    };
    ("finally") => {
        Unit::Keyword(Keyword::Finally)
    };
    ("for") => {
        Unit::Keyword(Keyword::For)
    };
    ("function") => {
        Unit::Keyword(Keyword::Function)
    };
    ("if") => {
        Unit::Keyword(Keyword::If)
    };
    ("import") => {
        Unit::Keyword(Keyword::Import)
    };
    ("in") => {
        Unit::Keyword(Keyword::In)
    };
    ("instanceof") => {
        Unit::Keyword(Keyword::Instanceof)
    };
    ("let") => {
        Unit::Keyword(Keyword::Let)
    };
    ("new") => {
        Unit::Keyword(Keyword::New)
    };
    ("null") => {
        Unit::Keyword(Keyword::Null)
    };
    ("return") => {
        Unit::Keyword(Keyword::Return)
    };
    ("super") => {
        Unit::Keyword(Keyword::Super)
    };
    ("switch") => {
        Unit::Keyword(Keyword::Switch)
    };
    ("this") => {
        Unit::Keyword(Keyword::This)
    };
    ("throw") => {
        Unit::Keyword(Keyword::Throw)
    };
    ("true") => {
        Unit::Keyword(Keyword::True)
    };
    ("try") => {
        Unit::Keyword(Keyword::Try)
    };
    ("typeof") => {
        Unit::Keyword(Keyword::Typeof)
    };
    ("var") => {
        Unit::Keyword(Keyword::Var)
    };
    ("void") => {
        Unit::Keyword(Keyword::Void)
    };
    ("while") => {
        Unit::Keyword(Keyword::While)
    };
    ("with") => {
        Unit::Keyword(Keyword::With)
    };
    ("yield") => {
        Unit::Keyword(Keyword::Yield)
    };
}
