use crate::auth::Auth;
use crate::db;
use crate::schema::users;
use chrono::{Duration, Utc};
use serde::Serialize;

#[derive(Identifiable, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub ugkthid: String,
    pub realname: String,
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
    username: &'a str,
    ugkthid: &'a str,
    realname: &'a str,
    superadmin: bool,
    assistant_in: Vec<String>,
    teacher_in: Vec<String>,
    token: String,
}

#[derive(Serialize)]
pub struct Profile {
    username: String,
    ugkthid: String,
    realname: String,
}

impl User {
    pub fn to_user_auth(&self, conn: &db::DbConn, secret: &[u8]) -> Option<UserAuth> {
        let exp = Utc::now() + Duration::days(60); // TODO: make sure it works as expected when it expires
        let token = Auth {
            id: self.id,
            username: self.username.clone(),
            ugkthid: self.ugkthid.clone(),
            realname: self.realname.clone(),
            exp: exp.timestamp(),
        }
        .token(secret);

        let superadmin = db::super_admins::is_super(conn, self.id).map(|_| true)?;
        let assistant_in = db::admins::assistant_queue_names(conn, self.id);
        let teacher_in = db::admins::teacher_queue_names(conn, self.id);
        Some(UserAuth {
            username: &self.username,
            ugkthid: &self.ugkthid,
            realname: &self.realname,
            superadmin,
            assistant_in,
            teacher_in,
            token,
        })
    }
}
