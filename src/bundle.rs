use std::{
    env,
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    process::Command,
};

use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};

use crate::error::{Error, Result};

const MAGIC: &[u8; 16] = b"SANGOKU-BUNDLE1!";
const FOOTER_SIZE: u64 = 56;

struct Payload {
    offset: u64,
    length: u64,
    digest: [u8; 32],
}

fn payload(file: &mut File) -> Result<Option<Payload>> {
    let size = file
        .metadata()
        .map_err(|e| Error::io("read executable metadata", e))?
        .len();
    if size < FOOTER_SIZE {
        return Ok(None);
    }
    file.seek(SeekFrom::End(-(FOOTER_SIZE as i64)))
        .map_err(|e| Error::io("seek bundle footer", e))?;
    let mut footer = [0; FOOTER_SIZE as usize];
    file.read_exact(&mut footer)
        .map_err(|e| Error::io("read bundle footer", e))?;
    if &footer[40..] != MAGIC {
        return Ok(None);
    }
    let length = u64::from_le_bytes(footer[..8].try_into().unwrap());
    let offset = (size - FOOTER_SIZE)
        .checked_sub(length)
        .ok_or_else(|| Error::Bundle("invalid embedded archive length".into()))?;
    Ok(Some(Payload {
        offset,
        length,
        digest: footer[8..40].try_into().unwrap(),
    }))
}

fn extract(file: &mut File, payload: &Payload, cache: &Path) -> Result<PathBuf> {
    let id = payload
        .digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let root = cache.join(id);
    if root.join(".complete").is_file() {
        return Ok(root);
    }
    fs::create_dir_all(cache).map_err(|e| Error::io("create toolchain cache", e))?;
    file.seek(SeekFrom::Start(payload.offset))
        .map_err(|e| Error::io("seek embedded archive", e))?;
    let mut hash = Sha256::new();
    let mut limited = (&mut *file).take(payload.length);
    let mut buffer = [0; 65536];
    loop {
        let read = limited
            .read(&mut buffer)
            .map_err(|e| Error::io("read embedded archive", e))?;
        if read == 0 {
            break;
        }
        hash.update(&buffer[..read]);
    }
    if hash.finalize().as_slice() != payload.digest {
        return Err(Error::Bundle("embedded archive checksum mismatch".into()));
    }
    eprintln!("Unpacking compiler toolchain into {}", root.display());
    let staging = tempfile::Builder::new()
        .prefix(".unpack-")
        .tempdir_in(cache)
        .map_err(|e| Error::io("create toolchain staging directory", e))?;
    file.seek(SeekFrom::Start(payload.offset))
        .map_err(|e| Error::io("seek embedded archive", e))?;
    let compressed = GzDecoder::new((&mut *file).take(payload.length));
    tar::Archive::new(compressed)
        .unpack(staging.path())
        .map_err(|e| Error::io("unpack compiler toolchain", e))?;
    let version = fs::read_to_string(staging.path().join("bundle-version"))
        .map_err(|e| Error::io("read bundled toolchain version", e))?;
    if version.trim() != "1" {
        return Err(Error::Bundle(
            "unsupported embedded toolchain version".into(),
        ));
    }
    fs::write(staging.path().join(".complete"), b"1\n")
        .map_err(|e| Error::io("mark toolchain extraction complete", e))?;
    if let Err(error) = fs::rename(staging.path(), &root)
        && !root.join(".complete").is_file()
    {
        return Err(Error::io("install extracted toolchain", error));
    }
    Ok(root)
}

pub fn root() -> Result<PathBuf> {
    let executable = env::current_exe().map_err(|e| Error::io("locate executable", e))?;
    let mut file = File::open(executable).map_err(|e| Error::io("open executable", e))?;
    let Some(payload) = payload(&mut file)? else {
        return Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    };
    let cache = env::var_os("SANGOKU_CACHE_DIR")
        .map(PathBuf::from)
        .or_else(|| env::var_os("XDG_CACHE_HOME").map(|p| PathBuf::from(p).join("sangoku")))
        .or_else(|| env::var_os("HOME").map(|p| PathBuf::from(p).join(".cache/sangoku")))
        .ok_or_else(|| {
            Error::Bundle("set SANGOKU_CACHE_DIR to a writable cache directory".into())
        })?;
    let cache = std::path::absolute(cache).map_err(|e| Error::io("resolve toolchain cache", e))?;
    extract(&mut file, &payload, &cache.join("toolchains"))
}

