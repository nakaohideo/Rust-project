use serde::Serialize;
use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{post,
    // get,put,delete,
    web::{Data,Json,
        // Path
    }, HttpResponse,};
// use mongodb::{bson::{oid::ObjectId},};


#[derive(Serialize)]
pub struct Text{
    msg:String,
    color:String
}

#[post["/sign-up"]]
pub async fn sign_up(db: Data<MongoRepo>, new_user: Json<User>)->HttpResponse{
    let data=User{
        id:None,
        email: new_user.email.to_owned(),
        password:new_user.password.to_owned(),
    };

    let user_detail= db.create_user(data).await;
    match user_detail{
        Ok(_)=> HttpResponse::Ok().json(Text{msg:"Successfully Registered".to_string(),color:"success".to_string()}),
        Err(err)=> HttpResponse::Ok().json(Text{msg:err.to_string(),color:"error".to_string()}),
    }
}

#[post["/sign-in"]]
pub async fn sign_in(db: Data<MongoRepo>, login_user: Json<User>)->HttpResponse{
    let data=User{
        id:None,
        email: login_user.email.to_owned(),
        password: login_user.password.to_owned()
    };

    let user_login= db.confirm_user(data).await;
    match user_login{
        Ok(ok)=>HttpResponse::Ok().json(Text{msg:ok.to_string(),color:"success".to_string()}),
        Err(err)=>HttpResponse::Ok().json(Text{msg:err.to_string(),color:"error".to_string()})
    }
}