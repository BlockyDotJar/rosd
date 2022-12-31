use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Schlechd" => "Err",
        "Guad" => "Ok",
        "Zeichnkedn" => "String",
        "Biachl" => "HashMap",
        "Standard" => "Default",
        "Fehla" => "Error",
        "Möglichkeit" => "Option",
        "Ebsend" => "Some",
        "Nixend" => "None",
        "Eagebnis" => "Result",
        "Seibsd" => "Self",
        "grusch" => "collections",
        "gsoßrad" => "println",
        "afhean" => "break",
        "asynchron" => "async",
        "obwoadn" => "await",
        "schleifn" => "loop",
        "schoib" => "move",
        "kisdn" => "crate",
        "Schachdl" => "Box",
        "unerreichbara_code" => "unreachable_code",
        "ois" => "as",
        "konstand" => "const",
        "eignschafd" => "trait",
        "typ" => "type",
        "gfealeh" => "unsafe",
        "in" => "in",
        "fo" => "from",
        "dynamisch" => "dyn",
        "afmocha" => "unwrap",
        "standard" => "default",
        "ois_ref" => "as_ref",
        "ea" => "io",
        "ausahoib" => "extern",
        "foisch" => "false",
        "funktion" => "fn",
        "übagoadned" => "super",
        "eifügn" => "insert",

        // Iterator Funktionen
        "wieda" => "iter",
        "zu_wieda" => "into_iter",
        "zuaoadnen" => "map",
        "ausbreidn" => "flat_map",
        "foid" => "fold",
        "lan" => "drain",
        "samen" => "collect",
        "find" => "find",
        "nimm" => "take",
        "produkd" => "product",

        // Ordering
        "vgl" => "cmp",
        "Oadnung" => "Ordering",
        "Meah" => "Greater",
        "Wenga" => "Less",
        "Gleich" => "Equal",
        "hoi" => "get",
        "ealaub" => "allow",
        "mei" | "drecks_scheiß" | "so_ein_schmarn" | "hagod" => "panic",
        "modui" => "mod",
        "ändabar" => "mut",
        "nei" => "new",
        "wo" => "where",
        "fia" => "for",
        "hoi_oda_füg_ei_mid" => "get_or_insert_with",
        "eistieg" => "main",
        "öffndle" => "pub",
        "nix" => None?,
        "zruckgem" => "return",
        "umstz" => "impl",
        "ref" => "ref",
        "entsprich" => "match",
        "wenn" => "if",
        "sunst" => "else",
        "seibsd" => "self",
        "loss" => "let",
        "stadisch" => "static",
        "struktur" => "struct",
        "eawoad" => "expect",
        "während" => "while",
        "nutz" => "use",
        "eina" => "into",
        "woah" => "true",
        "aufzeiung" => "enum",
        "Gruppn" => "Group",
        "Identifikador" => "Ident",
        "TokenStrom" => "TokenStream",
        "TokenBam" => "TokenTree",
        "zua_zeichnkedn" => "to_string",
        "ois_zeichnkedn" => "as_str",
        "reichweidn" => "span",
        "Tabein" => "Vec",
        "tabein" => "vec",
        "strom" => "stream",
        "schoim" => "push",
        "eaweidan" => "extend",
        "begrenza" => "delimiter",
        "zeichnsetzung" => "Punct",
        "literaln" => "Literal",
        "prozedurals_makro" => "proc_macro",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rosd(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
