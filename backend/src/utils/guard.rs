use axum::{extract::State, http::{Request, StatusCode}, middleware::Next, response::Response};
use headers::{authorization::Bearer, Authorization, HeaderMapExt};
use sqlx::{query,  MySqlPool};

use crate::handlers::user_handler::User;

use super::jwt::verify_jwt;

pub async fn guard<B>(State(db):State<MySqlPool>,mut req: Request<B>, next: Next) -> Result<Response,StatusCode>
where
    B: Send + 'static,
    axum::body::Body: From<B>,
{

    let token = req.headers().typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::BAD_REQUEST)?
        .token()
        .to_owned();

    let claim = verify_jwt(&token)
    .map_err(|_| StatusCode::UNAUTHORIZED);
    
    match claim {
        Ok(c) => {
            let user_id = c.claims.user_id as u64;
            let query = sqlx::query!( "SELECT * FROM tb_users WHERE id = ?", user_id)
            .fetch_one(&db)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
            let user =  User{
                id:Some(query.id.clone() as u64),
                surname: query.surname.clone(),
                lastname: query.lastname.clone(),
                email: query.email.clone(),
                password: "".to_string(),
                photo: query.photo.clone(),
                token:Some(token.clone()),
            };
            
            req.extensions_mut().insert(user);
        },
        Err(_) => return Err(StatusCode::UNAUTHORIZED)
    }

    let req = req.map(axum::body::Body::from);
    Ok(next.run(req).await)
} 