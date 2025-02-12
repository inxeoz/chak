use std::fs::File;
use std::{env, io};
use crate::config::get_project_dir;
use crate::diff::{hashed_content_from_string_lines, HashedContent};
use crate::diff_algo::{ hashed_content_from_file};
use crate::util::{ deserialize_file_content, serialize_struct,save_or_create_file};

pub fn restore_previous_version(
    fixed_next_content: &HashedContent,
    diff_content: &HashedContent,
) -> Result<Vec<String>, io::Error> {
    let mut previous_lines = Vec::new();

    for line_hash in &diff_content.hash_lines {
        if let Some(content) = fixed_next_content
            .hash_to_content
            .get(line_hash)
            .or_else(|| diff_content.hash_to_content.get(line_hash))
        {
            previous_lines.push(content.clone());
        }
    }

    Ok(previous_lines)
}

fn restore_previous_versiono() -> io::Result<()> {
    let file3 = File::open(env::current_dir()?.join("file3.txt"))?;
    let file3_content = hashed_content_from_file(&file3);

    // Generate mappings
    let diff2 = deserialize_file_content::<HashedContent>(
        &get_project_dir().join("restore").join("diff2.json"),
    )
        .ok()
        .expect("restore failed");

    let diff1 = deserialize_file_content::<HashedContent>(
        &get_project_dir().join("restore").join("diff1.json"),
    )
        .ok()
        .expect("restore failed");

    if let Ok(file2_content_vec) = restore_previous_version(&file3_content, &diff2) {
        let file2_content = hashed_content_from_string_lines(file2_content_vec.clone());
        // println!("diff content\n{}", serde_json::to_string_pretty(&)?);

        let serialzed = serialize_struct(&file2_content);
        save_or_create_file(
            &get_project_dir().join("restore").join("restored2.json"),
            Some(&serialzed),
            false,
            None
        )?;

        if let Ok(file1_content_vec) = restore_previous_version(&file2_content, &diff1) {
            let file1_content = hashed_content_from_string_lines(file1_content_vec.clone());
            // println!("diff content\n{}", serde_json::to_string_pretty(&)?);

            let serialzed = serialize_struct(&file1_content);
            save_or_create_file(
                &get_project_dir().join("restore").join("restored1.json"),
                Some(&serialzed),
                false,
                None
            )?;
        }
    }

    Ok(())
}