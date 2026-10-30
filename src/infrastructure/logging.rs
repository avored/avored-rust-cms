use std::{env, fs, io, path::Path};

use chrono::{Duration, NaiveDate, Utc};
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const LOG_DIRECTORY: &str = "public/log";
const LOG_FILE_PREFIX: &str = "avored-rust-cms";
const LOG_RETENTION_DAYS: i64 = 14;

pub fn initialize_tracing() -> io::Result<WorkerGuard> {
    let log_directory = Path::new(LOG_DIRECTORY);
    fs::create_dir_all(log_directory)?;

    let filter = match env::var("RUST_LOG") {
        Ok(directives) => EnvFilter::try_new(directives).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid RUST_LOG filter: {error}"),
            )
        })?,
        Err(env::VarError::NotPresent) => EnvFilter::new("info"),
        Err(error) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unable to read RUST_LOG: {error}"),
            ));
        }
    };

    let file_appender = rolling::RollingFileAppender::builder()
        .rotation(rolling::Rotation::DAILY)
        .filename_prefix(LOG_FILE_PREFIX)
        .filename_suffix("log")
        .build(log_directory)
        .map_err(|error| io::Error::other(format!("failed to create daily log file: {error}")))?;
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(io::stdout)
                .with_ansi(false),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(file_writer)
                .with_ansi(false),
        )
        .try_init()
        .map_err(|error| io::Error::other(format!("failed to initialize tracing: {error}")))?;

    Ok(guard)
}

pub fn cleanup_expired_logs(log_directory: &Path) -> io::Result<usize> {
    let today = Utc::now().date_naive();
    let cutoff = today - Duration::days(LOG_RETENTION_DAYS);
    let mut removed = 0;

    for entry in fs::read_dir(log_directory)? {
        let entry = entry?;
        let path = entry.path();
        if !entry.file_type()?.is_file() {
            continue;
        }

        let Some(log_date) = log_date_from_path(&path) else {
            continue;
        };
        if log_date < cutoff && log_date != today {
            fs::remove_file(path)?;
            removed += 1;
        }
    }

    Ok(removed)
}

pub fn spawn_log_cleanup() {
    tokio::spawn(async {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(24 * 60 * 60));
        interval.tick().await;

        loop {
            interval.tick().await;
            match cleanup_expired_logs(Path::new(LOG_DIRECTORY)) {
                Ok(removed) if removed > 0 => {
                    tracing::info!(
                        removed,
                        retention_days = LOG_RETENTION_DAYS,
                        "removed expired log files"
                    );
                }
                Ok(_) => {}
                Err(error) => {
                    tracing::error!(%error, "failed to clean up expired log files");
                }
            }
        }
    });
}

fn log_date_from_path(path: &Path) -> Option<NaiveDate> {
    let file_name = path.file_name()?.to_str()?;
    let date = file_name.strip_prefix(&format!("{LOG_FILE_PREFIX}.log."))?;
    NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_log_directory() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after UNIX epoch")
            .as_nanos();
        let path = env::temp_dir().join(format!("avored-log-test-{unique}"));
        fs::create_dir_all(&path).expect("temporary log directory should be created");
        path
    }

    #[test]
    fn cleanup_removes_only_managed_logs_older_than_fourteen_days() {
        let directory = temporary_log_directory();
        let today = Utc::now().date_naive();
        let expired_date = today - Duration::days(LOG_RETENTION_DAYS + 1);
        let cutoff_date = today - Duration::days(LOG_RETENTION_DAYS);
        let expired_log = directory.join(format!("{LOG_FILE_PREFIX}.log.{expired_date}"));
        let retained_log = directory.join(format!("{LOG_FILE_PREFIX}.log.{cutoff_date}"));
        let unrelated_log = directory.join(format!("other-service.{expired_date}"));
        let malformed_name = directory.join(format!("{LOG_FILE_PREFIX}.old"));

        fs::write(&expired_log, "expired").expect("expired log should be written");
        fs::write(&retained_log, "retained").expect("cutoff log should be written");
        fs::write(&unrelated_log, "unrelated").expect("unrelated log should be written");
        fs::write(&malformed_name, "unmanaged").expect("malformed file should be written");

        assert_eq!(cleanup_expired_logs(&directory).unwrap(), 1);
        assert!(!expired_log.exists());
        assert!(retained_log.exists());
        assert!(unrelated_log.exists());
        assert!(malformed_name.exists());

        fs::remove_dir_all(directory).expect("temporary log directory should be removed");
    }

    #[test]
    fn cleanup_does_not_remove_current_daily_log() {
        let directory = temporary_log_directory();
        let current_log =
            directory.join(format!("{LOG_FILE_PREFIX}.log.{}", Utc::now().date_naive()));
        fs::write(&current_log, "current").expect("current log should be written");

        assert_eq!(cleanup_expired_logs(&directory).unwrap(), 0);
        assert!(current_log.exists());

        fs::remove_dir_all(directory).expect("temporary log directory should be removed");
    }

    #[test]
    fn log_date_parser_accepts_only_the_managed_daily_filename_format() {
        let date = NaiveDate::from_ymd_opt(2026, 10, 31).unwrap();
        assert_eq!(
            log_date_from_path(Path::new("avored-rust-cms.log.2026-10-31")),
            Some(date)
        );
        assert_eq!(
            log_date_from_path(Path::new("other-service.2026-10-31")),
            None
        );
        assert_eq!(
            log_date_from_path(Path::new("avored-rust-cms.log.2026-10-31.log")),
            None
        );
    }
}
