use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Chat
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[skip_serializing_none]
pub struct Chatlog {
    #[serde(rename = "logId")]
    pub log_id: i64,

    #[serde(rename = "prevId")]
    pub prev_log_id: Option<i64>,

    #[serde(rename = "chatId")]
    pub channel_id: i64,

    #[serde(rename = "authorId")]
    pub author_id: i64,

    #[serde(rename = "sendAt")]
    pub send_at: i64,

    #[serde(flatten)]
    pub chat: Chat,

    pub referer: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Chat {
    #[serde(rename = "type")]
    pub chat_type: ChatType,

    #[serde(flatten)]
    pub content: ChatContent,

    #[serde(rename = "msgId")]
    pub message_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct ChatContent {
    pub message: Option<String>,
    pub attachment: Option<String>,
    pub supplement: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ChatType(pub i32);

macro_rules! define_chat_type {
    ($name: ident, $num: literal) => {
        pub const $name: ChatType = ChatType($num);
    };
}

impl ChatType {
    define_chat_type!(FEED, 0);
    define_chat_type!(TEXT, 1);
    define_chat_type!(PHOTO, 2);
    define_chat_type!(VIDEO, 3);
    define_chat_type!(CONTACT, 4);
    define_chat_type!(AUDIO, 5);
    define_chat_type!(DITEMEMOTICON, 6);
    define_chat_type!(DITEMGIFT, 7);
    define_chat_type!(DITEMIMG, 8);
    define_chat_type!(KAKAOLINKV1, 9);
    define_chat_type!(AVATAR, 11);
    define_chat_type!(STICKER, 12);
    define_chat_type!(SCHEDULE, 13);
    define_chat_type!(VOTE, 14);
    define_chat_type!(LOTTERY, 15);
    define_chat_type!(MAP, 16);
    define_chat_type!(PROFILE, 17);
    define_chat_type!(FILE, 18);
    define_chat_type!(STICKERANI, 20);
    define_chat_type!(NUDGE, 21);
    define_chat_type!(ACTIONCON, 22);
    define_chat_type!(SEARCH, 23);
    define_chat_type!(POST, 24);
    define_chat_type!(STICKERGIF, 25);
    define_chat_type!(REPLY, 26);
    define_chat_type!(MULTIPHOTO, 27);
    define_chat_type!(VOIP, 51);
    define_chat_type!(LIVETALK, 52);
    define_chat_type!(CUSTOM, 71);
    define_chat_type!(ALIM, 72);
    define_chat_type!(PLUSFRIEND, 81);
    define_chat_type!(PLUSEVENT, 82);
    define_chat_type!(PLUSFRIENDVIRAL, 83);
    define_chat_type!(OPEN_SCHEDULE, 96);
    define_chat_type!(OPEN_VOTE, 97);
    define_chat_type!(OPEN_POST, 98);

    pub const DELETED_BIT: i32 = 14;
    pub const DELETED_MASK: i32 = 1 << Self::DELETED_BIT;

    pub const fn into_original(self) -> Self {
        Self(self.0 & !Self::DELETED_MASK)
    }

    pub const fn into_deleted(self) -> Self {
        Self(self.0 | Self::DELETED_MASK)
    }

    pub const fn deleted(self) -> bool {
        (self.0 & Self::DELETED_MASK) != 0
    }
}
