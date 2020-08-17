use std::{collections::HashSet, sync::Arc};
use dashmap::DashMap;
use twilight::model::id::{GuildId, UserId, ChannelId};

#[derive(Default)]
pub struct Configuration {
    pub blocked_guilds: HashSet<GuildId>,
    pub blocked_users: HashSet<UserId>,
    pub disabled_channels: HashSet<ChannelId>,
    pub on_mention: String,
    pub default_prefix: String,
    pub owners: HashSet<UserId>,
    pub prefixes: Arc<DashMap<GuildId, String>>
}

impl Configuration {
    pub fn default_prefix(&mut self, prefix: &str) -> &mut Self {
        self.default_prefix = prefix.to_string();
        self
    }

    pub fn owners(&mut self, user_ids: HashSet<UserId>) -> &mut Self {
        self.owners = user_ids;
        self
    }

    pub fn on_mention(&mut self, id_to_mention: UserId) -> &mut Self {
        self.on_mention = id_to_mention.to_string();
        self
    }

    pub fn prefixes(&mut self, prefixes: Arc<DashMap<GuildId, String>>) -> &mut Self {
        self.prefixes = prefixes;
        self
    }
}