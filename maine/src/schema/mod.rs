use crate::pb;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::{collections::HashMap, convert::From, convert::Into};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Maine {
    pub id: i64,
    pub uid: i64,
    pub content: String,
    pub comments: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Maine {
    pub fn new(id: i64, uid: i64, content: String) -> Self {
        Maine {
            id,
            uid,
            content,
            comments: 0,
            created_at: chrono::Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }
}

// impl From<&Maine> for pb:: {
//     fn from(feed: &Maine) -> Self {
//         pb::Maine {
//             id: feed.id,
//             uid: feed.uid,
//             content: feed.content.clone(),
//             likes: feed.comments,
//             comments: feed.comments,
//             emojis: vec![],
//             publish_time: None,
//         }
//     }
// }

// impl From<Maine> for pb::Maine {
//     fn from(feed: Maine) -> Self {
//         (&feed).into()
//     }
// }

// impl Maine {
//     pub fn to_pb(&self) -> pb::Maine {
//         pb::Maine::default()
//     }
// }
