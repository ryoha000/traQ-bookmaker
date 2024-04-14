use derive_new::new;
use kernel::model::{message::NewMessage, Id};

#[derive(new)]
pub struct SendSummaryHelpMessage {
    pub channel_id: String,
    pub commands: Vec<CommandSummary>,
}

#[derive(new)]
pub struct CommandSummary {
    pub name: String,
    pub description: String,
}

fn format_summary(command: &CommandSummary) -> String {
    format!("- {} (`{}``)", command.name, command.description)
}

impl From<SendSummaryHelpMessage> for NewMessage {
    fn from(c: SendSummaryHelpMessage) -> Self {
        let commands_content = c
            .commands
            .iter()
            .map(|command| format_summary(command))
            .collect::<Vec<String>>()
            .join("\n");

        let content = format!("### コマンド\n{}", commands_content);
        NewMessage::new(Id::new(c.channel_id), content, false)
    }
}

#[derive(new)]
pub struct SendHelpMessage {
    pub channel_id: String,
    pub command: Command,
}

#[derive(new)]
pub struct Command {
    pub name: String,
    pub title: String,
    pub description: String,
    pub example: String,
}

fn format_command(command: &Command) -> String {
    format!(
        "### {}(`{}`)\n{}\n```\n{}\n```",
        command.title, command.name, command.description, command.example
    )
}

impl From<SendHelpMessage> for NewMessage {
    fn from(c: SendHelpMessage) -> Self {
        NewMessage::new(Id::new(c.channel_id), format_command(&c.command), true)
    }
}
