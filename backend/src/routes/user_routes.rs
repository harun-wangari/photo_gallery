

struct User{
    email: String,
    password: String
}

pub async fn user_login() -> String {
    "user login successful".to_string()
}

// pub async fn user_login(body:Json<User>) -> Result<Json<User>, StatusCode> {

//     let result = Result::ok();
//     match result {
//         Ok(user) => Ok(Json({
//             email: "user@example.com".to_string(),
//             password: "123456".to_string(),
//         })),
//         Err(err) => Err(StatusCode::INTERNAL_SERVER_ERROR),
//     }
// }