use std::collections::BTreeSet;
use egui_code_editor::Syntax;

pub trait CherryBlossomSyntax {
    fn cherry_blossom() -> Syntax;
}

impl CherryBlossomSyntax for Syntax {
    fn cherry_blossom() -> Syntax {
        Syntax {
            language: "CherryBlossom",
            case_sensitive: true,
            comment: "//",
            comment_multiline: ["/*", "*/"],
            hyperlinks: BTreeSet::from(["https"]),
            keywords: BTreeSet::from([
                "f", "auto", "var", "return", "bring", "cycle",
                "if", "else", "stem", "un", "begin", "end"
            ]),
            types: BTreeSet::from([
                "int",
                "i8", "i16", "i32", "i64",
                "u8", "u16", "u32", "u64",
                "f32", "f64", "f128",
                "bool",
                "un char",
                "char", "wchar", "char16", "char32",
                "str", "wstring", "u16string", "u32string",
                "intptr", "uintptr", "size", "ptrdiff",
                "void",
            ]),
            special: BTreeSet::from(["true", "false", "none", "@", "ugly", "main"]),
        }
    }
}
