use std::{fs, io};

use samefile::file_id;

#[test]
fn test_file_id_with_hard_links() -> Result<(), io::Error> {
    let temp_dir = tempfile::tempdir()?;
    let file_one = temp_dir.path().join("one.txt");
    let file_two = temp_dir.path().join("two.txt");
    fs::write(&file_one, "content")?;
    fs::hard_link(&file_one, &file_two)?;

    let file_id_one = dbg!(file_id(&file_one)?);
    let file_id_two = dbg!(file_id(&file_two)?);
    assert_eq!(file_id_one, file_id_two);

    Ok(())
}
