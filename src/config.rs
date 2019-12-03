// Copyright (c) 2018 10x Genomics, Inc. All rights reserved.

use debruijn::kmer;

// transcriptome fasta header formats
pub enum FastaFormat {
    Unknown,
    Gencode,
    Ensembl,
    Gffread,
}

// main configs
pub const MEM_SIZE: usize = 1;
pub const MIN_KMERS: usize = 1;
pub const STRANDED: bool = false;
pub const REPORT_ALL_KMER: bool = false;
pub const READ_COVERAGE_THRESHOLD: usize = 32;
pub const LEFT_EXTEND_FRACTION: f64 = 0.2;

pub const U32_MAX: usize = u32::max_value() as usize;

// Using Kmer24 for cockatoo so tests pass, but no optimisation has been done
// (computational or scientific).
pub type KmerType = kmer::Kmer24;

// Transcriptome mappability
pub const MAPPABILITY_COUNTS_LEN: usize = 11;
