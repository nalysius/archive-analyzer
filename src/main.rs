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
    println!("Files stored in archive");

    if let Ok(zip_file) = zip_file_result {
        for stored_file in zip_file.stored_files {
            println!("----------{}----------", stored_file.local_file_header.filename);
            println!("\tPosition in archive: {}", stored_file.position);
            println!("\tMinimum version to extract: {}", stored_file.local_file_header.minimum_version);
            println!("\tGeneral purpose flag: {}", stored_file.local_file_header.general_purpose_flag);
            println!("\tCompression method: {}", stored_file.local_file_header.compression_method);
            println!("\tFile last modification time: {}", stored_file.local_file_header.file_last_modification_time);
            println!("\tFile last modification date: {}", stored_file.local_file_header.file_last_modification_date);
            println!("\tCRC32: {}", stored_file.local_file_header.crc32);
            println!("\tCompressed size: {}", stored_file.local_file_header.compressed_size);
            println!("\tUncompressed size: {}", stored_file.local_file_header.uncompressed_size);
            println!("\tFilename: {}", stored_file.local_file_header.filename);
            println!("\tFound in central directory: {}", stored_file.found_in_central_directory);
            println!("\tOffset from start of archive: {}", stored_file.offset_in_archive);

            println!("\n")
        }

        println!("Central directory");
        if zip_file.central_directory.is_none() {
            println!("\t No central directory found");
        } else {
            let central_directory = zip_file.central_directory.unwrap();

            println!("\tHas a digital signature: {}", central_directory.digital_signature.is_some());
            println!("\tNumber of central directory records on this disk: {}", central_directory.end_of_central_directory_record.central_directory_records_number_on_disk);
            println!("\tTotal number of central directory records: {}", central_directory.end_of_central_directory_record.central_directory_records_total_number);
            println!("\tSize of central directory: {}", central_directory.end_of_central_directory_record.central_directory_size);
            println!("\tNumber of disks: {}", central_directory.end_of_central_directory_record.disk_number);
            println!("\tDisk on which starts the central directory: {}", central_directory.end_of_central_directory_record.disk_start_central_directory);
            println!("\tOffset of the central directory, relative to the start of archive: {}", central_directory.end_of_central_directory_record.offset_start_central_directory);

            for central_directory_file_headers in central_directory.file_headers {
                println!("\n");
                println!("----------{}----------", central_directory_file_headers.filename);
                if central_directory_file_headers.position.is_some() {
                    println!("\tPosition in central directory: {}", central_directory_file_headers.position.unwrap());
                }
                println!("\tCompressed size: {}", central_directory_file_headers.compressed_size);
                println!("\tCompression method: {}", central_directory_file_headers.compression_method);
                println!("\tCRC32: {}", central_directory_file_headers.crc32);
                println!("\tDisk where the archive starts: {}", central_directory_file_headers.disk_start);
                println!("\tExternal file attributes: {}", central_directory_file_headers.external_file_attributes);
                println!("\tInternal file attributes: {}", central_directory_file_headers.internal_file_attributes);
                println!("\tFile last modification time: {}", central_directory_file_headers.file_last_modification_time);
                println!("\tFile last modification date: {}", central_directory_file_headers.file_last_modification_date);
                println!("\tGeneral purpose flag: {}", central_directory_file_headers.general_purpose_flag);
                //println!("\tExtra field: {}", central_directory_file_headers.extra_field);
                println!("\tFile comment: {}", central_directory_file_headers.file_comment);
                println!("\tFilename: {}", central_directory_file_headers.filename);
            }
        }


    }
    println!("\n\n\n\n\n\n\n");

}



