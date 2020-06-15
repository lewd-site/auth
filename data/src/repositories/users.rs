use crate::models::users::{NewUser, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct UserRepository();

impl UserRepository {
    pub fn get_by_uuid(conn: &PgConnection, user_uuid: &str) -> Option<User> {
        use crate::schema::users::dsl::*;

        let items: Vec<User> = users
            .filter(uuid.eq(user_uuid))
            .limit(1)
            .load(conn)
            .unwrap();

        items.into_iter().next()
    }

    pub fn get_by_name(conn: &PgConnection, user_name: &str) -> Option<User> {
        use crate::schema::users::dsl::*;

        let items: Vec<User> = users
            .filter(name.eq(user_name))
            .limit(1)
            .load(conn)
            .unwrap();

        items.into_iter().next()
    }

    pub fn get_by_email(conn: &PgConnection, user_email: &str) -> Option<User> {
        use crate::schema::users::dsl::*;

        let items: Vec<User> = users
            .filter(email.eq(user_email))
            .limit(1)
            .load(conn)
            .unwrap();

        items.into_iter().next()
    }

    pub fn create(conn: &PgConnection, user: &NewUser) -> User {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(user)
            .get_result(conn)
            .unwrap()
    }
}
