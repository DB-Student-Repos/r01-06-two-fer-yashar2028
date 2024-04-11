pub fn twofer(name: &str) -> String {
        if name == "" {
             "One for you, one for me.".to_string()
        } else {
              format!("One for {name}, one for me.").to_string()
        }
}
