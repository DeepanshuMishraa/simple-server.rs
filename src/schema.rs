use diesel::sql_types::{Int4,Text,Int4};


diesel::table!{
    users (id){
        id->Int4,
        name->Text,
        age->Int4
    }
}
