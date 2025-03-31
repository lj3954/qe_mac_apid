#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(unused)]
#[allow(non_camel_case_types)]
#[allow(improper_ctypes)]
mod modelinfo;

mod io_subset;
mod oui;
mod plist_data;
mod qcow2;
mod serial;

use std::{
    io::{Seek, SeekFrom, Write},
    path::PathBuf,
};

use anyhow::{bail, Context, Result};
use clap::Parser;
use fatfs::FileSystem;
use gpt::{disk::LogicalBlockSize, GptConfig};
use io_subset::IoSubset;
use oui::APPLE_OUIS;
use plist_data::MacPlist;
use qcow2::Qcow2;
use rand::seq::IndexedRandom;
use uuid::Uuid;

const HEX_DIGITS: &[u8] = b"01234556789abcdef";

fn main() -> Result<()> {
    let args = Args::parse();
    env_logger::builder()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let mut qcow2 = Qcow2::new(&args.bootloader, args.dry_run)?;
    let mut first_partition = first_partition_subset(&mut qcow2)?;

    let fs = FileSystem::new(&mut first_partition, fatfs::FsOptions::new())
        .context("Failed to open FAT32 filesystem")?;

    let mut conf_plist = fs
        .root_dir()
        .open_file("EFI/OC/config.plist")
        .context("Failed to open config.plist")?;

    let mut plist: MacPlist = plist::from_reader(&mut conf_plist)?;

    let serial = serial::find_desired(plist.get_product_name())?;
    let uuid = Uuid::new_v4();
    let rom = {
        let mut rom = [0; 12];
        let mut rng = rand::rng();

        let rom_start = APPLE_OUIS
            .choose(&mut rng)
            .context("Couldn't find an Apple OUI")?;
        if rom_start.len() != 6 {
            bail!("Rom start length should be 6 bytes");
        }
        rom[..6].copy_from_slice(rom_start.as_bytes());

        for rom_byte in rom[6..].iter_mut() {
            *rom_byte = *HEX_DIGITS
                .choose(&mut rng)
                .context("Hex digits couldn't be generated")?;
        }

        rom
    };

    if args.dry_run {
        println!();
        println!("Set serial number to {}", serial.serial_number);
        println!("Set MLB to {}", serial.board_serial);
        println!("Set UUID to {}", uuid);
        println!(
            "Set ROM to {:?}",
            std::str::from_utf8(&rom).context("ROM should always be valid UTF-8")?
        );
        return Ok(());
    }

    plist.set_serial_number(serial.serial_number);
    plist.set_mlb(serial.board_serial);
    plist.set_uuid(uuid);
    plist.set_rom(rom);

    plist.debug();
    conf_plist
        .truncate()
        .context("Failed to truncate config.plist")?;
    conf_plist.seek(SeekFrom::Start(0))?;
    plist::to_writer_xml(&mut conf_plist, &plist).context("Failed to write config.plist")?;

    conf_plist.flush()?;
    drop(conf_plist);
    fs.unmount()?;
    first_partition.flush()?;
    qcow2.flush()?;

    Ok(())
}

fn first_partition_subset(mut qcow2: &mut Qcow2) -> Result<IoSubset<&mut Qcow2>> {
    let disk = GptConfig::new().open_from_device(&mut qcow2)?;

    let partitions = disk.partitions();
    let partition = partitions.get(&1).context("Failed to get partition")?;

    let start = partition.bytes_start(LogicalBlockSize::Lb512)?;
    let end = start + partition.bytes_len(LogicalBlockSize::Lb512)?;

    Ok(IoSubset::new(qcow2, start, end))
}

#[derive(Parser)]
struct Args {
    #[clap(long, help = "Path to the bootloader ('OpenCore.qcow2')")]
    bootloader: PathBuf,
    #[clap(short, long, help = "Don't commit changes to disk")]
    dry_run: bool,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity<clap_verbosity_flag::WarnLevel>,
}
