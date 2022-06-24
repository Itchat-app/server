use super::Base;
use crate::utils::snowflake;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:sessions)]
#[derive(Debug, Serialize, Deserialize, Clone, Default, OpgModel)]
pub struct Session {
    pub id: u64,
    pub token: String,
    pub user_id: u64,
}

impl Base for Session {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Session {
    pub fn new(user_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            token: nanoid!(64),
            user_id,
        }
    }

    #[cfg(test)]
    pub async fn faker() -> Self {
        use crate::structures::User;

        let user = User::faker();

        user.save().await;

        Self::new(user.id)
    }

    #[cfg(test)]
    pub async fn cleanup(&self) {
        use crate::utils::Ref;
        self.delete().await;
        self.user_id.user().await.unwrap().delete().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create() {
        crate::tests::setup().await;
        let session = Session::faker().await;
        session.cleanup().await;
    }
}
