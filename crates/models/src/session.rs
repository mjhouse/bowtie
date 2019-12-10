pub use bowtie_data::{schema::*,traits::*};
use crate::user::User;

use serde::{Serialize, Deserialize};

// model!(
//     table:  sessions,
//     owner:  (User),
//     traits: [Identifiable,Associations],
//     Session {
//         user_key: String,
//         user_id:  i32
// });

// access!( Session,
//     id:i32          => sessions::id,
//     user_key:String => sessions::user_key,
//     user_id:i32     => sessions::user_id
// );