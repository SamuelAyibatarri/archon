use uuid::Uuid;

#[derive(Debug)]
pub struct ToolCall {
    pub name: String,
    pub succeeded: bool,
}

pub struct Session {
    pub id: Uuid,
    pub prompt: String,
    pub tools: Vec<ToolCall>,
}

impl Session {
    pub fn new(prompt: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            prompt,
            tools: Vec::new(),
        }
    }

    pub fn branch_name(&self) -> String {
        let session_id = self.id;
        let branch_name = format!("archon/session-{session_id}");

        branch_name
    }

    pub fn record_tool(&mut self, name: String, succeeded: bool) {
        let tool_call = ToolCall { name, succeeded };

        self.tools.push(tool_call);
    }

    pub fn failures_for(&self, name: &str) -> usize {
        let counter: usize = self
            .tools
            .iter()
            .filter(|t| t.name == name && !t.succeeded)
            .count();
        counter
    }

    pub fn needs_escalation(&self, name: &str) -> bool {
        self.failures_for(name) >= 3
    }
}
