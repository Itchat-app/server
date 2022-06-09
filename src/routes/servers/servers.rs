use crate::database::DB as db;
use crate::guards::r#ref::Ref;
use crate::structures::*;
use crate::utils::error::*;
use rbatis::crud::CRUDMut;
use rocket::serde::{json::Json, Deserialize};
use validator::Validate;

#[openapi]
#[get("/")]
async fn fetch_many(user: User) -> Json<Vec<Server>> {
    Json(user.fetch_servers().await)
}

#[openapi]
#[get("/<server_id>")]
async fn fetch_one(server_id: Ref) -> Result<Json<Server>> {
    Ok(Json(server_id.server().await?))
}

#[derive(Deserialize, Validate, JsonSchema)]
struct CreateServerSchema<'a> {
    #[validate(length(min = 1, max = 50))]
    name: &'a str,
}

#[openapi]
#[post("/", data = "<data>")]
async fn create(user: User, data: Json<CreateServerSchema<'_>>) -> Result<Json<Server>> {
    let data = data.into_inner();

    data.validate()
        .map_err(|error| Error::InvalidBody { error })?;

    let server = Server::new(data.name.into(), user.id);
    let member = Member::new(user.id, server.id);
    let category = Channel::new_category("General".into(), server.id);
    let mut chat = Channel::new_text("general".into(), server.id);

    chat.parent_id = Some(category.id);

    let mut tx = db.acquire_begin().await.unwrap();

    tx.save(&server, &[]).await.unwrap();
    tx.save(&category, &[]).await.unwrap();
    tx.save(&chat, &[]).await.unwrap();
    tx.save(&member, &[]).await.unwrap();

    tx.commit().await.unwrap();

    Ok(Json(server))
}

#[openapi]
#[delete("/<server_id>")]
async fn delete(user: User, server_id: Ref) -> Result<()> {
    let server = server_id.server().await?;

    if server.owner_id == user.id {
        server.delete().await;
    } else {
        if let Some(member) = server.fetch_member(user.id).await {
            member.delete().await;
        } else {
            return Err(Error::UnknownMember);
        }
    }

    Ok(())
}

pub fn routes() -> (Vec<rocket::Route>, rocket_okapi::okapi::openapi3::OpenApi) {
    openapi_get_routes_spec![fetch_many, fetch_one, create, delete]
}
