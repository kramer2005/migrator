pub fn help() {
    println!("Usage: <filename> [--base-path <path>] [pg|quest] [add|deploy|revert|status]");
    println!("Options:");
    println!(
        "   --base-path <path>                     Base path for the migrations and seeds folders"
    );
    println!("  --init             Initialize the project");
    println!("  --help             Display this help message");
}