use mongodb::bson::doc;

use super::DataBase;
use crate::structure::user::User;

impl DataBase {
    pub async fn insert_user(&self, item: &User) -> Result<(), ()> {
        let coll = self.db.collection::<User>("user");

        if coll
            .find_one(doc! {"name": &item.name}, None)
            .await
            .unwrap()
            .is_some()
        {
            return Err(());
        }

        coll.insert_one(item, None)
            .await
            .map(|_| ())
            .map_err(|_| ())
    }

    pub async fn find_user(&self, item: &User) -> Result<(), ()> {
        let coll = self.db.collection::<User>("user");

        match coll
            .find_one(doc! {"name": &item.name}, None)
            .await
            .unwrap()
        {
            Some(user) if user.password == item.password => Ok(()),
            _ => Err(()),
        }
    }
}
