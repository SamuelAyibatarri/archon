use archon_core::Session;

fn main() {
    let mut new_session: Session = Session::new(String::from("A random prompt"));
    new_session.record_tool(String::from("Sharp new tool"), true);
    // Record some failures
    new_session.record_tool(String::from("tool_that_failed"), false);
    new_session.record_tool(String::from("tool_that_failed"), false);
    new_session.record_tool(String::from("tool_that_failed"), false);

    let id = new_session.id;
    let prompt = &new_session.prompt;
    let branch_name = new_session.branch_name();
    let tool_failure_count = new_session.failures_for("tool_that_failed");
    let tool_needs_escalation = if new_session.needs_escalation("tool_that_failed") {
        "yes"
    } else {
        "no"
    };

    println!("Hello, world!, I deadass just added this: {id}, with this prompt: {prompt}");
    println!("Branch: {branch_name}");
    println!("Tools history:  {:#?}", new_session.tools);

    // Shout output 3
    println!("Tool: 'tool_that_failed' failed {tool_failure_count} times!");
    // Needs Escalation?
    println!("Tool: 'tool_that_failed' needs escalation?: {tool_needs_escalation}");
}
