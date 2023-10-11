macro_rules! meta_match_visitor {
    ( $ident: ident ($typ: ty) {

        $( $pat: pat => $target: expr ),*

    } ) => {

        struct $ident <'a> ( &'a mut $typ );

        impl<'a> MetaVisitor for _Visitor0<'a> {
            type Output = ();

            fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
                fmt.write_str("a meta identifier")
            }

            fn visit_path<S>(self, source: S, path: Option<&str>, _span: Option<Span>) -> Result<Self::Output, S::Error> where S: MetaSource {
                match path {
                    $( Some($pat) => $target ),*
                    None => source.visit(self),
                    _ => Ok(())
                }
            }

            fn visit_list<A>(self, mut access: A, _span: Option<Span>) -> Result<Self::Output, A::Error> where A: MetaAccess {
                while access.remaining() {
                    access.visit(Self(self.0))?;
                }
                Ok(())
            }
        }

    };
}


pub(crate) use meta_match_visitor;