pub fn is_bundled(root: &Path) -> bool {
    root.join("bundle-version").is_file()
}

pub fn command(root: &Path, executable: &Path) -> Command {
    let mut command = Command::new(root.join("toolchain/lib/ld-linux-x86-64.so.2"));
    command
        .arg("--library-path")
        .arg(root.join("toolchain/lib"))
        .arg(executable);
    command
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::{Compression, write::GzEncoder};
    use std::io::Write;

    fn package(directory: &Path, version: &str) -> PathBuf {
        let compressed = GzEncoder::new(Vec::new(), Compression::fast());
        let mut archive = tar::Builder::new(compressed);
        for (name, contents) in [("bundle-version", version), ("runtime/test.asm", "ret\n")] {
            let mut header = tar::Header::new_gnu();
            header.set_size(contents.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            archive
                .append_data(&mut header, name, contents.as_bytes())
                .unwrap();
        }
        let bytes = archive.into_inner().unwrap().finish().unwrap();
        let path = directory.join("executable");
        let mut file = File::create(&path).unwrap();
        file.write_all(b"launcher").unwrap();
        file.write_all(&bytes).unwrap();
        file.write_all(&(bytes.len() as u64).to_le_bytes()).unwrap();
        file.write_all(&Sha256::digest(&bytes)).unwrap();
        file.write_all(MAGIC).unwrap();
        path
    }

    #[test]
    fn ordinary_executable_has_no_payload() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("plain");
        fs::write(&path, [0u8; 100]).unwrap();
        assert!(payload(&mut File::open(path).unwrap()).unwrap().is_none());
    }

    #[test]
    fn extracts_once_and_reuses_the_cache() {
        let directory = tempfile::tempdir().unwrap();
        let path = package(directory.path(), "1\n");
        let mut file = File::open(path).unwrap();
        let payload = payload(&mut file).unwrap().unwrap();
        let cache = directory.path().join("cache");
        let root = extract(&mut file, &payload, &cache).unwrap();
        assert_eq!(
            fs::read_to_string(root.join("runtime/test.asm")).unwrap(),
            "ret\n"
        );
        fs::write(root.join("retained"), "yes").unwrap();
        assert_eq!(extract(&mut file, &payload, &cache).unwrap(), root);
        assert!(root.join("retained").is_file());
        assert_eq!(fs::read_dir(cache).unwrap().count(), 1);
    }

    #[test]
    fn rejects_corrupt_archives_and_invalid_lengths() {
        let directory = tempfile::tempdir().unwrap();
        let path = package(directory.path(), "1\n");
        let mut file = File::options().read(true).write(true).open(path).unwrap();
        let info = payload(&mut file).unwrap().unwrap();
        file.seek(SeekFrom::Start(info.offset)).unwrap();
        file.write_all(&[0]).unwrap();
        let cache = directory.path().join("cache");
        assert!(
            extract(&mut file, &info, &cache)
                .unwrap_err()
                .to_string()
                .contains("checksum")
        );
        assert_eq!(fs::read_dir(cache).unwrap().count(), 0);
        file.seek(SeekFrom::End(-(FOOTER_SIZE as i64))).unwrap();
        file.write_all(&u64::MAX.to_le_bytes()).unwrap();
        assert!(payload(&mut file).is_err());
    }

    #[test]
    fn concurrent_extraction_installs_a_complete_directory() {
        let directory = tempfile::tempdir().unwrap();
        let path = package(directory.path(), "1\n");
        let cache = directory.path().join("cache");
        let paths = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..2)
                .map(|_| {
                    scope.spawn(|| {
                        let mut file = File::open(&path).unwrap();
                        let payload = payload(&mut file).unwrap().unwrap();
                        extract(&mut file, &payload, &cache).unwrap()
                    })
                })
                .collect();
            handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .collect::<Vec<_>>()
        });
        assert_eq!(paths[0], paths[1]);
        assert!(paths[0].join(".complete").is_file());
        assert_eq!(fs::read_dir(cache).unwrap().count(), 1);
    }
}
