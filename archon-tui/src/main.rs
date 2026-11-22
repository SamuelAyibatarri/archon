use archon_core::Session;

fn main() {
    let mut new_session: Session = Session::new(String::from("A random prompt"));
    new_session.record_tool(String::from("Sharp new tool"), true);
    // Record some failures
    new_session.record_tool(String::from("tool_that_failed"), false);
    new_session.record_tool(String::from("tool_that_failed"), false);
    new_session.record_tool(String::from("tool_that_failed"), false);

    let id = new_session.id;
    let prompt = new_session.prompt.clone();
    let branch_name = new_session.branch_name();
    let tool_failure_count = new_session.failures_for("tool_that_failed");

    // Demo the dedup flow: first sighting escalates, second doesn't.
    let first = new_session.should_escalate("tool_that_failed");
    if first {
        new_session.mark_escalated("tool_that_failed");
    }
    let second = new_session.should_escalate("tool_that_failed");

    println!("Hello, world!, I deadass just added this: {id}, with this prompt: {prompt}");
    println!("Branch: {branch_name}");
    println!("Tools history:  {:#?}", new_session.tools);

    // Shout output 3
    println!("Tool: 'tool_that_failed' failed {tool_failure_count} times!");
    println!("First should_escalate: {first}, second (after mark): {second}");
}
