use minigrep::app;
use minigrep::cli::Args;
use std::io::Write;
use tempfile;

#[test]
#[should_panic]
fn program_exits_given_missing_filepath() {
    let args: Args = Args {
        pattern: "".to_string(),
        filepath: "invalid".into(),
    };

    _ = app(&args);
}

#[test]
fn app_returns_nothing_given_empty_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = tempfile::NamedTempFile::new()?;
    let args: Args = Args {
        pattern: "".to_string(),
        filepath: file.path().into(),
    };

    let matches = app(&args)?;

    assert_eq!(0, matches.len());

    Ok(())
}

#[test]
fn app_returns_nothing_given_empty_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let mut file: tempfile::NamedTempFile = tempfile::NamedTempFile::new()?;
    writeln!(file, "hello")?;
    writeln!(file, "world")?;
    let args: Args = Args {
        pattern: "".to_string(),
        filepath: file.path().into(),
    };

    let matches = app(&args)?;

    assert_eq!(0, matches.len());
    Ok(())
}

#[test]
fn app_returns_nothing_given_pattern_with_no_match() -> Result<(), Box<dyn std::error::Error>> {
    let mut file: tempfile::NamedTempFile = tempfile::NamedTempFile::new()?;
    writeln!(file, "hello")?;
    writeln!(file, "world")?;
    let args: Args = Args {
        pattern: "no match".to_string(),
        filepath: file.path().into(),
    };

    let matches = app(&args)?;

    assert_eq!(0, matches.len());
    Ok(())
}

#[test]
fn app_returns_matching_line_given_matching_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let mut file: tempfile::NamedTempFile = tempfile::NamedTempFile::new()?;
    writeln!(file, "hello")?;
    writeln!(file, "world")?;
    let args: Args = Args {
        pattern: "he".to_string(),
        filepath: file.path().into(),
    };

    let matches = app(&args)?;

    assert_eq!(1, matches.len());
    assert_eq!("hello", matches[0]);
    Ok(())
}
