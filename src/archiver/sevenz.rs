use sevenz_rust::{Password, SevenZArchiveEntry, SevenZWriter};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use crate::archiver::{Archiver, ArchiverMode};
use crate::shared_utils::{AppError, AppResult};
use crate::task::ArchiveJob;

#[derive(Debug, Clone)]
pub(super) struct Archiver7z {}

impl Archiver for Archiver7z {
    fn exec(&self, job: &ArchiveJob) -> AppResult<bool> {
        if !self.job_check(&job) {
            return Err(AppError::Archiver("Archiver7z:Invalid job".to_string()));
        }
        match job.mode {
            super::ArchiverMode::Archive => self.archive(job),
            super::ArchiverMode::Extract => self.extract(job),
            super::ArchiverMode::Unknown => {
                Err(AppError::Archiver("Archiver7z:Unknown mode.".to_string()))
            }
            super::ArchiverMode::List => match self.list(job) {
                Ok(items) => {
                    for item in items.iter() {
                        print!("{:?}", item);
                    }
                    return Ok(true);
                }
                Err(e) => Err(e),
            },
        }
    }

    fn archive_support_check(&self, path: String, mode: super::ArchiverMode) -> bool {
        if mode == ArchiverMode::Unknown {
            return false;
        }
        return path.ends_with(".7z");
    }

    fn avaliable_options(&self, mode: super::ArchiverMode) -> HashMap<String, String> {
        return HashMap::new(); // TODO
    }

    fn job_check(&self, job: &ArchiveJob) -> bool {
        return true; // TODO
    }
}

impl Archiver7z {
    fn archive(&self, job: &ArchiveJob) -> AppResult<bool> {
        let file_handle =
            File::create(job.target_path.clone()).expect("Archive7z: Can't create file");
        match SevenZWriter::new(file_handle) {
            Ok(writer) => {
                write_sevenz_impl(writer, job.source_paths.clone(), true);
                return Ok(true);
            }
            Err(e) => return Err(AppError::Archiver(e.to_string())),
        }
    }

    fn extract(&self, job: &ArchiveJob) -> AppResult<bool> {
        for s_path in job.source_paths.iter() {
            if !(s_path.exists() && s_path.is_file()) {
                println!("attn point:{:?}", s_path)
            }
            let mut file = File::open(s_path).expect("Archive7z: file handle failed.");
            let len = file.metadata().unwrap().len();
            let password = Password::empty();
            let archive = match sevenz_rust::Archive::read(&mut file, len, password.as_ref()) {
                Ok(reader) => reader,
                Err(e) => return Err(AppError::Fatal(Box::new(e))),
            };
            let folder_count = archive.folders.len();
            for findex in 0..folder_count {
                let folder_decoder = sevenz_rust::BlockDecoder::new(
                    findex,
                    &archive,
                    password.as_slice(),
                    &mut file,
                );
                if let Err(e) = folder_decoder.for_each_entries(&mut |entry, reader| {
                    let dest = job.target_path.join(entry.name.clone());
                    sevenz_rust::default_entry_extract_fn(entry, reader, &dest)
                }) {
                    return Err(AppError::Fatal(Box::new(e)));
                }
            }
        }

        Ok(true)
    }

    fn list(&self, job: &ArchiveJob) -> AppResult<Vec<String>> {
        for s_path in job.source_paths.iter() {
            let mut reader = File::open(s_path).unwrap();
            let len = reader.metadata().unwrap().len();
            match sevenz_rust::Archive::read(&mut reader, len, Password::empty().as_ref()) {
                Ok(archive) => {
                    let mut r = Vec::<String>::new();
                    for entry in &archive.files {
                        r.push(entry.name.clone())
                    }
                    return Ok(r);
                }
                Err(e) => return Err(AppError::Fatal(Box::new(e))),
            }
        }
        return Ok(Vec::new());
    }
}

