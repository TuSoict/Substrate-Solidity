use chrono::{NaiveDate};
use uuid::Uuid;

use crate::db_connection::establish_connection;

use tonic::{Request, Response, Status};

use crate::user::{
    user_service_server::UserService,
    CreateUserReply, CreateUserRequest,
    DeleteUserReply,
    EmptyBodyRequest, UpdateUserReply,
    UpdateUserRequest, UserReply, UserRequest, Users,
    LoginResponse, LoginRequest,
};

#[derive(Default)]
pub struct User {}

#[tonic::async_trait]
impl UserService for User {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        println!("Got a request: {:#?}", &request);

        let LoginRequest {  phone_number , password} = &request.into_inner();

        let mut conn = establish_connection();

        let rows = &conn
            .query("SELECT password FROM users WHERE phone_number = $1 limit 1", &[&phone_number])
            .unwrap();
        
        let get_password_from_phone : &str = rows.get(0).unwrap().get(0) ;
        let reply = if get_password_from_phone != password {
            LoginResponse {
                ok: false,
            }
        }else {
            LoginResponse {
                ok: true,
            }
        } ;


        Ok(Response::new(reply))
    }

    async fn get_user(&self, request: Request<UserRequest>) -> Result<Response<UserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UserRequest {id,  phone_number } = &request.into_inner();

        let mut conn = establish_connection();

        let rows = &conn
            .query("SELECT * FROM users WHERE phone_number = $1 OR id= $2 limit 1", &[&phone_number, &id])
            .unwrap();

        let date_of_birth: NaiveDate = rows.get(0).unwrap().get(4);
        let reply = UserReply {
            id: rows.get(0).unwrap().get(0),
	        phone_number : rows.get(0).unwrap().get(1),
            first_name: rows.get(0).unwrap().get(2),
            last_name: rows.get(0).unwrap().get(3),
            date_of_birth: date_of_birth.to_string(),
            password : "".to_string()
        };
        Ok(Response::new(reply))
    }

    async fn list_users(&self, request: Request<EmptyBodyRequest>) -> Result<Response<Users>, Status> {
        println!("Got a request: {:#?}", &request);
        let mut conn = establish_connection();

        let mut v: Vec<UserReply> = Vec::new();
        for row in &conn.query("SELECT * FROM users order by date_of_birth asc", &[]).unwrap() {
            let date_of_birth: NaiveDate = row.get(4);
            let user = UserReply {
                id: row.get(0),
		        phone_number : row.get(1),
                first_name: row.get(2),
                last_name: row.get(3),
                date_of_birth: date_of_birth.to_string(),
                password : "".to_string(),
            };
            v.push(user);
        }

        let reply = Users { users: v };

        Ok(Response::new(reply))
    }

    async fn create_user( &self, request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserReply>, Status> {
        println!("Got a request: {:#?}", &request);

        let CreateUserRequest {
	        phone_number,
            first_name,
            last_name,
            date_of_birth,
            password,
        } = &request.into_inner();

        let mut conn = establish_connection();

        let user_id = Uuid::new_v4().to_hyphenated().to_string();
        let serialize_date_of_birth = NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").unwrap(); // String to Date

        let number_of_rows_affected = &conn.execute(
            "INSERT INTO users (id, phone_number, first_name, last_name, date_of_birth, password) VALUES ($1, $2, $3, $4, $5, $6)",
            &[
                &user_id,
		        &phone_number,
                &first_name,
                &last_name,
                &serialize_date_of_birth,
                &password
            ],
        )
            .unwrap();

        let reply = if number_of_rows_affected == &(0 as u64) {
            CreateUserReply {
                message: format!(
                    "Fail to create user with id {} and phone_number {} .",
                    &user_id, &phone_number
                ),
            }
        } else {
            CreateUserReply {
                message: format!(
                    "Success create {} user with id {} and phone_number {}.",
                    &number_of_rows_affected, &user_id, &phone_number, 
                ),
            }
        };

        Ok(Response::new(reply))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UpdateUserRequest {
            id,
	    phone_number,
            first_name,
            last_name,
            date_of_birth,
        } = &request.into_inner();

        let serialize_date_of_birth = NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").unwrap(); // String to Date

        let mut conn = establish_connection();

        let number_of_rows_affected = &conn
            .execute(
                "UPDATE users SET phone_number =$2 ,  first_name = $3, last_name = $4, date_of_birth = $5  WHERE id = $1 OR phone_number = $2",
                &[
                    &id,
	  	    &phone_number,
                    &first_name,
                    &last_name,
                    &serialize_date_of_birth,
                ],
            )
            .unwrap();

        let reply = if number_of_rows_affected == &(0 as u64) {
            UpdateUserReply {
                message: format!("Fail to update the user with id {}.", id),
            }
        } else {
            UpdateUserReply {
                message: format!("Update {} user with phone {}.", &number_of_rows_affected, &phone_number),
            }
        };

        Ok(Response::new(reply))
    }

    async fn delete_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<DeleteUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UserRequest { id, phone_number } = &request.into_inner();
        let mut conn = establish_connection();

        let number_of_rows_affected = &conn
            .execute("DELETE FROM users WHERE id = $1 OR phone_number = $2", &[&id, &phone_number ])
            .unwrap();

        let reply = if number_of_rows_affected == &(0 as u64) {
            DeleteUserReply {
                message: format!("Fail to delete the user with id {}.", id),
            }
        } else {
            DeleteUserReply {
                message: format!("Remove the user with phone_number {}.", phone_number),
            }
        };

        Ok(Response::new(reply))
    }

    async fn delete_users(
        &self,
        request: Request<EmptyBodyRequest>,
    ) -> Result<Response<DeleteUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let mut conn = establish_connection();

        let rows = &conn.query("DELETE FROM users", &[]).unwrap();

        let reply = DeleteUserReply {
            message: format!("Remove {} user data from the database.", rows.len()),
        };

        Ok(Response::new(reply))
    }
}

