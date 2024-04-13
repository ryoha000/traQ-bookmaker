use derive_new::new;
use std::sync::Arc;

use crate::{
    model::message::message_created::MessageCreatedEvent,
    module::{Modules, ModulesExt},
};

mod help;
mod reg;

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
        .ok_or(anyhow::anyhow!("No command specified"))?;
    match command_name.as_str() {
        "help" | "--help" | "-h" => help::handle(modules, event.message.channel_id).await?,
        "reg" => {
            reg::handle(
                modules,
                reg::RegArg::new(
                    event.message.user.id,
                    event.message.user.name,
                    event.message.channel_id,
                ),
            )
            .await?
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown command: {}", command_name));
        }
    }

    Ok(())
}
