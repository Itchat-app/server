use super::*;
use crate::utils::snowflake;
use nanoid::nanoid;
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, Default, OpgModel)]
#[ormlite(table = "sessions")]
pub struct Session {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    pub token: String,
    #[opg(string)]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub user_id: i64,
}

impl Session {
    pub fn new(user_id: i64) -> Self {
        Self {
            id: snowflake::generate(),
            token: nanoid!(64),
            user_id,
        }
    }

    #[cfg(test)]
    pub async fn faker() -> Self {
        let user = User::faker();
        let session = Self::new(user.id);

        user.insert(pool()).await.unwrap();

        session
    }

    #[cfg(test)]
    pub async fn cleanup(self) {
        use crate::utils::Ref;
        self.user_id
            .user()
            .await
            .unwrap()
            .delete(pool())
            .await
            .unwrap();
    }
}

impl Base for Session {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn create() {
        run(async {
            let session = Session::faker().await;
            let session = session.insert(pool()).await.unwrap();
            let session = Session::get_one(session.id, pool()).await.unwrap();

            session.cleanup().await;
        });
    }
}
