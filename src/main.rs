use archive_analyzer::zip::reader;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: archive-analyzer <zipFilename>");
    }

    let zip_filename = &args[1];    
    let mut file = fs::File::open(zip_filename).unwrap();
    let zip_file_result = reader::ZipFileReader::read(&mut file);

    println!("\n\n\n\n\n\n\n");

    if let Ok(zip_file) = zip_file_result {
        for stored_file in zip_file.stored_files {
            println!("----------{}----------", stored_file.local_file_header.filename);
            println!("\tMinimum version to extract: {}", stored_file.local_file_header.minimum_version);
            println!("\tGeneral purpose flag: {}", stored_file.local_file_header.general_purpose_flag);
            println!("\tCompression method: {}", stored_file.local_file_header.compression_method);
            println!("\tFile last modification time: {}", stored_file.local_file_header.file_last_modification_time);
            println!("\tFile last modification date: {}", stored_file.local_file_header.file_last_modification_date);
            println!("\tCRC32: {}", stored_file.local_file_header.crc32);
            println!("\tCompressed size: {}", stored_file.local_file_header.compressed_size);
            println!("\tUncompressed size: {}", stored_file.local_file_header.uncompressed_size);
            println!("\tFilename: {}", stored_file.local_file_header.filename);
            println!("\tFound in archive: {}", stored_file.found_in_archive);
            println!("\tFound in central directory: {}", stored_file.found_in_central_directory);
            if stored_file.found_in_archive {
                println!("\tPosition in archive: {}", stored_file.position_in_archive.unwrap());
                println!("\tOffset from start of archive: {}", stored_file.offset_in_archive.or(Some(0)).unwrap());
            }
            if stored_file.found_in_central_directory {
                println!("\tPosition in central directory: {}", stored_file.position_in_central_directory.unwrap());
                println!("\tOffset from central directory: {}", stored_file.offset_from_central_directory.unwrap())
            }
            
            println!("\n")
        }

    }
    println!("\n\n\n\n\n\n\n");

}



