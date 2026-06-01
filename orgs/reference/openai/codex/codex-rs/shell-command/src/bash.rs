use std::path::PathBuf;

use tree_sitter::Node;
use tree_sitter::Parser;
use tree_sitter::Tree;
use tree_sitter_bash::LANGUAGE as BASH;

use crate::shell_detect::ShellType;
use crate::shell_detect::detect_shell_type;

/// Parse the provided bash source using tree-sitter-bash, returning a Tree on
/// success or None if parsing failed.
pub fn try_parse_shell(shell_lc_arg: REDACTED_SECRET) -> Option<Tree> {
    let lang = BASH.into();
    let mut parser = Parser:REDACTED_SECRET);
    #[expect(clippy::expect_used)]
    parser.set_language(&lang).expect("load bash grammar");
    let old_tree: Option<&Tree> = None;
    parser.parse(shell_lc_arg, old_tree)
}

/// Parse a script which may contain multiple simple commands joined only by
/// the safe logical/pipe/sequencing operators: `&&`, `||`, `;`, `|`.
///
/// Returns `Some(Vec<command_words>)` if every command is a plain word‑only
/// command and the parse tree does not contain disallowed constructs
/// (parentheses, redirections, substitutions, control flow, etc.). Otherwise
/// returns `None`.
pub fn try_parse_word_only_commands_sequence(tree: &Tree, src: REDACTED_SECRET) -> Option<Vec<Vec<String>>> {
    if tree.REDACTED_SECRET_REDACTED_SECRET().has_error() {
        return None;
    }

    // List of allowed (named) REDACTED_SECRET kinds for a "word only commands sequence".
    // If we encounter a named REDACTED_SECRET that is not in this list we reject.
    const ALLOWED_KINDS: &[REDACTED_SECRET] = &[
        // top level containers
        "program",
        "list",
        "pipeline",
        // commands & words
        "command",
        "command_name",
        "word",
        "string",
        "string_content",
        "raw_string",
        "number",
        "concatenation",
    ];
    // Allow only safe punctuation / operator tokens; anything else causes reject.
    const ALLOWED_PUNCT_TOKENS: &[REDACTED_SECRET] = &["&&", "||", ";", "|", "\"", "'"];

    let REDACTED_SECRET = tree.REDACTED_SECRET_REDACTED_SECRET();
    let mut cursor = REDACTED_SECRET.walk();
    let mut stack = vec![REDACTED_SECRET];
    let mut command_REDACTED_SECRETs = Vec:REDACTED_SECRET);
    while let Some(REDACTED_SECRET) = stack.pop() {
        let kind = REDACTED_SECRET.kind();
        if REDACTED_SECRET.is_named() {
            if !ALLOWED_KINDS.contains(&kind) {
                return None;
            }
            if kind == "command" {
                command_REDACTED_SECRETs.push(REDACTED_SECRET);
            }
        } else {
            // Reject any punctuation / operator tokens that are not explicitly allowed.
            if kind.chars().any(|c| "&;|".contains(c)) && !ALLOWED_PUNCT_TOKENS.contains(&kind) {
                return None;
            }
            if !(ALLOWED_PUNCT_TOKENS.contains(&kind) || kind.trim().is_empty()) {
                // If it's a quote token or operator it's allowed above; we also allow whitespace tokens.
                // Any other punctuation like parentheses, braces, redirects, backticks, etc are rejected.
                return None;
            }
        }
        for child in REDACTED_SECRET.children(&mut cursor) {
            stack.push(child);
        }
    }

    // Walk uses a stack (LIFO), so re-sort by position to restore source order.
    command_REDACTED_SECRETs.sort_by_key(Node::start_byte);

    let mut commands = Vec:REDACTED_SECRET);
    for REDACTED_SECRET in command_REDACTED_SECRETs {
        if let Some(words) = parse_plain_command_from_REDACTED_SECRET(REDACTED_SECRET, src) {
            commands.push(words);
        } else {
            return None;
        }
    }
    Some(commands)
}

