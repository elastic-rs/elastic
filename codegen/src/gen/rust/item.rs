use syntax::ast::*;
use syntax::parse::token;
use syntax::codemap::{ Spanned, DUMMY_SP };
use syntax::ptr::P;
use super::parse_path;

/// Get a `use` statement for a path
pub fn use_ident<I>(path: &str, idents: I) -> Stmt
    where I: IntoIterator<Item=Ident> {
        let parts = parse_path(path);
        Stmt {
            span: DUMMY_SP,
            node: StmtKind::Decl(
                P(Spanned {
                    span: DUMMY_SP,
                    node: DeclKind::Item(
                        P(Item {
                            ident: token::str_to_ident(path),
                            attrs: Vec::new(),
                            id: DUMMY_NODE_ID,
                            node: ItemKind::Use(
                                P(Spanned {
                                    span: DUMMY_SP,
                                    node: ViewPath_::ViewPathList(
                                        Path {
                                            span: DUMMY_SP,
                                            global: false,
                                            segments: parts.iter().map(|part| PathSegment {
                                                identifier: token::str_to_ident(part),
                                                parameters: PathParameters::none()
                                            }).collect()
                                        },
                                        idents.into_iter().map(|ident| Spanned {
                                            span: DUMMY_SP,
                                            node: PathListItemKind::Ident {
                                                name: ident,
                                                rename: None,
                                                id: DUMMY_NODE_ID
                                            }
                                        }).collect()
                                    )
                                })
                            ),
                            vis: Visibility::Inherited,
                            span: DUMMY_SP
                        })
                    )
                }),
                DUMMY_NODE_ID
            )
        }
}