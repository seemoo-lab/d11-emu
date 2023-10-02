use std::{process::Command, fs};

fn main() {
	fs::read_dir("tests").unwrap().into_iter()

	.filter(|file| {
		file.as_ref().unwrap().path().to_str().unwrap().ends_with(".asm")
	})
	.for_each(| file| {
		println!("{:?}", file);
		let f = file.unwrap().path();
		let file_str = f.to_str().unwrap().strip_suffix(".asm").unwrap();

		let src = format!("{}{}", file_str, ".asm");
		let dst = format!("{}{}", file_str, ".bin");


		let output = Command::new("b43-asm").args(&[src.as_str(), dst.as_str(), "-f raw-le32"]).output().expect("asm file missing");
		println!("{:?}", output);
	});
}