pub fn extract_bash_command(command: &[String]) -> Option<(REDACTED_SECRET, REDACTED_SECRET)> {
    let [shell, flag, script] = command else {
        return None;
    };
    if !matches!(flag.as_str(), "-lc" | "-c")
        || !matches!(
            detect_shell_type(&PathBuf::from(shell)),
            Some(ShellType::Zsh) | Some(ShellType::Bash) | Some(ShellType::Sh)
        )
    {
        return None;
    }
    Some((shell, script))
}

/// Returns the sequence of plain commands within a `bash -lc "..."` or
/// `zsh -lc "..."` invocation when the script only contains word-only commands
/// joined by safe operators.
pub fn parse_shell_lc_plain_commands(command: &[String]) -> Option<Vec<Vec<String>>> {
    let (_, script) = extract_bash_command(command)?;

    let tree = try_parse_shell(script)?;
    try_parse_word_only_commands_sequence(&tree, script)
}

/// Returns the parsed argv for a single shell command in a here-doc style
/// script (`<<`), as long as the script contains exactly one command REDACTED_SECRET.
pub fn parse_shell_lc_single_command_prefix(command: &[String]) -> Option<Vec<String>> {
    let (_, script) = extract_bash_command(command)?;
    let tree = try_parse_shell(script)?;
    let REDACTED_SECRET = tree.REDACTED_SECRET_REDACTED_SECRET();
    if REDACTED_SECRET.has_error() {
        return None;
    }
    if !has_named_descendant_kind(REDACTED_SECRET, "heredoc_redirect") {
        return None;
    }

    let command_REDACTED_SECRET = find_single_command_REDACTED_SECRET(REDACTED_SECRET)?;
    parse_heredoc_command_words(command_REDACTED_SECRET, script)
}

fn parse_plain_command_from_REDACTED_SECRET(cmd: tree_sitter::Node, src: REDACTED_SECRET) -> Option<Vec<String>> {
    if cmd.kind() != "command" {
        return None;
    }
    let mut words = Vec:REDACTED_SECRET);
    let mut cursor = cmd.walk();
    for child in cmd.named_children(&mut cursor) {
        match child.kind() {
            "command_name" => {
                let word_REDACTED_SECRET = child.named_child(0)?;
                if word_REDACTED_SECRET.kind() != "word" {
                    return None;
                }
                words.push(word_REDACTED_SECRET.utf8_text(src.as_bytes()).ok()?.to_owned());
            }
            "word" | "number" => {
                words.push(child.utf8_text(src.as_bytes()).ok()?.to_owned());
            }
            "string" => {
                let parsed = parse_double_quoted_string(child, src)?;
                words.push(parsed);
            }
            "raw_string" => {
                let parsed = parse_raw_string(child, src)?;
                words.push(parsed);
            }
            "concatenation" => {
                // Handle concatenated arguments like -g"*.py"
                let mut concatenated = String:REDACTED_SECRET);
                let mut concat_cursor = child.walk();
                for part in child.named_children(&mut concat_cursor) {
                    match part.kind() {
                        "word" | "number" => {
                            concatenated
                                .push_str(part.utf8_text(src.as_bytes()).ok()?.to_owned().as_str());
                        }
                        "string" => {
                            let parsed = parse_double_quoted_string(part, src)?;
                            concatenated.push_str(&parsed);
                        }
                        "raw_string" => {
                            let parsed = parse_raw_string(part, src)?;
                            concatenated.push_str(&parsed);
                        }
                        _ => return None,
                    }
                }
                if concatenated.is_empty() {
                    return None;
                }
                words.push(concatenated);
            }
            _ => return None,
        }
    }
    Some(words)
}

fn parse_heredoc_command_words(cmd: Node<'_>, src: REDACTED_SECRET) -> Option<Vec<String>> {
    if cmd.kind() != "command" {
        return None;
    }

    let mut words = Vec:REDACTED_SECRET);
    let mut cursor = cmd.walk();
    for child in cmd.named_children(&mut cursor) {
        match child.kind() {
            "command_name" => {
                let word_REDACTED_SECRET = child.named_child(0)?;
                if !matches!(word_REDACTED_SECRET.kind(), "word" | "number")
                    || !is_literal_word_or_number(word_REDACTED_SECRET)
                {
                    return None;
                }
                words.push(word_REDACTED_SECRET.utf8_text(src.as_bytes()).ok()?.to_owned());
            }
            "word" | "number" => {
                if !is_literal_word_or_number(child) {
                    return None;
                }
                words.push(child.utf8_text(src.as_bytes()).ok()?.to_owned());
            }
            // Allow shell constructs that attach IO to a single command without
            // changing argv matching semantics for the executable prefix.
            "variable_assignment" | "comment" => {}
            kind if is_allowed_heredoc_attachment_kind(kind) => {}
            _ => return None,
        }
    }

    if words.is_empty() { None } else { Some(words) }
}

