#![allow(dead_code)]

use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy)]
enum Endian {
    Little,
    Big,
}

impl Endian {
    fn u16(self, value: u16) -> [u8; 2] {
        match self {
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        }
    }

    fn u32(self, value: u32) -> [u8; 4] {
        match self {
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        }
    }
}

pub struct Elf32 {
    bytes: Vec<u8>,
}

impl Elf32 {
    pub const HEADER_SIZE: usize = 52;
    pub const SECTION_SIZE: usize = 40;

    const NAMES: &'static [u8] = b"\0.shstrtab\0";
    const MAGIC: Range<usize> = 0..4;
    const CLASS: usize = 4;
    const DATA: usize = 5;
    const IDENT_VERSION: usize = 6;
    const FILE_TYPE: Range<usize> = 16..18;
    const MACHINE: Range<usize> = 18..20;
    const VERSION: Range<usize> = 20..24;
    const SECTION_TABLE: Range<usize> = 32..36;
    const HEADER_SIZE_FIELD: Range<usize> = 40..42;
    const SECTION_ENTRY_SIZE: Range<usize> = 46..48;
    const SECTION_COUNT: Range<usize> = 48..50;
    const NAMES_INDEX: Range<usize> = 50..52;

    pub fn headerless() -> Self {
        Self::build(0, Endian::Little, false, false)
    }

    pub fn with_headers(headers: u16) -> Self {
        assert!(
            headers < 0xff00,
            "reserved counts need SHN_LORESERVE encoding"
        );
        Self::build(u32::from(headers), Endian::Little, false, false)
    }

    pub fn with_headers_big_endian(headers: u16) -> Self {
        assert!(
            headers < 0xff00,
            "reserved counts need SHN_LORESERVE encoding"
        );
        Self::build(u32::from(headers), Endian::Big, false, false)
    }

    pub fn with_names(headers: u16) -> Self {
        assert!(headers >= 2, "a name table needs two headers");
        assert!(
            headers < 0xff00,
            "reserved counts need SHN_LORESERVE encoding"
        );
        Self::build(u32::from(headers), Endian::Little, true, false)
    }

    /// Extended numbering: `e_shnum` is zero and section 0 holds the count.
    pub fn extended(headers: u32) -> Self {
        assert!(
            headers >= 0xff00,
            "extended numbering starts at SHN_LORESERVE"
        );
        Self::build(headers, Endian::Little, false, true)
    }

    /// A reserved value written as a literal count.
    pub fn with_reserved_count(count: u16) -> Self {
        assert!(count >= 0xff00, "reserved counts start at SHN_LORESERVE");
        Self::build(u32::from(count), Endian::Little, false, false)
    }

    /// A nonzero count with no section table.
    pub fn with_missing_table(count: u16) -> Self {
        assert!(count > 0, "a missing table still needs a claimed count");
        let mut elf = Self::build(u32::from(count), Endian::Little, false, false);
        elf.bytes[Self::SECTION_TABLE].fill(0);
        elf
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    fn build(headers: u32, endian: Endian, names: bool, extended: bool) -> Self {
        let count = if extended {
            0
        } else {
            u16::try_from(headers).expect("a literal count fits u16")
        };
        let table_size = headers as usize * Self::SECTION_SIZE;
        let names_offset = Self::HEADER_SIZE + table_size;
        let extra = if names { Self::NAMES.len() } else { 0 };
        let mut bytes = vec![0; names_offset + extra];

        bytes[Self::MAGIC].copy_from_slice(b"\x7fELF");
        bytes[Self::CLASS] = 1; // ELFCLASS32
        bytes[Self::DATA] = match endian {
            Endian::Little => 1, // ELFDATA2LSB
            Endian::Big => 2,    // ELFDATA2MSB
        };
        bytes[Self::IDENT_VERSION] = 1; // EV_CURRENT
        bytes[Self::FILE_TYPE].copy_from_slice(&endian.u16(2)); // ET_EXEC
        bytes[Self::MACHINE].copy_from_slice(&endian.u16(3)); // EM_386
        bytes[Self::VERSION].copy_from_slice(&endian.u32(1));
        bytes[Self::HEADER_SIZE_FIELD].copy_from_slice(&endian.u16(Self::HEADER_SIZE as u16));
        bytes[Self::SECTION_ENTRY_SIZE].copy_from_slice(&endian.u16(Self::SECTION_SIZE as u16));

        if headers == 0 {
            return Self { bytes };
        }

        bytes[Self::SECTION_TABLE].copy_from_slice(&endian.u32(Self::HEADER_SIZE as u32));
        bytes[Self::SECTION_COUNT].copy_from_slice(&endian.u16(count));

        if extended {
            let section = Self::HEADER_SIZE;
            bytes[section + 20..section + 24].copy_from_slice(&endian.u32(headers));
        }

        if names {
            bytes[Self::NAMES_INDEX].copy_from_slice(&endian.u16(1));
            let section = Self::HEADER_SIZE + Self::SECTION_SIZE;
            bytes[section..section + 4].copy_from_slice(&endian.u32(1)); // sh_name
            bytes[section + 4..section + 8].copy_from_slice(&endian.u32(3)); // SHT_STRTAB
            bytes[section + 16..section + 20].copy_from_slice(&endian.u32(names_offset as u32));
            bytes[section + 20..section + 24]
                .copy_from_slice(&endian.u32(Self::NAMES.len() as u32));
            bytes[names_offset..].copy_from_slice(Self::NAMES);
        }

        Self { bytes }
    }
}

pub fn workspace(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create workspace");
    dir
}

pub fn linked_clean_elf(dir: &Path) -> PathBuf {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let object = dir.join("clean.o");
    let elf = dir.join("clean.elf");

    let assembled = Command::new("as")
        .args(["--32", "-mx86-used-note=no"])
        .arg(fixtures.join("clean.S"))
        .arg("-o")
        .arg(&object)
        .output()
        .expect("run as");
    assert!(
        assembled.status.success(),
        "as failed: {}",
        String::from_utf8_lossy(&assembled.stderr)
    );

    let linked = Command::new("ld")
        .args(["-m", "elf_i386"])
        .arg("-T")
        .arg(fixtures.join("clean.ld"))
        .arg(&object)
        .arg("-o")
        .arg(&elf)
        .output()
        .expect("run ld");
    assert!(
        linked.status.success(),
        "ld failed: {}",
        String::from_utf8_lossy(&linked.stderr)
    );

    elf
}
