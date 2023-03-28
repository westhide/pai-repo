// [ECMA Keyword][1]
//
// [1]:https://tc39.es/ecma262/#sec-keywords-and-reserved-words

declare_enum![
    NAME: Keyword,
    Await = "await",
    Break = "break",
    Case = "case",
    Catch = "catch",
    Class = "class",
    Const = "const",
    Continue = "continue",
    Debugger = "debugger",
    Default = "default",
    Delete = "delete",
    Do = "do",
    Else = "else",
    Enum = "enum",
    Export = "export",
    Extends = "extends",
    False = "false",
    Finally = "finally",
    For = "for",
    Function = "function",
    If = "if",
    Import = "import",
    In = "in",
    Instanceof = "instanceof",
    Let = "let",
    New = "new",
    Null = "null",
    Return = "return",
    Super = "super",
    Switch = "switch",
    This = "this",
    Throw = "throw",
    True = "true",
    Try = "try",
    Typeof = "typeof",
    Var = "var",
    Void = "void",
    While = "while",
    With = "with",
    Yield = "yield",
];

impl Keyword {}

pub const AWAIT: &str = "await";
pub const BREAK: &str = "break";
pub const CASE: &str = "case";
pub const CATCH: &str = "catch";
pub const CLASS: &str = "class";
pub const CONST: &str = "const";
pub const CONTINUE: &str = "continue";
pub const DEBUGGER: &str = "debugger";
pub const DEFAULT: &str = "default";
pub const DELETE: &str = "delete";
pub const DO: &str = "do";
pub const ELSE: &str = "else";
pub const ENUM: &str = "enum";
pub const EXPORT: &str = "export";
pub const EXTENDS: &str = "extends";
pub const FALSE: &str = "false";
pub const FINALLY: &str = "finally";
pub const FOR: &str = "for";
pub const FUNCTION: &str = "function";
pub const IF: &str = "if";
pub const IMPORT: &str = "import";
pub const IN: &str = "in";
pub const INSTANCEOF: &str = "instanceof";
pub const LET: &str = "let";
pub const NEW: &str = "new";
pub const NULL: &str = "null";
pub const RETURN: &str = "return";
pub const SUPER: &str = "super";
pub const SWITCH: &str = "switch";
pub const THIS: &str = "this";
pub const THROW: &str = "throw";
pub const TRUE: &str = "true";
pub const TRY: &str = "try";
pub const TYPEOF: &str = "typeof";
pub const VAR: &str = "var";
pub const VOID: &str = "void";
pub const WHILE: &str = "while";
pub const WITH: &str = "with";
pub const YIELD: &str = "yield";

pub trait KeywordExt {
    fn is_keyword(&self) -> bool;
}

impl KeywordExt for str {
    fn is_keyword(&self) -> bool {
        // SAFETY: first byte of the &str
        match unsafe { *self.as_ptr() } {
            b'a' => matches!(self, AWAIT),
            b'b' => matches!(self, BREAK),
            b'c' => {
                matches! {
                    self,
                    CASE     |
                    CATCH    |
                    CLASS    |
                    CONST    |
                    CONTINUE
                }
            },
            b'd' => {
                matches! {
                    self,
                    DEBUGGER |
                    DEFAULT  |
                    DELETE   |
                    DO
                }
            },
            b'e' => {
                matches! {
                    self,
                    ELSE    |
                    ENUM    |
                    EXPORT  |
                    EXTENDS
                }
            },
            b'f' => {
                matches! {
                    self,
                    FALSE    |
                    FINALLY  |
                    FOR      |
                    FUNCTION
                }
            },
            b'i' => {
                matches! {
                    self,
                    IF         |
                    IMPORT     |
                    IN         |
                    INSTANCEOF
                }
            },
            b'l' => matches!(self, LET),
            b'n' => matches!(self, NEW | NULL),
            b'r' => matches!(self, RETURN),
            b's' => matches!(self, SUPER | SWITCH),
            b't' => {
                matches! {
                    self,
                    THIS   |
                    THROW  |
                    TRUE   |
                    TRY    |
                    TYPEOF
                }
            },
            b'v' => matches!(self, VAR | VOID),
            b'w' => matches!(self, WHILE | WITH),
            b'y' => matches!(self, YIELD),
            _ => false,
        }
    }
}