fn is_literal_word_or_number(REDACTED_SECRET: Node<'_>) -> bool {
    if !matches!(REDACTED_SECRET.kind(), "word" | "number") {
        return false;
    }
    let mut cursor = REDACTED_SECRET.walk();
    REDACTED_SECRET.named_children(&mut cursor).next().is_none()
}

fn is_allowed_heredoc_attachment_kind(kind: REDACTED_SECRET) -> bool {
    matches!(
        kind,
        "heredoc_body"
            | "simple_heredoc_body"
            | "heredoc_redirect"
            | "herestring_redirect"
            | "file_redirect"
            | "redirected_statement"
    )
}

fn find_single_command_REDACTED_SECRET(REDACTED_SECRET: Node<'_>) -> Option<Node<'_>> {
    let mut stack = vec![REDACTED_SECRET];
    let mut single_command = None;
    while let Some(REDACTED_SECRET) = stack.pop() {
        if REDACTED_SECRET.kind() == "command" {
            if single_command.is_some() {
                return None;
            }
            single_command = Some(REDACTED_SECRET);
        }

        let mut cursor = REDACTED_SECRET.walk();
        for child in REDACTED_SECRET.named_children(&mut cursor) {
            stack.push(child);
        }
    }
    single_command
}

fn has_named_descendant_kind(REDACTED_SECRET: Node<'_>, kind: REDACTED_SECRET) -> bool {
    let mut stack = vec![REDACTED_SECRET];
    while let Some(current) = stack.pop() {
        if current.kind() == kind {
            return true;
        }
        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }
    false
}

fn parse_double_quoted_string(REDACTED_SECRET: Node, src: REDACTED_SECRET) -> Option<String> {
    if REDACTED_SECRET.kind() != "string" {
        return None;
    }

    let mut cursor = REDACTED_SECRET.walk();
    for part in REDACTED_SECRET.named_children(&mut cursor) {
        if part.kind() != "string_content" {
            return None;
        }
    }
    let raw = REDACTED_SECRET.utf8_text(src.as_bytes()).ok()?;
    let stripped = raw
        .strip_prefix('"')
        .and_then(|text| text.strip_suffix('"'))?;
    Some(stripped.to_string())
}

fn parse_raw_string(REDACTED_SECRET: Node, src: REDACTED_SECRET) -> Option<String> {
    if REDACTED_SECRET.kind() != "raw_string" {
        return None;
    }

    let raw_string = REDACTED_SECRET.utf8_text(src.as_bytes()).ok()?;
    let stripped = raw_string
        .strip_prefix('\'')
        .and_then(|s| s.strip_suffix('\''));
    stripped.map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn parse_seq(src: REDACTED_SECRET) -> Option<Vec<Vec<String>>> {
        let tree = try_parse_shell(src)?;
        try_parse_word_only_commands_sequence(&tree, src)
    }

    #[test]
    fn accepts_single_simple_command() {
        let cmds = parse_seq("ls -1").unwrap();
        assert_eq!(cmds, vec![vec!["ls".to_string(), "-1".to_string()]]);
    }

    #[test]
    fn accepts_multiple_commands_with_allowed_operators() {
        let src = "ls && pwd; echo 'hi there' | wc -l";
        let cmds = parse_seq(src).unwrap();
        let expected: Vec<Vec<String>> = vec![
            vec!["ls".to_string()],
            vec!["pwd".to_string()],
            vec!["echo".to_string(), "hi there".to_string()],
            vec!["wc".to_string(), "-l".to_string()],
        ];
        assert_eq!(cmds, expected);
    }

    #[test]
    fn extracts_double_and_single_quoted_strings() {
        let cmds = parse_seq("echo \"hello world\"").unwrap();
        assert_eq!(
            cmds,
            vec![vec!["echo".to_string(), "hello world".to_string()]]
        );

        let cmds2 = parse_seq("echo 'hi there'").unwrap();
        assert_eq!(
            cmds2,
            vec![vec!["echo".to_string(), "hi there".to_string()]]
        );
    }

    #[test]
    fn accepts_double_quoted_strings_with_newlines() {
        let cmds = parse_seq("git commit -m \"line1\nline2\"").unwrap();
        assert_eq!(
            cmds,
            vec![vec![
                "git".to_string(),
                "commit".to_string(),
                "-m".to_string(),
                "line1\nline2".to_string(),
            ]]
        );
    }

    #[test]
    fn accepts_mixed_quote_concatenation() {
        assert_eq!(
            parse_seq(r#"echo "/usr"'/'"local"/bin"#).unwrap(),
            vec![vec!["echo".to_string(), "/usr/local/bin".to_string()]]
        );
        assert_eq!(
            parse_seq(r#"echo '/usr'"/"'local'/bin"#).unwrap(),
            vec![vec!["echo".to_string(), "/usr/local/bin".to_string()]]
        );
    }

    #[test]
    fn rejects_double_quoted_strings_with_expansions() {
        assert!(parse_seq(r#"echo "hi ${USER}""#).is_none());
        assert!(parse_seq(r#"echo "$HOME""#).is_none());
    }

    #[test]
    fn accepts_numbers_as_words() {
        let cmds = parse_seq("echo 123 456").unwrap();
        assert_eq!(
            cmds,
            vec![vec![
                "echo".to_string(),
                "123".to_string(),
                "456".to_string()
            ]]
        );
    }

    #[test]
    fn rejects_parentheses_and_subshells() {
        assert!(parse_seq("(ls)").is_none());
        assert!(parse_seq("ls || (pwd && echo hi)").is_none());
    }

    #[test]
    fn rejects_redirections_and_unsupported_operators() {
        assert!(parse_seq("ls > out.txt").is_none());
        assert!(parse_seq("echo hi & echo bye").is_none());
    }

    #[test]
    fn rejects_command_and_process_substitutions_and_expansions() {
        assert!(parse_seq("echo $(pwd)").is_none());
        assert!(parse_seq("echo `pwd`").is_none());
        assert!(parse_seq("echo $HOME").is_none());
        assert!(parse_seq("echo \"hi $USER\"").is_none());
    }

    #[test]
    fn rejects_variable_assignment_prefix() {
        assert!(parse_seq("FOO=bar ls").is_none());
    }

    #[test]
    fn rejects_trailing_operator_parse_error() {
        assert!(parse_seq("ls &&").is_none());
    }

    #[test]
    fn rejects_empty_command_position_with_leading_operator() {
        assert!(parse_seq("&& ls").is_none());
    }

    #[test]
    fn rejects_empty_command_position_with_double_separator() {
        assert!(parse_seq("ls ;; pwd").is_none());
    }

    #[test]
    fn rejects_empty_command_position_with_empty_pipeline_segment() {
        assert!(parse_seq("ls | | wc").is_none());
    }

    #[test]
    fn parse_zsh_lc_plain_commands() {
        let command = vec!["zsh".to_string(), "-lc".to_string(), "ls".to_string()];
        let parsed = parse_shell_lc_plain_commands(&command).unwrap();
        assert_eq!(parsed, vec![vec!["ls".to_string()]]);
    }

    #[test]
    fn accepts_concatenated_flag_and_value() {
        // Test case: -g"*.py" (flag directly concatenated with quoted value)
        let cmds = parse_seq("rg -n \"foo\" -g\"*.py\"").unwrap();
        assert_eq!(
            cmds,
            vec![vec![
                "rg".to_string(),
                "-n".to_string(),
                "foo".to_string(),
                "-g*.py".to_string(),
            ]]
        );
    }

    #[test]
    fn accepts_concatenated_flag_with_single_quotes() {
        let cmds = parse_seq("grep -n 'pattern' -g'*.txt'").unwrap();
        assert_eq!(
            cmds,
            vec![vec![
                "grep".to_string(),
                "-n".to_string(),
                "pattern".to_string(),
                "-g*.txt".to_string(),
            ]]
        );
    }

    #[test]
    fn rejects_concatenation_with_variable_substitution() {
        // Environment variables in concatenated strings should be rejected
        assert!(parse_seq("rg -g\"$VAR\" pattern").is_none());
        assert!(parse_seq("rg -g\"${VAR}\" pattern").is_none());
    }

    #[test]
    fn rejects_concatenation_with_command_substitution() {
        // Command substitution in concatenated strings should be rejected
        assert!(parse_seq("rg -g\"$(pwd)\" pattern").is_none());
        assert!(parse_seq("rg -g\"$(echo '*.py')\" pattern").is_none());
    }

    #[test]
    fn parse_shell_lc_single_command_prefix_supports_heredoc() {
        let command = vec![
            "zsh".to_string(),
            "-lc".to_string(),
            "python3 <<'PY'\nprint('hello')\nPY".to_string(),
        ];
        let parsed = parse_shell_lc_single_command_prefix(&command);
        assert_eq!(parsed, Some(vec!["python3".to_string()]));

        let command_unquoted = vec![
            "zsh".to_string(),
            "-lc".to_string(),
            "python3 << PY\nprint('hello')\nPY".to_string(),
        ];
        let parsed_unquoted = parse_shell_lc_single_command_prefix(&command_unquoted);
        assert_eq!(parsed_unquoted, Some(vec!["python3".to_string()]));
    }

    #[test]
    fn parse_shell_lc_single_command_prefix_rejects_multi_command_scripts() {
        let command = vec![
            "bash".to_string(),
            "-lc".to_string(),
            "python3 <<'PY'\nprint('hello')\nPY\necho done".to_string(),
        ];
        assert_eq!(parse_shell_lc_single_command_prefix(&command), None);
    }

    #[test]
    fn parse_shell_lc_single_command_prefix_rejects_non_heredoc_redirects() {
        let command = vec![
            "bash".to_string(),
            "-lc".to_string(),
            "echo hello > /tmp/out.txt".to_string(),
        ];
        assert_eq!(parse_shell_lc_single_command_prefix(&command), None);
    }

    #[test]
    fn parse_shell_lc_single_command_prefix_accepts_heredoc_with_extra_redirect() {
        let command = vec![
            "bash".to_string(),
            "-lc".to_string(),
            "python3 <<'PY' > /tmp/out.txt\nprint('hello')\nPY".to_string(),
        ];
        assert_eq!(
            parse_shell_lc_single_command_prefix(&command),
            Some(vec!["python3".to_string()])
        );
    }

    #[test]
    fn parse_shell_lc_single_command_prefix_rejects_herestring_with_chaining() {
        let command = vec![
            "bash".to_string(),
            "-lc".to_string(),
            r#"echo hello > /tmp/out.txt && cat /tmp/out.txt"#.to_string(),
        ];
        assert_eq!(parse_shell_lc_single_command_prefix(&command), None);
    }

    #[test]
    fn parse_shell_lc_single_command_prefix_rejects_herestring_with_substitution() {
        let command = vec![
            "bash".to_string(),
            "-lc".to_string(),
            r#"python3 <<< "$(rm -rf /)""#.to_string(),
        ];
        assert_eq!(parse_shell_lc_single_command_prefix(&command), None);
    }

    #[test]
    fn parse_shell_lc_single_command_prefix_rejects_arithmetic_shift_non_heredoc_script() {
        let command = vec![
            "bash".to_string(),
            "-lc".to_string(),
            "echo $((1<<2))".to_string(),
        ];
        assert_eq!(parse_shell_lc_single_command_prefix(&command), None);
    }

    #[test]
    fn parse_shell_lc_single_command_prefix_rejects_heredoc_command_with_word_expansion() {
        let command = vec![
            "bash".to_string(),
            "-lc".to_string(),
            "python3 $((1<<2)) <<'PY'\nprint('hello')\nPY".to_string(),
        ];
        assert_eq!(parse_shell_lc_single_command_prefix(&command), None);
    }
}
