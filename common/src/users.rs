use crate::{
    data::{Data, DataFinder},
    error::Error,
    Result,
};
use serde::{Deserialize, Serialize};
use std::{collections::hash_map::HashMap, path::Path};

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub enum AccessControlKind {
    Allow,
    Deny,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub salt: String,
    pub password_hash: String,
    pub pubkey_fpr: String,
    pub superuser: bool,
    pub acl: Vec<AccessControl>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct AccessControl {
    pub control: AccessControlKind,
    pub service: String,
}

impl User {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        match Data::load(&path)? {
            Data::User(user) => Ok(user),
            _ => Err(Error::new(format!("{:?} is not a user.", path.as_ref()))),
        }
    }
}

pub fn load_users<P>(users_dir: P) -> Result<HashMap<String, User>>
where
    P: AsRef<Path>,
{
    let users_dir = users_dir.as_ref();
    let mut users: HashMap<String, User> = HashMap::new();
    for data in DataFinder::new(&users_dir)? {
        let data = data?;
        let user = match data {
            Data::User(item) => item,
            _ => continue,
        };
        users.insert(user.username.clone(), user);
    }
    Ok(users)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_user() {
        let user = User::load("test/users/foo1.json").unwrap();
        assert_eq!(user.id, "Foo1".to_string());
        assert_eq!(user.username, "Foo1 Foo1".to_string());
        assert_eq!(user.email, "foo1@example.com".to_string());
        assert_eq!(user.salt, "foo1 salt".to_string());
        assert_eq!(user.password_hash, "foo1 hash".to_string());
        assert_eq!(user.pubkey_fpr, "foo1 fpr".to_string());
        assert_eq!(user.superuser, true);
        assert_eq!(user.acl.len(), 2);
        assert_eq!(user.acl[0].control, AccessControlKind::Allow);
        assert_eq!(user.acl[0].service, "service 1".to_string());
        assert_eq!(user.acl[1].control, AccessControlKind::Deny);
        assert_eq!(user.acl[1].service, "*".to_string());
    }

    #[test]
    fn test_load_users() {
        let users = load_users("test/users").unwrap();
        assert_eq!(users.len(), 4);

        let user = users.get("Foo1 Foo1").unwrap();
        assert_eq!(user.id, "Foo1".to_string());
        assert_eq!(user.username, "Foo1 Foo1".to_string());
        assert_eq!(user.email, "foo1@example.com".to_string());
        assert_eq!(user.salt, "foo1 salt".to_string());
        assert_eq!(user.password_hash, "foo1 hash".to_string());
        assert_eq!(user.pubkey_fpr, "foo1 fpr".to_string());
        assert_eq!(user.superuser, true);
        assert_eq!(user.acl.len(), 2);
        assert_eq!(user.acl[0].control, AccessControlKind::Allow);
        assert_eq!(user.acl[0].service, "service 1".to_string());
        assert_eq!(user.acl[1].control, AccessControlKind::Deny);
        assert_eq!(user.acl[1].service, "*".to_string());

        let user = users.get("Foo2 Foo2").unwrap();
        assert_eq!(user.id, "Foo2".to_string());
        assert_eq!(user.username, "Foo2 Foo2".to_string());
        assert_eq!(user.email, "foo2@example.com".to_string());
        assert_eq!(user.salt, "foo2 salt".to_string());
        assert_eq!(user.password_hash, "foo2 hash".to_string());
        assert_eq!(user.pubkey_fpr, "foo2 fpr".to_string());
        assert_eq!(user.superuser, false);
        assert_eq!(user.acl.len(), 3);
        assert_eq!(user.acl[0].control, AccessControlKind::Allow);
        assert_eq!(user.acl[0].service, "service 1".to_string());
        assert_eq!(user.acl[1].control, AccessControlKind::Allow);
        assert_eq!(user.acl[1].service, "service 2".to_string());
        assert_eq!(user.acl[2].control, AccessControlKind::Deny);
        assert_eq!(user.acl[2].service, "*".to_string());

        let user = users.get("Bar Bar").unwrap();
        assert_eq!(user.id, "Bar".to_string());
        assert_eq!(user.username, "Bar Bar".to_string());
        assert_eq!(user.email, "bar@example.com".to_string());
        assert_eq!(user.salt, "bar salt".to_string());
        assert_eq!(user.password_hash, "bar hash".to_string());
        assert_eq!(user.pubkey_fpr, "bar fpr".to_string());
        assert_eq!(user.superuser, false);
        assert_eq!(user.acl.len(), 1);
        assert_eq!(user.acl[0].control, AccessControlKind::Allow);
        assert_eq!(user.acl[0].service, "*".to_string());

        let user = users.get("Baz Baz").unwrap();
        assert_eq!(user.id, "Baz".to_string());
        assert_eq!(user.username, "Baz Baz".to_string());
        assert_eq!(user.email, "baz@example.com".to_string());
        assert_eq!(user.salt, "baz salt".to_string());
        assert_eq!(user.password_hash, "baz hash".to_string());
        assert_eq!(user.pubkey_fpr, "baz fpr".to_string());
        assert_eq!(user.superuser, false);
        assert_eq!(user.acl.len(), 1);
        assert_eq!(user.acl[0].control, AccessControlKind::Deny);
        assert_eq!(user.acl[0].service, "*".to_string());
    }
}
