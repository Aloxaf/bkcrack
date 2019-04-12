use crate::attack::Attack;
use crate::crc32_tab::CRC32TAB;
use crate::keystream_tab::KEYSTREAMTAB;
use crate::progress;
use rayon::prelude::*;
use std::mem;

pub struct Zreduction<'a> {
    keystream: &'a [u8],
    zi_2_32_vector: Vec<u32>,
    index: usize,
}

impl<'a> Zreduction<'a> {
    // TODO: 自定义 WAIT_SIZE
    const WAIT_SIZE: usize = 1 << 8;
    const TRACK_SIZE: usize = 1 << 16;

    pub fn new(keystream: &[u8]) -> Zreduction {
        Zreduction {
            zi_2_32_vector: Vec::new(),
            keystream,
            index: 0,
        }
    }

    pub fn generate(&mut self) {
        self.index = self.keystream.len();
        self.zi_2_32_vector.reserve(1 << 22);

        for &zi_2_16 in KEYSTREAMTAB
            .get_zi_2_16_array(*self.keystream.last().unwrap())
            .iter()
        {
            for high in 0..(1 << 16) {
                self.zi_2_32_vector.push(high << 16 | zi_2_16);
            }
        }
    }

    pub fn reduce(&mut self) {
        // variables to keep track of the smallest Zi[2,32) vector
        let mut tracking = false;
        let mut best_copy = Vec::with_capacity(1 << 16);
        let (mut best_index, mut best_size) = (0usize, Zreduction::TRACK_SIZE);

        // variables to wait for a limited number of steps when a small enough vector is found
        let mut waiting = false;
        let mut wait = 0usize;

        let mut zim1_10_32_vector = Vec::with_capacity(1 << 16);
        let mut zim1_2_32_vector = Vec::with_capacity(1 << 16);

        for i in (Attack::SIZE..self.index).rev() {
            zim1_10_32_vector.clear();
            zim1_2_32_vector.clear();

            // generate the Z{i-1}[10,32) values
            for &zi_2_32 in &self.zi_2_32_vector {
                // get Z{i-1}[10,32) from CRC32^-1
                let zim1_10_32 = CRC32TAB.get_zim1_10_32(zi_2_32);
                // collect only those that are compatible with keystream{i-1}
                if KEYSTREAMTAB.has_zi_2_16(self.keystream[i - 1], zim1_10_32) {
                    zim1_10_32_vector.push(zim1_10_32);
                }
            }

            // remove duplicates
            if zim1_10_32_vector.len() >= (1 << 12) {
                // 如果 Vec 本身不大, 并行排序可能反而降低效率
                zim1_10_32_vector.par_sort_unstable();
            } else {
                zim1_10_32_vector.sort_unstable();
            }

            zim1_10_32_vector.dedup();

            // complete Z{i-1}[10,32) values up to Z{i-1}[2,32)
            for &zim1_10_32 in &zim1_10_32_vector {
                // get Z{i-1}[2,16) values from keystream byte k{i-1} and Z{i-1}[10,16)
                for &zim1_2_16 in KEYSTREAMTAB.get_zi_2_16_vector(self.keystream[i - 1], zim1_10_32)
                {
                    //println!("({} {})", zi_2_32, zim1_10_32);
                    zim1_2_32_vector.push(zim1_10_32 | zim1_2_16);
                }
            }
            //std::process::exit(1);

            // update smallest vector tracking
            if zim1_2_32_vector.len() <= best_size {
                tracking = true;
                best_index = i - 1;
                best_size = zim1_2_32_vector.len();
                waiting = false;
            } else if tracking {
                // vector is bigger than bestSize
                if best_index == i {
                    // hit a minimum
                    // keep a copy of the vector because size is about to grow
                    std::mem::swap(&mut best_copy, &mut self.zi_2_32_vector);

                    if best_size <= Zreduction::WAIT_SIZE {
                        // enable waiting
                        waiting = true;
                        wait = best_size * 4;
                    }
                }

                wait -= 1;
                if waiting && wait == 0 {
                    break;
                }
            }

            // put result in z_2_32_vector
            mem::swap(&mut self.zi_2_32_vector, &mut zim1_2_32_vector);
            // self.zi_2_32_vector = zim1_2_32_vector;
            let now = self.keystream.len() - i;
            let total = self.keystream.len() - Attack::SIZE;
            progress(now, total);
        }

        if tracking {
            // put bestCopy in z_2_32_vector only if bestIndex is not the index of z_2_32_vector
            if best_index != Attack::SIZE - 1 {
                mem::swap(&mut self.zi_2_32_vector, &mut best_copy);
                //self.zi_2_32_vector = best_copy;
            }
            self.index = best_index;
        } else {
            self.index = Attack::SIZE - 1;
        }
    }

    pub fn size(&self) -> usize {
        self.zi_2_32_vector.len()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_zi_2_32_vector(&self) -> &Vec<u32> {
        &self.zi_2_32_vector
    }
}
