use std::{collections::VecDeque, str::FromStr};

pub fn run(input: &str) -> (String, String) {
    (part1(input).to_string(), part2(input).to_string())
}

pub fn parse(input: &str) -> String {
    input.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum File {
    Index(usize),
    Free,
}

#[derive(Debug, Clone)]
pub struct Block {
    files: Vec<File>,
    free_space: usize,
}

impl Block {
    pub fn free(size: usize) -> Self {
        Self {
            files: vec![File::Free; size],
            free_space: size,
        }
    }

    pub fn files(size: usize, index: usize) -> Self {
        Self {
            files: vec![File::Index(index); size],
            free_space: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.files.len()
    }

    pub fn all_free(&self) -> bool {
        self.free_space == self.size()
    }

    pub fn free_space(&self) -> usize {
        self.free_space
    }

    pub fn merge(&mut self, other: &mut Self) {
        for i in 0..self.size() {
            if self.files[i] == File::Free {
                if let Some(file) = other.files.iter_mut().find(|f| **f != File::Free) {
                    self.files[i] = *file;
                    *file = File::Free;
                    self.free_space -= 1;
                    other.free_space += 1;
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct FileDisk {
    files: VecDeque<File>,
}

impl From<Vec<usize>> for FileDisk {
    fn from(files: Vec<usize>) -> Self {
        let mut disk = Self::default();
        for file in files {
            disk.files.push_back(File::Index(file));
        }
        disk
    }
}

impl From<Vec<File>> for FileDisk {
    fn from(files: Vec<File>) -> Self {
        Self {
            files: VecDeque::from(files),
        }
    }
}

impl FromStr for FileDisk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut disk = Self::default();
        let mut index = 0;
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                let times = c.to_digit(10).unwrap() as usize;
                {
                    let this = &mut disk;
                    if times == 0 {
                        panic!("Invalid input: times is 0");
                    }
                    for _ in 0..times {
                        this.files.push_back(File::Index(index));
                    }
                };
                index += 1;
            } else {
                if c == '0' {
                    continue;
                }
                let space = c.to_digit(10).unwrap() as usize;
                {
                    let this = &mut disk;
                    for _ in 0..space {
                        this.files.push_back(File::Free);
                    }
                };
            }
        }
        Ok(disk)
    }
}

impl FileDisk {
    pub fn sort(&mut self) -> Vec<usize> {
        let mut sorted = Vec::new();
        while let Some(digit) = self.files.pop_front() {
            match digit {
                File::Index(index) => sorted.push(index),
                File::Free => {
                    while let Some(digit) = self.files.pop_back() {
                        match digit {
                            File::Index(index) => {
                                sorted.push(index);
                                break;
                            }
                            File::Free => continue,
                        }
                    }
                }
            }
        }
        sorted
    }

    pub fn checksum(&self) -> usize {
        let mut sum = 0;
        for (i, file) in self.files.iter().enumerate() {
            if let File::Index(index) = file {
                sum += i * index;
            }
        }
        sum
    }
}

pub fn part1(input: &str) -> usize {
    let mut disk: FileDisk = input.parse().unwrap();
    let sorted = disk.sort();
    let disk: FileDisk = sorted.into();
    disk.checksum()
}

struct BlockDisk {
    blocks: Vec<Block>,
}

impl BlockDisk {
    pub fn sort(&mut self) {
        let len = self.blocks.len();
        let blocks = &mut self.blocks[..len];
        let mut ptr_right = len - 1;
        while ptr_right > 0 {
            if blocks[ptr_right].all_free() {
                ptr_right -= 1;
            } else {
                let mut ptr_left = 0;
                while ptr_left < ptr_right {
                    if blocks[ptr_left].free_space() >= blocks[ptr_right].size() {
                        unsafe {
                            let blocks = blocks.as_mut_ptr();
                            let block_left = &mut *blocks.add(ptr_left);
                            let block_right = &mut *blocks.add(ptr_right);
                            block_left.merge(block_right);
                        }
                        break;
                    } else {
                        ptr_left += 1;
                    }
                }
                ptr_right -= 1;
            }
        }
    }
}

impl FromStr for BlockDisk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = Vec::new();
        let mut index = 0;
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                let times = c.to_digit(10).unwrap() as usize;
                blocks.push(Block::files(times, index));
                index += 1;
            } else {
                let space = c.to_digit(10).unwrap() as usize;
                blocks.push(Block::free(space));
            }
        }
        Ok(Self { blocks })
    }
}

impl From<BlockDisk> for FileDisk {
    fn from(block_disk: BlockDisk) -> Self {
        FileDisk {
            files: block_disk
                .blocks
                .into_iter()
                .flat_map(|b| b.files)
                .collect(),
        }
    }
}

pub fn part2(input: &str) -> usize {
    let mut block_disk: BlockDisk = input.parse().unwrap();
    block_disk.sort();
    let file_disk: FileDisk = block_disk.into();
    file_disk.checksum()
}
