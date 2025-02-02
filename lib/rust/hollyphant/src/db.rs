use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub mas_client_id: Option<String>,
    pub mas_client_secret: Option<String>,
    pub mas_access_token: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::accounts)]
pub struct NewMasAccount<'a> {
    name: &'a str,
    mas_instance: &'a str,
    mas_client_id: &'a str,
    mas_client_secret: &'a str,
    mas_access_token: &'a str,
}

impl<'a> NewMasAccount<'a> {
    pub fn new(
        name: &'a str,
        mas_instance: &'a str,
        mas_client_id: &'a str,
        mas_client_secret: &'a str,
        mas_access_token: &'a str,
    ) -> Self {
        Self {
            name,
            mas_instance,
            mas_client_id,
            mas_client_secret,
            mas_access_token,
        }
    }
}
