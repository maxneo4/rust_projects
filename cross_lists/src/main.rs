use std::fs::File;
use std::collections::HashSet;
use std::io::Write;
//use std::env;

fn main() {

    let filename = "C:/MegaSync/_intercambio archivos/guidsDataCompare.csv";
    let out_file_name = "C:/git/rust_projects/cross_lists/result.txt";
    let file = File::open(filename).unwrap();
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(file);

    let mut hsa = HashSet::new();
    let mut hsb = HashSet::new();

    for result in rdr.records(){
        let record  = result.unwrap();
        let ra = record.get(0).map(String::from).unwrap();
        let rb= record.get(1).map(String::from).unwrap();
        hsa.insert(ra);
        hsb.insert(rb);
    }
    hsa.remove("");
    hsb.remove("");

    let symetric_difference = hsa.symmetric_difference(&hsb).collect::<Vec<&String>>();
    let mut file = File::create(out_file_name).unwrap();

    for s in symetric_difference.into_iter().enumerate(){
        let mut sr = String::from(s.1);
        sr.push('\n');
        let _ = file.write_all(sr.as_bytes());
    }
}