use chrono::NaiveDateTime;
use diesel::{prelude::*, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    config::db::Connection,
    models::Model,
    schema::{user, user::dsl::*},
};

#[derive(Serialize, Clone, Deserialize, Default, Queryable, Selectable, ToSchema)]
#[diesel(table_name = user)]
#[diesel(belongs_to(Game))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub game_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = user)]
pub struct ScoreDTO {
    pub name: String,
    pub game_id: Uuid,
}

impl Model<User, Uuid, ScoreDTO> for User {
    fn find_all(conn: &mut Connection) -> QueryResult<Vec<User>> {
        user.load::<User>(conn)
    }

    fn find_by_id(user_id: Uuid, conn: &mut Connection) -> QueryResult<User> {
        user.find(user_id).get_result::<User>(conn)
    }

    fn insert(data: ScoreDTO, conn: &mut Connection) -> QueryResult<User> {
        diesel::insert_into(user)
            .values(&data)
            .get_result::<User>(conn)
    }

    fn update(user_id: Uuid, data: ScoreDTO, conn: &mut Connection) -> QueryResult<User> {
        diesel::update(user)
            .filter(id.eq(user_id))
            .set(data)
            .get_result::<User>(conn)
    }

    fn delete(user_id: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(user)
            .filter(id.eq(user_id))
            .execute(conn)
    }
}
