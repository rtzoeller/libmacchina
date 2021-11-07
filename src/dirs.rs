#![allow(dead_code)]
use crate::extra::read_lines;
use std::path::PathBuf;

pub fn pkgdb_dir() -> Option<PathBuf> {
    if cfg!(target_os = "netbsd") {
        if let Ok(lines) = read_lines("/etc/mk.conf") {
            for line in lines {
                if let Ok(var) = line {
                    if var.starts_with("PKG_DBDIR") {
                        let pkg_db =
                            PathBuf::from(var.split("=").nth(1).unwrap().trim().to_string());
                        if pkg_db.is_dir() {
                            return Some(pkg_db);
                        }
                    }

                    continue;
                }
            }
        }

        return Some(PathBuf::from("/usr/pkg/pkgdb"));
    }

    None
}

pub fn localbase_dir() -> Option<PathBuf> {
    if cfg!(target_os = "netbsd") {
        if let Ok(lines) = read_lines("/etc/mk.conf") {
            for line in lines {
                if let Ok(var) = line {
                    if var.starts_with("LOCALBASE") {
                        let localbase =
                            PathBuf::from(var.split("=").nth(1).unwrap().trim().to_string());
                        if localbase.is_dir() {
                            return Some(localbase);
                        }
                    }

                    continue;
                }
            }
        }

        return Some(PathBuf::from("/usr/pkg"));
    }

    None
}

pub fn usr_share_dir() -> Option<PathBuf> {
    Some(PathBuf::from("/usr/share"))
}