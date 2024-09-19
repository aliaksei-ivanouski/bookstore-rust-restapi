use std::process::id;
use std::time::SystemTime;
use rocket::http::Status;
use rocket::serde::{Serialize, json::Json, Deserialize};
use rocket::State;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryOrder};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTimeUtc;
use crate::auth::AuthenticatedUser;
use crate::controllers::{ErrorResponse, Response, SuccessResponse};
use crate::controllers::books::{ResBook, ResBookList};
use crate::entities::{author, book, prelude::Author};
use crate::entities::author::Model;
use crate::entities::prelude::Book;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResAuthor {
    id: i32,
    firstname: String,
    lastname: String,
    bio: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResAuthorList {
    total: usize,
    authors: Vec<ResAuthor>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqAuthor {
    firstname: String,
    lastname: String,
    bio: String,
}

impl From<&author::Model> for ResAuthor {
    fn from(a: &author::Model) -> Self {
        Self {
            id: a.id,
            firstname: a.firstname.to_owned(),
            lastname: a.lastname.to_owned(),
            bio: a.bio.to_owned(),
        }
    }
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Response<Json<ResAuthorList>> {
    let db = db as &DatabaseConnection;

    let authors = Author::find()
        .order_by_desc(author::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(ResAuthor::from)
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResAuthorList { total: authors.len(), authors })
    )))
}

#[post("/", data = "<req_author>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_author: Json<ReqAuthor>,
) -> Response<Json<ResAuthor>> {
    let db = db as &DatabaseConnection;

    let author = author::ActiveModel {
        user_id: Set(user.id),
        firstname: Set(req_author.firstname.to_owned()),
        lastname: Set(req_author.lastname.to_owned()),
        bio: Set(req_author.bio.to_owned()),
        ..Default::default()
    };

    let author = author.insert(db).await?;
    Ok(SuccessResponse((
        Status::Created,
        Json(ResAuthor::from(&author)),
    )))
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<Json<ResAuthor>> {
    let db = db as &DatabaseConnection;

    let author = Author::find_by_id(id).one(db).await?;

    let author = match author {
        Some(author) => author,
        None => {
            return Err(super::ErrorResponse((
                Status::NotFound,
                "Author not found.".to_string()
            )));
        }
    };

    Ok(SuccessResponse((
        Status::Created,
        Json(ResAuthor::from(&author)),
    )))
}

#[put("/<id>", data = "<req_author>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
    req_author: Json<ReqAuthor>,
) -> Response<Json<ResAuthor>> {
    let db = db as &DatabaseConnection;

    let mut author: author::ActiveModel = match Author::find_by_id(id).one(db).await? {
        Some(a) => a.into(),
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "Author not found.".to_string()
            )));
        }
    };

    author.firstname = Set(req_author.firstname.to_owned());
    author.lastname = Set(req_author.lastname.to_owned());
    author.bio = Set(req_author.bio.to_owned());
    author.updated_at = Set(DateTimeUtc::from(SystemTime::now()));

    let author = author.update(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(ResAuthor::from(&author)),
    )))
}

#[delete("/<id>")]
pub async fn delete(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<String> {
    let db = db as &DatabaseConnection;

    let author = match Author::find_by_id(id).one(db).await? {
        Some(a) => a,
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "Author not found.".to_string()
            )));
        }
    };

    author.delete(db).await?;

    Ok(SuccessResponse((Status::Ok, "Author deleted.".to_string())))
}

#[get("/<id>/books")]
pub async fn get_books(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<Json<ResBookList>> {
    let db = db as &DatabaseConnection;

    let author = match Author::find_by_id(id).one(db).await? {
        Some(a) => a,
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "Author not found.".to_string()
            )));
        }
    };

    let books: Vec<book::Model> = author.find_related(Book).all(db).await?;

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBookList {
            total: books.len(),
            books: books.iter().map(|b| ResBook {
                id: b.id,
                author_id: b.author_id,
                title: b.title.to_owned(),
                year: b.year.to_owned(),
                cover: b.cover.to_owned(),
            })
                .collect::<Vec<_>>(),
        })
    )))
}
