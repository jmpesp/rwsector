//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

extern crate libc;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use anyhow::Result;
use rand::RngCore;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "interactive dd")]
enum Args {
    Read {
        #[structopt(short, long, parse(from_os_str))]
        path: PathBuf,

        #[structopt(short, long)]
        offset: usize,

        #[structopt(short, long)]
        count: usize,
    },
    Write {
        #[structopt(short, long, parse(from_os_str))]
        path: PathBuf,

        #[structopt(short, long)]
        offset: usize,

        #[structopt(short, long)]
        count: usize,

        #[structopt(short, long)]
        byte: u8,
    },
    WriteRandom {
        #[structopt(short, long, parse(from_os_str))]
        path: PathBuf,

        #[structopt(short, long)]
        offset: usize,

        #[structopt(short, long)]
        count: usize,
    },
}

fn hexdump(data: &[u8], starting_offset: usize, count: usize, bsz: usize) {
    for block in 0..count {
        for row in 0..32 {
            let offset = block * bsz + row * 16;
            print!("{:0>8x}: ", starting_offset * bsz + offset);

            for col in 0..16 {
                let i = offset + col;
                print!("{:02x}", data[i]);

                if (col + 1) % 2 == 0 {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    let bsz = 512;
    match args {
        Args::Read {
            path,
            offset,
            count,
        } => {
            let mut file = File::open(path)?;
            let mut data = vec![0u8; count * bsz];

            file.seek(SeekFrom::Start((offset * bsz) as u64))?;
            file.read_exact(&mut data[..])?;

            println!("read from block {}:", offset);
            hexdump(&data, offset, count, bsz);
        }
        Args::Write {
            path,
            offset,
            count,
            byte,
        } => {
            let mut file = OpenOptions::new()
                .write(true)
                .custom_flags(libc::O_DIRECT)
                .open(path)?;
            let data = vec![byte; count * bsz];

            println!("write to block {}:", offset);
            hexdump(&data, offset, count, bsz);

            file.seek(SeekFrom::Start((offset * bsz) as u64))?;
            file.write_all(&data[..])?;
            file.flush()?;
        }
        Args::WriteRandom {
            path,
            offset,
            count,
        } => {
            let mut file = OpenOptions::new()
                .write(true)
                .custom_flags(libc::O_DIRECT)
                .open(path)?;

            let mut rng = rand::thread_rng();
            let mut data = vec![0u8; count * bsz];
            rng.fill_bytes(&mut data[..]);

            println!("write to block {}:", offset);
            hexdump(&data, offset, count, bsz);

            file.seek(SeekFrom::Start((offset * bsz) as u64))?;
            file.write_all(&data[..])?;
            file.flush()?;
        }
    }

    Ok(())
}
