pub trait UserService {
    fn add_user(username: String, user_id: i32);
}

pub struct UserServiceImpl;

// impl UserService for UserServiceImpl {
//     fn add_user(username: String, user_id: i32) {
        
//     }
// }

impl UserServiceImpl {
    fn test3() -> String {
        "zbf".to_string()
    }
}