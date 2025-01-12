use chrono::NaiveDateTime;
use diesel::{dsl::count_star, prelude::*, result::Error, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    config::db::Connection,
    models::game::Game,
    schema::user::{self, dsl::*},
};

#[derive(Serialize, Associations, Identifiable, Queryable, Selectable, ToSchema)]
#[diesel(table_name = user)]
#[diesel(belongs_to(Game))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub game_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = user)]
pub struct UserForm {
    pub name: String,
    pub game_id: Uuid,
}

impl User {
    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<User>> {
        user.load::<User>(conn)
    }

    pub fn find_by_id(user_id: Uuid, conn: &mut Connection) -> QueryResult<User> {
        user.find(user_id).get_result::<User>(conn)
    }

    pub fn find_by_game(game: &Game, conn: &mut Connection) -> QueryResult<Vec<User>> {
        User::belonging_to(game)
            .select(User::as_select())
            .load(conn)
    }

    pub fn insert(data: UserForm, conn: &mut Connection) -> QueryResult<User> {
        diesel::insert_into(user)
            .values(&data)
            .get_result::<User>(conn)
    }

    pub fn update(user_id: Uuid, data: UserForm, conn: &mut Connection) -> QueryResult<User> {
        diesel::update(user)
            .filter(id.eq(user_id))
            .set(data)
            .get_result::<User>(conn)
    }

    pub fn delete(user_id: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(user).filter(id.eq(user_id)).execute(conn)
    }

    pub fn count(game: &Option<Game>, conn: &mut Connection) -> Result<i64, Error> {
        let count = if let Some(value) = game {
            User::belonging_to(value)
                .select(count_star())
                .first(conn)?
        } else {
            user.select(count_star())
                .first(conn)?
        };

        Ok(count)
    }
}
