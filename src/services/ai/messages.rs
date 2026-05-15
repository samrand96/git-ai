use super::Message;

pub(super) fn combine_messages(prompt: &str, messages: Option<&[Message]>) -> String {
    if let Some(messages) = messages {
        let combined = messages
            .iter()
            .map(|msg| format!("{}: {}", msg.role.to_uppercase(), msg.content))
            .collect::<Vec<_>>()
            .join("\n");
        if combined.trim().is_empty() {
            prompt.to_string()
        } else {
            combined
        }
    } else {
        prompt.to_string()
    }
}

pub(super) fn split_system_prompt(prompt: &str, messages: Option<&[Message]>) -> (String, String) {
    if let Some(messages) = messages {
        let mut system = String::new();
        let mut user = String::new();
        for msg in messages {
            if msg.role == "system" {
                system.push_str(&msg.content);
                system.push('\n');
            } else if msg.role == "user" {
                user.push_str(&msg.content);
                user.push('\n');
            }
        }
        let system = system.trim().to_string();
        let user = if user.trim().is_empty() {
            prompt.to_string()
        } else {
            user.trim().to_string()
        };
        (system, user)
    } else {
        (String::new(), prompt.to_string())
    }
}
