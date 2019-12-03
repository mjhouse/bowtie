pub use bowtie_data::{schema::*,traits::*};
use crate::user::User;

use diesel::prelude::*;
use serde::{Serialize};
use chrono::prelude::*;

use diesel::result::Error as DieselError;

model!(
    table:  views,
    owner:  (User),
    traits: [Identifiable,Associations],
    View {
        user_id: i32
});