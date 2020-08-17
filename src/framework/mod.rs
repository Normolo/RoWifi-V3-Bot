pub mod context;
mod configuration;
mod map;
mod parser;
pub mod prelude;
mod structures;
pub mod utils;

use twilight::{
    gateway::Event,
    model::{
        channel::Message,
        guild::Permissions
    },
    command_parser::Arguments
};
use uwl::Stream;

use context::Context;
use configuration::Configuration;
use map::CommandMap;
use structures::*;
use parser::{ParseError, Invoke};

#[derive(Default)]
pub struct Framework {
    commands: Vec<(&'static Command, CommandMap)>,
    help: Option<&'static HelpCommand>,
    config: Configuration
}

impl Framework {
    pub fn configure<F>(mut self, f: F) -> Self where F: FnOnce(&mut Configuration) -> &mut Configuration {
        f(&mut self.config);
        self
    }

    pub fn command(mut self, command: &'static Command) -> Self {
        let map = CommandMap::new(&[command]);
        self.commands.push((command, map));
        self
    }

    async fn dispatch(&self, msg: Message, mut context: Context) {
        if msg.author.bot || msg.webhook_id.is_some() || msg.guild_id.is_none() || msg.content.is_empty() {
            return;
        }

        let mut stream = Stream::new(&msg.content);
        stream.take_while_char(|c| c.is_whitespace());

        let prefix = parser::find_prefix(&mut stream, &msg, &self.config).await;
        if prefix.is_some() && stream.rest().is_empty() {
            //print just the prefix
            return;
        }

        if prefix.is_none() {
            return;
        }

        let invocation = parser::command(&mut stream, &self.commands, &self.help.as_ref().map(|h| h.options.name)).await;
        let invoke = match invocation {
            Ok(i) => i,
            Err(ParseError::UnrecognisedCommand(_)) => {
                return;
            }
        };

        match invoke {
            Invoke::Help => {

            },
            Invoke::Command{command} => {
                if !self.run_checks(&context, &msg, command).await {
                    return;
                }
                let args = Arguments::new(stream.rest());

                let res = (command.fun)(&mut context, &msg, args).await;

                match res {
                    Ok(()) => {},
                    Err(why) => println!("Command {:?} errored: {:?}", command, why)
                }
            }
        }
    }

    pub async fn handle_event(&self, event: Event, context: Context) {
        match event {
            Event::MessageCreate(msg) => {
                self.dispatch(msg.0, context).await;
            },
            _ => {}
        }
    }

    async fn run_checks(&self, context: &Context, msg: &Message, command: &Command) -> bool {
        if self.config.blocked_users.contains(&msg.author.id) {
            return false;
        }

        if self.config.owners.contains(&msg.author.id) {
            return true;
        }

        //Check for disabled channels (Implement after database cache)
        //Bucketing mechanism

        if let Some(guild) = context.cache.guild(msg.guild_id.unwrap()).await {
            if self.config.blocked_users.contains(&guild.owner_id) {
                return false;
            }

            if msg.author.id.0 == guild.owner_id.0 {
                return true;
            }

            if let Some(member) = context.cache.member(guild.id, msg.author.id).await {
                for role in member.roles.iter() {
                    if let Some(role) = context.cache.role(*role).await {
                        if role.permissions.contains(Permissions::ADMINISTRATOR) {
                            return true;
                        }
                        if role.permissions.contains(command.options.required_permissions) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}