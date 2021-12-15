fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut days: Vec<String> = std::fs::read_dir(".")?
		.filter_map(Result::ok)
		.map(|dir| dir.file_name().to_string_lossy().into_owned())
		.filter(|filename| filename.contains("day"))
		.collect();
	days.sort_by_key(|day| day.strip_prefix("day_").unwrap().parse::<u64>().unwrap());

	for filename in days {
		println!("{}", filename);
		std::process::Command::new("cargo")
			.arg("run")
			.arg("--release")
			.arg("--quiet")
			.arg("-p")
			.arg(filename)
			.spawn()?
			.wait()?;
		println!();
	}

	Ok(())
}
