use std::fs::File;
use std::path::PathBuf;

use sevenz_rust::{Password, SevenZArchiveEntry, SevenZWriter};

use crate::archiver::Archiver;
use crate::shared_utils::{AppError, AppResult};
use crate::task::ArchiveJob;

#[derive(Debug)]
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
            super::ArchiverMode::List => self.list(job),
        }
    }

    fn archive_support_check(&self, format: String, mode: super::ArchiverMode) -> bool {
        todo!()
    }

    fn avaliable_options(
        &self,
        mode: super::ArchiverMode,
    ) -> std::collections::HashMap<String, String> {
        todo!()
    }

    fn job_check(&self, job: &ArchiveJob) -> bool {
        return true;
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
            let len = s_path.metadata().unwrap().len();
            let mut file = File::create(s_path).expect("Archive7z: file handle failed.");
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

    fn list(&self, job: &ArchiveJob) -> AppResult<bool> {
        todo!()
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
