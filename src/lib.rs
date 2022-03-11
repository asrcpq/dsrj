pub fn compile(input: &str) -> String {
	let mut result: Vec<String> = Vec::new();
	for line in input.split('\n') {
		let line = line.trim().to_string();
		match line.chars().next() {
			Some('#') | None => continue,
			Some('}') | Some(']') => {
				let prev_line = result.last_mut().unwrap();
				if let Some(',') = prev_line.chars().last() {
					prev_line.pop();
				}
			}
			_ => {}
		}
		result.push(line);
	}
	result.join("\n")
}
