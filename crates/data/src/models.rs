
// This is a macro for building the model itself-
// it's mainly in a different macro to allow the
// creation of the 'belongs_to' line.
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

// This is the main model-building macro. It defines
// two structs- the 'object' struct used throughout the
// project, and a 'model' struct that is used only as 
// a query result.
// Also defined:
//      * From<Model>/From<Object> trait implementations
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

    // The main body of the macro
    (   table:  $tn:ident,
        owner:  ( $( $bt:ident ),* ),
        traits: [ $( $oi:ident ),* ], 
        $n:ident {
            $( $c:ident: $t:ty ),*
    }) => {
        paste::item! {

            // Define the object struct that is
            // used for insertion.
            make_object!(
                table:  stringify!($tn),
                owner:  ($( $bt ),*),
                traits: [$( $oi, )* Insertable,Debug,Serialize],
                [<$n Model>] {
                    id: Option<i32>,
                    $( $c: $t ),*
            });

            // Define the model struct that is
            // used for query results.
            #[derive(Serialize, Queryable, Debug)]
            pub struct $n {
                pub id: i32,
                $( pub $c: $t ),*
            }

            // Implement From<Model> for Object.
            impl From<[<$n Model>]> for $n {
                fn from(model: [<$n Model>]) -> Self {
                    $n {
                        id: model.id.unwrap_or(-1),
                        $( $c: model.$c ),*
                    }
                }
            }
            
            // implement From<Object> for Model.
            impl From<$n> for [<$n Model>] {
                fn from(object: $n) -> Self {
                    [<$n Model>] {
                        id: Some(object.id),
                        $( $c: object.$c ),*
                    }
                }
            }
        }

    }
}
