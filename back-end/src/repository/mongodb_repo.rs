use std::env;
extern crate dotenv;
use dotenv::dotenv;
use bcrypt::{hash, verify};

use mongodb::{
    bson::{extjson::de::Error,
        // oid::ObjectId,
        doc},
    results::{InsertOneResult,
        // UpdateResult,
        // DeleteResult
    },
    Client, Collection
};

// use futures::stream::TryStreamExt;

use crate::models::user_model::User;

pub struct MongoRepo{
    col: Collection<User>,
}

impl MongoRepo{
    pub async fn init()->Self{
        dotenv().ok();
        let uri=match env::var("MONGO_URI"){
            Ok(v)=>v.to_string(),
            Err(_)=>format!("Error on loading env variable"),
        };
        let client=Client::with_uri_str(uri).await.unwrap();
        let db=client.database("rustWithUser");
        let col: Collection<User>=db.collection("User");

        MongoRepo {col}
    }
    
    pub async fn create_user(&self, new_user:User)->Result<InsertOneResult, Error>{

        let email=new_user.email.clone();
        let value=doc! {"email":email};
        let user_exist=self.col.find_one(value,None).await.ok().expect("Error on finding user!!!");

        match user_exist{
            Some(_)=>{return Err(Error::DeserializationError{ message:"Email already exist!!!".to_string()})}
            None=>{
            let new_doc=User{
               id:None,
               email:new_user.email,
               password:hash(new_user.password,10).unwrap(),
            };
   
            let user=self.col.insert_one(new_doc, None)
               .await.ok().expect("Error on creating the User!!!");
               Ok(user)
            }
        }
    }

    pub async fn confirm_user(&self, login_user:User)->Result<&str,Error>{

        let email=login_user.email.clone();
        let value=doc! {"email":email};
        let user_exist=self.col.find_one(value,None).await.ok().expect("Error on finding user!!!");

        match user_exist{
            Some(user_doc)=>{
                let hashed_password = user_doc.password;
                let is_valid=verify(login_user.password, &hashed_password)
                    .map_err(|e| Error::DeserializationError {
                    message: format!("Error verifying password: {}", e),
                })?;
                if is_valid{
                    Ok("Successfully Logined!!!")
                }else{
                    Err(Error::DeserializationError{message: "Invalid Password!!!".to_string()})
                }
            }
            None=>{return Err(Error::DeserializationError{message: "User not found!!!".to_string()})}
        }
    }
}