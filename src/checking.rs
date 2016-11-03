use json::JsonValue;

pub fn check_lists(tdos: JsonValue) -> String {
    let mut content = String::new();
    for list in tdos.entries() {
        let return_result = check_undone(list, &content);
        match return_result {
            Some(ref c) => content = c.to_owned(),
            None => {}
        }

    }

    content
}

pub fn check_undone(list: (&str, &JsonValue), content: &String) -> Option<String> {
    let mut intern: String = content.to_owned();
    let mut has_tasks = false;
    intern.push_str("\n------------------------------------------------------------\n\t");
    intern.push_str(list.0);
    intern.push_str("\n------------------------------------------------------------\n");
    for item in list.1.entries() {
        let mut tdo_content = item.1.members();
        let text = "- ".to_owned() + tdo_content.next().unwrap().as_str().unwrap();
        let done = tdo_content.next().unwrap().as_bool().unwrap();
        if !done {
            has_tasks = true;
            intern.push_str(&text);
            intern.push_str("\n\n");
        }
    }
    if !has_tasks {
        None
    } else {
        Some(intern)
    }
}
