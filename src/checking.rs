use json::JsonValue;

pub fn check_undone(list: (&str, &JsonValue), content: String) -> String {
    let mut intern: String = content.to_owned();
    let mut has_tasks = false;
    intern.push_str(list.0);
    intern.push_str("\n------------------------------\n");
    for item in list.1.entries() {
        let mut tdo_content = item.1.members();
        let text = tdo_content.next().unwrap().as_str().unwrap();
        let done = tdo_content.next().unwrap().as_bool().unwrap();
        if !done {
            has_tasks = true;
            intern.push_str(text);
            intern.push_str("\n");
        }
    }
    if !has_tasks {
        intern.push_str("This Category has no (undone) Tasks\n");
    }
    intern
}
