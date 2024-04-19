pub mod bet;
pub mod r#match;
pub mod message;
pub mod user;

fn escape_arg(arg: &str) -> String {
    // space が入っている場合はダブルクォーテーションで囲んで、ダブルクォーテーションの前にバックスラッシュを挿入する
    if arg.contains(' ') {
        arg.replace("\\", "\\\\").replace("\"", "\\\"")
    } else {
        arg.to_string()
    }
}
