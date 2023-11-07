use grpc_api::api::*;
use crate::errors::CustomError;
use db::queries;
use deadpool_postgres::Pool;
use tonic::{Request, Response, Status};

pub struct UsersService {
    pub pool: Pool,
}

#[tonic::async_trait]
impl grpc_api::users_server::Fortunes for UsersService {
    async fn get_users(
        &self,
        _request: Request<GetUsersRequest>,
    ) -> Result<Response<GetUsersResponse>, Status> {
        // Get a client from our database pool
        let client = self
            .pool
            .get()
            .await
            .map_err(|e| CustomError::Database(e.to_string()))?;

        // Get the fortunes from the database
        let fortunes = queries::users::users(&client)
            .await
            .map_err(|e| CustomError::Database(e.to_string()))?;

        // Map the structs we get from cornucopia to the structs
        // we need for our gRPC reply.
        let users = users
            .into_iter()
            .map(|user| User {
                id: user.id as u32,
                email: user.email,
            })
            .collect();

        let users = GetUsersResponse {
            users,
        };

        return Ok(Response::new(response));
    }
}