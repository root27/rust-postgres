use actix_web::{web, HttpResponse, Responder};

use crate::models::User;
use crate::AppState;




pub async fn create(user: web::Json<User>, data: web::Data<AppState>) -> impl Responder {
    let user = User::create(user.into_inner(), &data.db).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub async fn get_all(data: web::Data<AppState>) -> impl Responder {
    let users = User::get_all(&data.db).await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub async fn get(id: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let user = User::get(id.into_inner(), &data.db).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub async fn update(id: web::Path<i32>, user: web::Json<User>, data: web::Data<AppState>) -> impl Responder {
    let user = User::update(id.into_inner(), user.into_inner(), &data.db).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


pub async fn delete(id: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let user = User::delete(id.into_inner(), &data.db).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}







