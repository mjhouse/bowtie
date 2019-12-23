
// Builds 'for_<field>' functions for already-existing models
// so that single results can be queried by value. Should be
// expanded to include 'Vec<Model>'-returning query functions
// as well.
#[macro_export]
macro_rules! queries {
    (   table: $tn: ident,
        model: $mn: ident,
        one: {
            $( $fn:ident: $ft:ty => $fp:path ),*
        }
    ) => {
        impl $mn {
            paste::item! {
                $(
                    pub fn [<for_ $fn>](t_conn: &PgConnection, t_value: $ft) -> Option<Self> {
                        let conn = db!(None);
                        match $tn::table
                        .filter($fp.eq(t_value))
                        .first::<[<$mn Model>]>(t_conn) {
                            Ok(m) => Some(m.into()),
                            Err(e) => {
                                warn!("Error during query: {}",e);
                                None
                            }
                        }
                    } 
                )*
            }
        } 
    };
}