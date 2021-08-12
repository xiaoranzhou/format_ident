#![feature(test)]


extern crate needletail;
extern crate test;
use needletail::{parse_fastx_file, Sequence, FastxReader};
use std::{str, fmt};




fn main() {
    let filename = "/mnt/e/raw_data/SRR396636.sra_2.fastq";

    let mut n_bases = 0;
    let mut n_valid_kmers = 0;
    let mut reader = parse_fastx_file(&filename).expect("valid path/file");
    while let Some(record) = reader.next() {
        
        let seqrec = record.expect("invalid record");
        // keep track of the total number of bases
        n_bases += seqrec.num_bases();
        // normalize to make sure all the bases are consistently capitalized and
        // that we remove the newlines since this is FASTA
        let norm_seq = seqrec.normalize(false);
        // we make a reverse complemented copy of the sequence first for
        // `canonical_kmers` to draw the complemented sequences from.
        let rc = norm_seq.reverse_complement();
        // now we keep track of the number of AAAAs (or TTTTs via
        // canonicalization) in the file; note we also get the position (i.0;
        // in the event there were `N`-containing kmers that were skipped)
        // and whether the sequence was complemented (i.2) in addition to
        // the canonical kmer (i.1)
//         while seqrec.start_line_number <5 {
//             let row = seqrec.raw_seq();
//             println!( " sads{}", str::from_utf8(row.as_ref()).unwrap() );
            
            
//         }
        for (_, kmer, _) in norm_seq.canonical_kmers(4, &rc) {
            if kmer == b"AAAA" {
                n_valid_kmers += 1;
            }
        }
    }
    println!("There are {} bases in your file.", n_bases);
    println!("There are {} AAAAs in your file.", n_valid_kmers);
}



#[bench]
fn benchmarking (b: &mut test::Bencher) {
    b.iter(|| 
     {(0..1000).fold(0, |x, y| x + y)}
	 //   main();
    );
}