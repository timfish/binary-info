use goblin::{
    elf64::header::{EM_AARCH64, EM_ARM, EM_X86_64},
    mach::{cputype::get_arch_name_from_types, Mach},
    pe::header::{COFF_MACHINE_ARM64, COFF_MACHINE_X86, COFF_MACHINE_X86_64},
    Object as Obj,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[napi(object)]
#[derive(Serialize, Deserialize, Debug)]
pub struct BinaryInfo {
    pub platform: String,
    pub arch: String,
}

#[napi]
pub fn get_info(path: String) -> Result<BinaryInfo> {
    let path = Path::new(&path);
    let buffer = fs::read(path)?;

    match Obj::parse(&buffer).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))? {
        Obj::Elf(elf) => {
            let arch = match elf.header.e_machine {
                EM_AARCH64 => "arm64",
                EM_X86_64 => "x64",
                EM_ARM => "arm",
                _ => {
                    return Err(Error::new(
                        Status::GenericFailure,
                        "Unknown architecture".to_string(),
                    ))
                }
            };

            Ok(BinaryInfo {
                platform: "linux".to_string(),
                arch: arch.to_string(),
            })
        }
        Obj::PE(pe) => {
            let arch = match pe.header.coff_header.machine {
                COFF_MACHINE_ARM64 => "arm64",
                COFF_MACHINE_X86 => "ia32",
                COFF_MACHINE_X86_64 => "x64",
                _ => {
                    return Err(Error::new(
                        Status::GenericFailure,
                        "Unknown architecture".to_string(),
                    ))
                }
            };

            Ok(BinaryInfo {
                platform: "win32".to_string(),
                arch: arch.to_string(),
            })
        }
        Obj::Mach(mach) => match mach {
            Mach::Fat(_) => todo!(),
            Mach::Binary(mach_o) => {
                let arch = match get_arch_name_from_types(
                    mach_o.header.cputype(),
                    mach_o.header.cpusubtype(),
                ) {
                    Some("x86_64") => "x64",
                    Some("arm64") => "arm64",
                    _ => {
                        return Err(Error::new(
                            Status::GenericFailure,
                            "Unknown architecture".to_string(),
                        ))
                    }
                };

                Ok(BinaryInfo {
                    platform: "darwin".to_string(),
                    arch: arch.to_string(),
                })
            }
        },
        _ => Err(Error::new(
            Status::GenericFailure,
            "Not a binary".to_string(),
        )),
    }
}
