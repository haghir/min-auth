use crate::{data::Data, users::AccessControl};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub enum RequestContent {
    CreateUser(CreateUserRequest),
    UpdateUser(UpdateUserRequest),
    DeleteUser(DeleteUserRequest),
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Request {
    pub id: String,
    pub issuer: String,
    pub timestamp: String,
    pub content: RequestContent,
    pub rand: u64,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub superuser: bool,
    pub acl: Vec<AccessControl>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct UpdateUserRequest {
    pub user_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub superuser: Option<bool>,
    pub acl: Option<Vec<AccessControl>>,
    pub renew_password: bool,
    pub renew_pubkey: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct DeleteUserRequest {
    pub user_id: String,
}

impl Request {
    pub fn load<P>(path: P) -> Result<Request, Box<dyn std::error::Error>>
    where
        P: AsRef<Path>,
    {
        match Data::load(&path)? {
            Data::Request(request) => Ok(request),
            _ => Err(format!("{:?} is not a request.", path.as_ref()).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::users::AccessControlKind;

    #[test]
    fn test_load_request() {
        let req = Request::load("test/requests/create-user-1.json").unwrap();
        assert_eq!(req.id, "create-user-1-request");
        assert_eq!(req.issuer, "create-user-1-issuer");
        assert_eq!(req.timestamp, "2024-01-01 12:34:56.789");
        if let RequestContent::CreateUser(content) = req.content {
            assert_eq!(content.username, "new-user-1".to_string());
            assert_eq!(content.email, "new-user-1@example.com".to_string());
            assert_eq!(content.superuser, true);
            assert_eq!(content.acl.len(), 2);
            assert_eq!(content.acl[0].control, AccessControlKind::Allow);
            assert_eq!(content.acl[0].service, "service 1".to_string());
            assert_eq!(content.acl[1].control, AccessControlKind::Deny);
            assert_eq!(content.acl[1].service, "*".to_string());
        } else {
            panic!("Failed to parse a CreateUser content.");
        }
        assert_eq!(req.rand, 123456789);

        let req = Request::load("test/requests/update-user-1.json").unwrap();
        assert_eq!(req.id, "update-user-1-request");
        assert_eq!(req.issuer, "update-user-1-issuer");
        assert_eq!(req.timestamp, "2024-01-01 12:34:56.789");
        if let RequestContent::UpdateUser(content) = req.content {
            assert_eq!(content.user_id, "user-1-id".to_string());
            assert_eq!(content.username, Some("user-1".to_string()));
            assert_eq!(content.email, Some("user-1@example.com".to_string()));
            assert_eq!(content.superuser, Some(true));
            if let Some(acl) = content.acl {
                assert_eq!(acl.len(), 2);
                assert_eq!(acl[0].control, AccessControlKind::Allow);
                assert_eq!(acl[0].service, "service 1".to_string());
                assert_eq!(acl[1].control, AccessControlKind::Deny);
                assert_eq!(acl[1].service, "*".to_string());
            } else {
                panic!("Failed to parse an ACL.");
            }
        } else {
            panic!("Failed to parse a UpdateUser content.");
        }
        assert_eq!(req.rand, 123456789);

        let req = Request::load("test/requests/update-user-2.json").unwrap();
        assert_eq!(req.id, "update-user-2-request");
        assert_eq!(req.issuer, "update-user-2-issuer");
        assert_eq!(req.timestamp, "2024-01-01 12:34:56.789");
        if let RequestContent::UpdateUser(content) = req.content {
            assert_eq!(content.user_id, "user-2-id".to_string());
            assert_eq!(content.username, None);
            assert_eq!(content.email, None);
            assert_eq!(content.superuser, None);
            assert_eq!(content.acl, None);
        } else {
            panic!("Failed to parse a UpdateUser content.");
        }
        assert_eq!(req.rand, 987654321);

        let req = Request::load("test/requests/delete-user-1.json").unwrap();
        assert_eq!(req.id, "delete-user-1-request");
        assert_eq!(req.issuer, "delete-user-1-issuer");
        assert_eq!(req.timestamp, "2024-01-01 12:34:56.789");
        if let RequestContent::DeleteUser(content) = req.content {
            assert_eq!(content.user_id, "user-1-id".to_string());
        } else {
            panic!("Failed to parse a DeleteUser content.");
        }
        assert_eq!(req.rand, 123456789);
    }
}
