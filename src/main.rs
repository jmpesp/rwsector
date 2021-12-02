//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use anyhow::Result;
use rand::RngCore;
use sha3::{Digest, Sha3_256};
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

        #[structopt(short, long, default_value = "512")]
        bsz: usize,

        #[structopt(short, long, default_value = "16")]
        width: usize,
    },
    Write {
        #[structopt(short, long, parse(from_os_str))]
        path: PathBuf,

        #[structopt(short, long)]
        offset: usize,

        #[structopt(short, long)]
        count: usize,

        #[structopt(short, long)]
        value: u8,

        #[structopt(short, long, default_value = "512")]
        bsz: usize,

        #[structopt(short, long, default_value = "16")]
        width: usize,
    },
    WriteRandom {
        #[structopt(short, long, parse(from_os_str))]
        path: PathBuf,

        #[structopt(short, long)]
        offset: usize,

        #[structopt(short, long)]
        count: usize,

        #[structopt(short, long, default_value = "512")]
        bsz: usize,

        #[structopt(short, long, default_value = "16")]
        width: usize,
    },
    Digest {
        #[structopt(short, long, parse(from_os_str))]
        path: PathBuf,

        #[structopt(short, long)]
        offset: usize,

        #[structopt(short, long)]
        count: usize,

        #[structopt(short, long, default_value = "512")]
        bsz: usize,
    },
}

fn hexdump(data: &[u8], starting_offset: usize, count: usize, bsz: usize, width: usize) {
    for block in 0..count {
        for row in 0..(bsz / width) {
            let offset = block * bsz + row * width;
            print!("{:0>8x}: ", starting_offset * bsz + offset);

            for col in 0..width {
                let i = offset + col;
                print!("{:02x}", data[i]);

                if (col + 1) % 2 == 0 {
                    print!(" ");
                }
            }
            println!();
        }

        if (block + 1) != count {
            println!();
        }
    }
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    match args {
        Args::Read {
            path,
            offset,
            count,
            bsz,
            width,
        } => {
            let mut file = File::open(path)?;
            let mut data = vec![0u8; count * bsz];

            file.seek(SeekFrom::Start((offset * bsz) as u64))?;
            file.read_exact(&mut data[..])?;

            println!("read from block {}:", offset);
            hexdump(&data, offset, count, bsz, width);
        }
        Args::Write {
            path,
            offset,
            count,
            value,
            bsz,
            width,
        } => {
            let mut file = OpenOptions::new().write(true).open(path)?;
            let data = vec![value; count * bsz];

            println!("write to block {}:", offset);
            hexdump(&data, offset, count, bsz, width);

            file.seek(SeekFrom::Start((offset * bsz) as u64))?;
            file.write_all(&data[..])?;
            file.flush()?;
        }
        Args::WriteRandom {
            path,
            offset,
            count,
            bsz,
            width,
        } => {
            let mut file = OpenOptions::new().write(true).open(path)?;

            let mut rng = rand::thread_rng();
            let mut data = vec![0u8; count * bsz];
            rng.fill_bytes(&mut data[..]);

            println!("write to block {}:", offset);
            hexdump(&data, offset, count, bsz, width);

            file.seek(SeekFrom::Start((offset * bsz) as u64))?;
            file.write_all(&data[..])?;
            file.flush()?;
        }
        Args::Digest {
            path,
            offset,
            count,
            bsz,
        } => {
            let mut file = File::open(path)?;
            let mut data = vec![0u8; count * bsz];

            file.seek(SeekFrom::Start((offset * bsz) as u64))?;
            file.read_exact(&mut data[..])?;

            let mut hasher = Sha3_256::new();
            hasher.update(data);
            let result = hasher.finalize();

            println!("{:x}", result);
        }
    }

    Ok(())
}
