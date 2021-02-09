pub fn sqlstrip(tostrip: String) -> String {
    tostrip
        .trim()
        .replace("\\", "")
        .replace("\"", "")
        .replace("'", "")
}
