
#[macro_export]
macro_rules! make_object {
    (   table:  $tn:expr,
        owner:  ( $( $bt:ident ),* ),
        traits: [ $( $oi:ident ),* ], 
        $n:ident {
            $( $c:ident: $t:ty ),*
    }) => {
        #[derive($( $oi, )*)]
        $( #[belongs_to($bt)] )*
        #[table_name=$tn]
        pub struct $n {
            $( pub $c: $t ),*
        } 
    }
}

#[macro_export]
macro_rules! model {

    // An alias matcher that leaves out the "owner"
    // argument
    (   table:  $tn:ident,
        traits: [ $( $oi:ident ),* ], 
        $n:ident {
            $( $c:ident: $t:ty ),*
    }) => {
        model!(
            table:  $tn,
            owner:  (),
            traits: [ $( $oi ),* ], 
            $n {
                $( $c: $t ),*
            }
        );
    };

    // This is the main model-building macro. It defines
    // two structs- the 'object' struct used throughout the
    // project, and a 'model' struct that is used only as 
    // a query result.
    // Also defined:
    //      * From<Model>/From<Object> trait implementations
    //      * Helper macros for queries
    //      *  
    (   table:  $tn:ident,
        owner:  ( $( $bt:ident ),* ),
        traits: [ $( $oi:ident ),* ], 
        $n:ident {
            $( $c:ident: $t:ty ),*
    }) => {
        paste::item! {

            macro_rules! query {

                // A query macro that returns an Option<Object>
                ( one: $d:expr, $q:expr ) => {
                    match $tn::table
                    .filter($q)
                    .first::<[<$n Model>]>($d)
                    {
                        Ok(m) => {
                            Some(m.into())
                        }
                        Err(e) => {
                            warn!("Error during query: {}",e);
                            None
                        }
                    }
                };

                // A query macro that returns a Vec<Object> result
                ( many: $d:expr, $q:expr ) => {
                    query!(many: $d, $q, $tn::id.desc())
                };

                // A query macro that returns an ordered Vec<Object>
                ( many: $d:expr, $q:expr, $o:expr ) => {
                    match $tn::table
                        .filter($q)
                        .order($o)
                        .load::<[<$n Model>]>($d) {
                            Ok(p) => {
                                p.into_iter()
                                    .map(|m| m.into())
                                    .collect()
                            },
                            Err(e) => {
                                warn!("Error during query: {}",e);
                                vec![]
                            }
                        }
                }
                
            }

            // Define the object struct that is
            // used for insertion.
            make_object!(
                table:  stringify!($tn),
                owner:  ($( $bt ),*),
                traits: [$( $oi, )* Insertable,Debug,Serialize],
                $n {
                    id: Option<i32>,
                    $( $c: $t ),*
            });

            // Define the model struct that is
            // used for query results.
            #[derive(Queryable, Debug)]
            pub struct [<$n Model>] {
                pub id: i32,
                $( pub $c: $t ),*
            }

            // Implement From<Model> for Object.
            impl From<[<$n Model>]> for $n {
                fn from(model: [<$n Model>]) -> Self {
                    $n {
                        id: Some(model.id),
                        $( $c: model.$c ),*
                    }
                }
            }
            
            // implement From<Object> for Model.
            impl From<$n> for [<$n Model>] {
                fn from(object: $n) -> Self {
                    [<$n Model>] {
                        id: object.id.unwrap_or(-1),
                        $( $c: object.$c ),*
                    }
                }
            }
        }

    }
}

#[macro_export]
macro_rules! access {
    ( $s:ty,
      $( $n:ident:$t:ty => $p:path ),*
    ) => {
        impl $s {
            paste::item! {
                $(
                    pub fn [<for_ $n>](t_value: $t) -> Option<Self> {
                        let conn = db!(None);
                        query!(one: &conn,$p.eq(t_value))
                    } 
                )*
            }
        } 
    }
}