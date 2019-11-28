
#[macro_export]
macro_rules! model {
    (   table:  $tn:expr,
        traits: [ $( $oi:ident ),* ],
        owner:  ( $( $bt:ident ),* ), 
        $n:ident {
            $( $c:ident: $t:ty ),*
    }) => {
        #[derive($( $oi, )* Insertable,Debug,Serialize)]
        $( #[belongs_to($bt)] )*
        #[table_name=$tn]
        pub struct $n {
            pub id: Option<i32>,
            $( pub $c: $t ),*
        }

        // Define the model struct that
        // acts as a query result.
        paste::item! {
            #[derive(Queryable, Debug)]
            pub struct [<$n Model>] {
                pub id: i32,
                $( pub $c: $t ),*
            }

            // Implement From<Model> for Object.
            // the model ($mn) is used to query the 
            // database while the object ($n) is 
            // used for insertion and all other
            // operations
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