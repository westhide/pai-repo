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
        Unit::Punctuator(Punctuator::Not)
    };
    ("!=") => {
        Unit::Punctuator(Punctuator::NotEq)
    };
    ("!==") => {
        Unit::Punctuator(Punctuator::StrictNotEq)
    };
    ("#") => {
        Unit::Punctuator(Punctuator::Hash)
    };
    ("#!") => {
        Unit::Punctuator(Punctuator::Hashbang)
    };
    ("%") => {
        Unit::Punctuator(Punctuator::Mod)
    };
    ("%=") => {
        Unit::Punctuator(Punctuator::ModAssign)
    };
    ("&") => {
        Unit::Punctuator(Punctuator::BitAnd)
    };
    ("&=") => {
        Unit::Punctuator(Punctuator::BitAndAssign)
    };
    ("&&") => {
        Unit::Punctuator(Punctuator::LogicAnd)
    };
    ("&&=") => {
        Unit::Punctuator(Punctuator::LogicAndAssign)
    };
    ("(") => {
        Unit::Punctuator(Punctuator::LParen)
    };
    (")") => {
        Unit::Punctuator(Punctuator::RParen)
    };
    ("*") => {
        Unit::Punctuator(Punctuator::Mul)
    };
    ("*=") => {
        Unit::Punctuator(Punctuator::MulAssign)
    };
    ("**") => {
        Unit::Punctuator(Punctuator::Pow)
    };
    ("**=") => {
        Unit::Punctuator(Punctuator::PowAssign)
    };
    ("+") => {
        Unit::Punctuator(Punctuator::Add)
    };
    ("+=") => {
        Unit::Punctuator(Punctuator::AddAssign)
    };
    ("++") => {
        Unit::Punctuator(Punctuator::BitInc)
    };
    (",") => {
        Unit::Punctuator(Punctuator::Comma)
    };
    ("-") => {
        Unit::Punctuator(Punctuator::Sub)
    };
    ("-=") => {
        Unit::Punctuator(Punctuator::SubAssign)
    };
    ("--") => {
        Unit::Punctuator(Punctuator::BitDec)
    };
    (".") => {
        Unit::Punctuator(Punctuator::Dot)
    };
    ("...") => {
        Unit::Punctuator(Punctuator::Ellipsis)
    };
    ("/") => {
        Unit::Punctuator(Punctuator::Div)
    };
    ("/=") => {
        Unit::Punctuator(Punctuator::DivAssign)
    };
    (":") => {
        Unit::Punctuator(Punctuator::Colon)
    };
    (";") => {
        Unit::Punctuator(Punctuator::Semi)
    };
    ("<") => {
        Unit::Punctuator(Punctuator::Lt)
    };
    ("<=") => {
        Unit::Punctuator(Punctuator::Le)
    };
    ("<<") => {
        Unit::Punctuator(Punctuator::BitSal)
    };
    ("<<=") => {
        Unit::Punctuator(Punctuator::BitSalAssign)
    };
    ("=") => {
        Unit::Punctuator(Punctuator::Assign)
    };
    ("=>") => {
        Unit::Punctuator(Punctuator::Arrow)
    };
    ("==") => {
        Unit::Punctuator(Punctuator::Eq)
    };
    ("===") => {
        Unit::Punctuator(Punctuator::StrictEq)
    };
    (">") => {
        Unit::Punctuator(Punctuator::Gt)
    };
    (">=") => {
        Unit::Punctuator(Punctuator::Ge)
    };
    (">>") => {
        Unit::Punctuator(Punctuator::BitSar)
    };
    (">>=") => {
        Unit::Punctuator(Punctuator::BitSarAssign)
    };
    (">>>") => {
        Unit::Punctuator(Punctuator::BitShr)
    };
    (">>>=") => {
        Unit::Punctuator(Punctuator::BitShrAssign)
    };
    ("?") => {
        Unit::Punctuator(Punctuator::Question)
    };
    ("?.") => {
        Unit::Punctuator(Punctuator::OptionalChain)
    };
    ("??") => {
        Unit::Punctuator(Punctuator::Coalesce)
    };
    ("??=") => {
        Unit::Punctuator(Punctuator::CoalesceAssign)
    };
    ("@") => {
        Unit::Punctuator(Punctuator::At)
    };
    ("[") => {
        Unit::Punctuator(Punctuator::LBracket)
    };
    ("]") => {
        Unit::Punctuator(Punctuator::RBracket)
    };
    ("^") => {
        Unit::Punctuator(Punctuator::BitXor)
    };
    ("^=") => {
        Unit::Punctuator(Punctuator::BitXorAssign)
    };
    ("{") => {
        Unit::Punctuator(Punctuator::LBrace)
    };
    ("}") => {
        Unit::Punctuator(Punctuator::RBrace)
    };
    ("|") => {
        Unit::Punctuator(Punctuator::BitOr)
    };
    ("|=") => {
        Unit::Punctuator(Punctuator::BitOrAssign)
    };
    ("||") => {
        Unit::Punctuator(Punctuator::LogicOr)
    };
    ("||=") => {
        Unit::Punctuator(Punctuator::LogicOrAssign)
    };
    ("~") => {
        Unit::Punctuator(Punctuator::BitNot)
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
