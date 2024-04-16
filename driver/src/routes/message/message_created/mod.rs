use app::model::message::help::{Command, SendHelpMessage};
use derive_new::new;
use std::sync::Arc;

use crate::{
    model::message::message_created::MessageCreatedEvent,
    module::{Modules, ModulesExt},
};

use self::close::CloseArg;

mod close;
mod help;
mod reg;
mod start;

#[derive(new)]
pub struct ParseState {
    pub duble_quote_enabled: bool,
    pub next_char_escape_enabled: bool,
    pub next_char_is_new_arg: bool,
    pub args: Vec<Vec<char>>,
}

fn parse_args(text: &str) -> Vec<String> {
    text.chars()
        .fold(
            ParseState::new(false, false, false, vec![]),
            |mut state, c| {
                let insert_char_to_last_arg = |state: &mut ParseState, c: char| {
                    if state.args.is_empty() {
                        state.args.push(vec![c]);
                    } else {
                        state.args.last_mut().unwrap().push(c);
                    }
                };

                if state.next_char_escape_enabled {
                    insert_char_to_last_arg(&mut state, c);
                    state.next_char_escape_enabled = false;
                } else if c == '"' {
                    state.duble_quote_enabled = !state.duble_quote_enabled;
                } else if c.is_whitespace() {
                    if state.duble_quote_enabled {
                        insert_char_to_last_arg(&mut state, c);
                    } else {
                        state.next_char_is_new_arg = true;
                    }
                } else if c == '\\' {
                    state.next_char_escape_enabled = true;
                } else {
                    if state.next_char_is_new_arg {
                        state.args.push(vec![c]);
                        state.next_char_is_new_arg = false;
                    } else {
                        insert_char_to_last_arg(&mut state, c);
                    }
                }
                state
            },
        )
        .args
        .iter()
        .map(|arg| arg.iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_args() {
        assert_eq!(parse_args("a b c"), vec!["a", "b", "c"]);
        assert_eq!(parse_args("a \"b c\""), vec!["a", "b c"]);

        assert_eq!(parse_args("\"a\" \"b\""), vec!["a", "b"]);
        assert_eq!(parse_args("\"a b\" \"c d\""), vec!["a b", "c d"]);

        assert_eq!(parse_args("a\\ b c"), vec!["a b", "c"]);
        assert_eq!(parse_args("\"a\\\"b\" c"), vec!["a\"b", "c"]);

        assert_eq!(parse_args("a#b c&d e^f"), vec!["a#b", "c&d", "e^f"]);
        assert_eq!(parse_args("\"a b c\" d e"), vec!["a b c", "d", "e"]);

        assert_eq!(parse_args("  a b c  "), vec!["a", "b", "c"]);
        assert_eq!(parse_args("  \"a b\" c  "), vec!["a b", "c"]);

        assert_eq!(parse_args(""), Vec::<String>::new());
        assert_eq!(parse_args("     "), Vec::<String>::new());
        assert_eq!(parse_args(" \"  \" "), vec!["  "]);
    }
}

fn is_help_command(args: &[String]) -> bool {
    args.first().map(|s| s.as_str()) == Some("help")
        || args.first().map(|s| s.as_str()) == Some("--help")
        || args.first().map(|s| s.as_str()) == Some("-h")
}

pub async fn handle(modules: Arc<Modules>, event: MessageCreatedEvent) -> anyhow::Result<()> {
    let is_mentioned = event
        .message
        .embedded
        .iter()
        .any(|e| e.r#type == "user" && e.id == modules.bot_user_id());
    if !is_mentioned {
        return Ok(());
    }

    let args = parse_args(&event.message.text);

    let command_name = args
        .first()
        .and_then(|s| Some(s.as_str()))
        .unwrap_or_default();
    let args = args.iter().skip(1).cloned().collect::<Vec<_>>();
    let channel_id = event.message.channel_id;
    match command_name {
        "help" | "--help" | "-h" => help::handle(modules, channel_id).await?,
        "reg" => {
            reg::handle(
                modules,
                reg::RegArg::new(event.message.user.id, event.message.user.name, channel_id),
            )
            .await?
        }
        "start" => {
            if is_help_command(&args) {
                modules
                    .message_use_case()
                    .send_help_message(SendHelpMessage::new(
                        channel_id,
                        Command::new(
                            "start".to_string(),
                            "賭けの開始".to_string(),
                            "賭けを開始します\n進行中の賭けはチャンネルごとに1つのみです"
                                .to_string(),
                            "@BOT_bookmaker start \"VCT PACIFIC\" Gen.G PRX".to_string(),
                        ),
                    ))
                    .await?;
                return Ok(());
            }
            start::handle(
                modules,
                start::StartArg::new(
                    channel_id,
                    args.first()
                        .and_then(|s| Some(s.to_string()))
                        .unwrap_or_default(),
                    args.iter().skip(1).cloned().collect(),
                ),
            )
            .await?
        }
        "close" => {
            if is_help_command(&args) {
                modules
                    .message_use_case()
                    .send_help_message(SendHelpMessage::new(
                        channel_id,
                        Command::new(
                            "close".to_string(),
                            "bet の締め切り".to_string(),
                            "bet を締め切ります。\nこの時点でレートは確定し、bet は受け付けられなくなります"
                                .to_string(),
                            "@BOT_bookmaker close".to_string(),
                        ),
                    ))
                    .await?;
                return Ok(());
            }
            close::handle(modules, CloseArg::new(channel_id)).await?
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown command: {}", command_name));
        }
    }

    Ok(())
}
