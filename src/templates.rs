use std::path::Path;

use crate::config::Alias;

pub const CONFIG_TEMPLATE: &str = r#"{
    "aliases": [
        ["alias1", "long long command 1"],
        ["alias2", "long long command 2"]
    ]
}
"#;

pub fn render_sh_file(alias: Alias, shebang_path: Option<&Path>) -> String {
    format!(
        "#!{}\nexec {} \"$@\"\n",
        shebang_path.unwrap_or(Path::new("/bin/sh")).display(),
        alias.command
    )
}

pub fn render_bat_file(alias: Alias) -> String {
    format!("@echo off\n{} %*\n", alias.command)
}

pub fn render_ps1_file(alias: Alias) -> String {
    format!("& {} @args\n", alias.command)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET_SH_FILE: &str = "#!/usr/bin/env bash\nexec long long command \"$@\"\n";
    const TARGET_BAT_FILE: &str = "@echo off\nlong long command %*\n";
    const TARGET_PS1_FILE: &str = "& long long command @args\n";

    fn test_alias() -> Alias {
        Alias {
            name: "alias".into(),
            command: "long long command".into(),
        }
    }

    #[test]
    fn sh_is_valid() {
        assert_eq!(
            render_sh_file(test_alias(), Some(Path::new("/usr/bin/env bash")),),
            TARGET_SH_FILE
        );
    }

    #[test]
    fn bat_is_valid() {
        assert_eq!(render_bat_file(test_alias()), TARGET_BAT_FILE);
    }

    #[test]
    fn ps1_is_valid() {
        assert_eq!(render_ps1_file(test_alias()), TARGET_PS1_FILE);
    }
}