fn process_file(szw: &mut SevenZWriter<File>, target: PathBuf) -> AppResult<()> {
    let name = target.to_str().unwrap();
    if let Err(e) = szw.push_archive_entry(
        SevenZArchiveEntry::from_path(&target, name.to_string()),
        Some(File::open(target).unwrap()),
    ) {
        return Err(AppError::Archiver(e.to_string()));
    }
    Ok(())
}

fn process_dir(szw: &mut SevenZWriter<File>, target: PathBuf) -> AppResult<()> {
    for entry in target.read_dir().unwrap() {
        if let Ok(e) = entry {
            let p = e.path();
            if p.is_dir() {
                process_dir(szw, e.path())?
            } else if p.is_file() {
                process_file(szw, e.path())?
            }
        }
    }
    Ok(())
}

fn write_sevenz_impl(
    mut szw: SevenZWriter<File>,
    paths: Vec<PathBuf>,
    recursive: bool,
) -> AppResult<()> {
    for target in paths {
        let path = target.as_path();
        if path.is_dir() && recursive {
            process_dir(&mut szw, path.to_path_buf())?
        } else {
            process_file(&mut szw, path.to_path_buf())?
        }
    }
    if let Err(e) = szw.finish() {
        return Err(AppError::Archiver(e.to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_list() {
        let archiver = Archiver7z {};
        let file = PathBuf::from("testdata/test.7z");
        let mut tmp_job = ArchiveJob::new();
        tmp_job.source_paths.push(file);
        match archiver.list(&tmp_job) {
            Ok(r) => {
                assert_eq!(r.len(), 21);
                assert_eq!(r.get(0), Some("Cargo.toml".to_string()).as_ref());
                assert_eq!(r.get(1), Some("build.rs".to_string()).as_ref());
                assert_eq!(r.get(2), Some("LICENSE".to_string()).as_ref());
                assert_eq!(r.get(3), Some("README.md".to_string()).as_ref());
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_extract_archive() {
        let archiver = Archiver7z {};
        let file = PathBuf::from("testdata/test.7z");
        let tmp_job = ArchiveJob {
            source_paths: vec![file],
            target_path: PathBuf::from("results/sevenz"),
            archiver: Box::new(Archiver7z {}),
            mode: crate::archiver::ArchiverMode::Extract,
            overwrite: true,
            with_creation: true,
            options: HashMap::new(),
        };
        match archiver.extract(&tmp_job) {
            Ok(_) => {
                assert!(true);
                assert!(PathBuf::from("results/sevenz/Cargo.toml").exists());
                std::fs::remove_dir_all(PathBuf::from("results/sevenz")).unwrap();
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false)
            }
        };
    }

    #[test]
    fn test_format() {
        let archiver = Archiver7z {};
        assert!(archiver.archive_support_check("test.7z".to_string(), ArchiverMode::Archive));
        assert!(archiver.archive_support_check("test.7z".to_string(), ArchiverMode::Extract));
        assert!(archiver.archive_support_check("test.7z".to_string(), ArchiverMode::List));
        assert!(!archiver.archive_support_check("test.7z".to_string(), ArchiverMode::Unknown));
    }

    fn run_test<F>(f: F)
    where
        F: FnOnce(),
    {
        // setup(); // 予めやりたい処理
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
        teardown(); // 後片付け処理

        if let Err(err) = result {
            std::panic::resume_unwind(err);
        }
    }

    #[test]
    fn test_zip() {
        run_test(|| {
            let archiver: Archiver7z = Archiver7z {};
            let tmp_job = ArchiveJob {
                source_paths: vec!["src", "Cargo.toml"]
                    .iter()
                    .map(|&s| PathBuf::from(s))
                    .collect(),
                target_path: PathBuf::from("results/test.7z"),
                archiver: Box::new(archiver.clone()),
                mode: ArchiverMode::Archive,
                overwrite: true,
                with_creation: true,
                options: HashMap::new(),
            };
            let result = archiver.archive(&tmp_job);
            assert!(result.is_ok());
        });
    }

    fn teardown() {
        let _ = std::fs::remove_file("results/test.7z");
    }
}
