use crate::{schema, service::db};
use anyhow::{Context, Result};
use sqlx::FromRow;
use sqlx::*;

pub struct Maine {
    pub db: db::DB,
}

impl Maine {
    pub fn builder(db: db::DB) -> Self {
        Maine { db }
    }

    pub async fn insert_feed(&self, feed: &schema::Maine) -> Result<()> {
        // sqlx::query("INSERT INTO feeds (id, uid, content, comments) VALUES (?, ?, ?, ?)")
        //     .bind(feed.id)
        //     .bind(feed.uid)
        //     .bind(&feed.content)
        //     .bind(feed.comments)
        //     .execute(&self.db.pool)
        //     .await?;
        Ok(())
    }
}
