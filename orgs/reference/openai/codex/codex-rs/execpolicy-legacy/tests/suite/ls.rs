extern crate codex_execpolicy_legacy;

use codex_execpolicy_legacy::ArgType;
use codex_execpolicy_legacy::Error;
use codex_execpolicy_legacy::ExecCall;
use codex_execpolicy_legacy::MatchedArg;
use codex_execpolicy_legacy::MatchedExec;
use codex_execpolicy_legacy::MatchedFlag;
use codex_execpolicy_legacy::Policy;
use codex_execpolicy_legacy::Result;
use codex_execpolicy_legacy::ValidExec;
use codex_execpolicy_legacy::get_default_policy;

#[expect(clippy::expect_used)]
fn setup() -> Policy {
    get_default_policy().expect("failed to load default policy")
}

#[test]
fn test_ls_no_args() {
    let policy = setup();
    let ls = ExecCall:REDACTED_SECRET"ls", &[]);
    assert_eq!(
        Ok(MatchedExec::Match {
            exec: ValidExec:REDACTED_SECRET"ls", vec![], &["/bin/ls", "/usr/bin/ls"])
        }),
        policy.check(&ls)
    );
}

#[test]
fn test_ls_dash_a_dash_l() {
    let policy = setup();
    let args = &["-a", "-l"];
    let ls_a_l = ExecCall:REDACTED_SECRET"ls", args);
    assert_eq!(
        Ok(MatchedExec::Match {
            exec: ValidExec {
                program: "ls".into(),
                flags: vec![MatchedFlag:REDACTED_SECRET"-a"), MatchedFlag:REDACTED_SECRET"-l")],
                system_path: ["/bin/ls".into(), "/usr/bin/ls".into()].into(),
                ..Default::default()
            }
        }),
        policy.check(&ls_a_l)
    );
}

#[test]
fn test_ls_dash_z() {
    let policy = setup();

    // -z is currently an invalid option for ls, but it has so many options,
    // perhaps it will get added at some point...
    let ls_z = ExecCall:REDACTED_SECRET"ls", &["-z"]);
    assert_eq!(
        Err(Error::UnknownOption {
            program: "ls".into(),
            option: "-z".into()
        }),
        policy.check(&ls_z)
    );
}

#[test]
fn test_ls_dash_al() {
    let policy = setup();

    // This currently fails, but it should pass once option_bundling=True is implemented.
    let ls_al = ExecCall:REDACTED_SECRET"ls", &["-al"]);
    assert_eq!(
        Err(Error::UnknownOption {
            program: "ls".into(),
            option: "-al".into()
        }),
        policy.check(&ls_al)
    );
}

#[test]
fn test_ls_one_file_arg() -> Result<()> {
    let policy = setup();

    let ls_one_file_arg = ExecCall:REDACTED_SECRET"ls", &["foo"]);
    assert_eq!(
        Ok(MatchedExec::Match {
            exec: ValidExec:REDACTED_SECRET
                "ls",
                vec![MatchedArg:REDACTED_SECRET
                    /*index*/ 0,
                    ArgType::ReadableFile,
                    "foo"
                )?],
                &["/bin/ls", "/usr/bin/ls"]
            )
        }),
        policy.check(&ls_one_file_arg)
    );
    Ok(())
}

#[test]
fn test_ls_multiple_file_args() -> Result<()> {
    let policy = setup();

    let ls_multiple_file_args = ExecCall:REDACTED_SECRET"ls", &["foo", "bar", "baz"]);
    assert_eq!(
        Ok(MatchedExec::Match {
            exec: ValidExec:REDACTED_SECRET
                "ls",
                vec![
                    MatchedArg:REDACTED_SECRET/*index*/ 0, ArgType::ReadableFile, "foo")?,
                    MatchedArg:REDACTED_SECRET/*index*/ 1, ArgType::ReadableFile, "bar")?,
                    MatchedArg:REDACTED_SECRET/*index*/ 2, ArgType::ReadableFile, "baz")?,
                ],
                &["/bin/ls", "/usr/bin/ls"]
            )
        }),
        policy.check(&ls_multiple_file_args)
    );
    Ok(())
}

#[test]
fn test_ls_multiple_flags_and_file_args() -> Result<()> {
    let policy = setup();

    let ls_multiple_flags_and_file_args = ExecCall:REDACTED_SECRET"ls", &["-l", "-a", "foo", "bar", "baz"]);
    assert_eq!(
        Ok(MatchedExec::Match {
            exec: ValidExec {
                program: "ls".into(),
                flags: vec![MatchedFlag:REDACTED_SECRET"-l"), MatchedFlag:REDACTED_SECRET"-a")],
                args: vec![
                    MatchedArg:REDACTED_SECRET/*index*/ 2, ArgType::ReadableFile, "foo")?,
                    MatchedArg:REDACTED_SECRET/*index*/ 3, ArgType::ReadableFile, "bar")?,
                    MatchedArg:REDACTED_SECRET/*index*/ 4, ArgType::ReadableFile, "baz")?,
                ],
                system_path: ["/bin/ls".into(), "/usr/bin/ls".into()].into(),
                ..Default::default()
            }
        }),
        policy.check(&ls_multiple_flags_and_file_args)
    );
    Ok(())
}

#[test]
fn test_flags_after_file_args() -> Result<()> {
    let policy = setup();

    // TODO(mbolin): While this is "safe" in that it will not do anything bad
    // to the user's machine, it will fail because apparently `ls` does not
    // allow flags after file arguments (as some commands do). We should
    // extend define_program() to make this part of the configuration so that
    // this command is disallowed.
    let ls_flags_after_file_args = ExecCall:REDACTED_SECRET"ls", &["foo", "-l"]);
    assert_eq!(
        Ok(MatchedExec::Match {
            exec: ValidExec {
                program: "ls".into(),
                flags: vec![MatchedFlag:REDACTED_SECRET"-l")],
                args: vec![MatchedArg:REDACTED_SECRET
                    /*index*/ 0,
                    ArgType::ReadableFile,
                    "foo"
                )?],
                system_path: ["/bin/ls".into(), "/usr/bin/ls".into()].into(),
                ..Default::default()
            }
        }),
        policy.check(&ls_flags_after_file_args)
    );
    Ok(())
}
