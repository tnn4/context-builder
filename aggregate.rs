use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

fn print_help() {
    println!(r#"
    File Compiler Utility
    Recursively finds files and appends them to a single text file.

    USAGE:
    executable [FLAGS]

    FLAGS:
    -h, --help               Prints help information
    -o, --output <FILE>      Sets the name of the output file (Default: compiled_files.txt)
    -e, --extension <EXT>    Add an extension to search for (can be used multiple times)
    -d, --directory <PATH>   The directory to start the search in (Default: .)

    EXAMPLES:
    ./file_compiler -d ./src -e rs -o source_dump.txt
    ./file_compiler --directory /home/user/logs --extension log
    "#);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
        return Ok(());
    }

    let mut target_extensions = Vec::new();
    let mut output_file = String::from("compiled_files.txt");
    let mut search_dir = String::from(".");

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output_file = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: -o requires a filename.");
                    std::process::exit(1);
                }
            }
            "-e" | "--extension" => {
                if i + 1 < args.len() {
                    let ext = args[i + 1].trim_start_matches('.').to_string();
                    target_extensions.push(ext);
                    i += 2;
                } else {
                    eprintln!("Error: -e requires an extension name.");
                    std::process::exit(1);
                }
            }
            "-d" | "--directory" => {
                if i + 1 < args.len() {
                    search_dir = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: -d requires a directory path.");
                    std::process::exit(1);
                }
            }
            _ => i += 1,
        }
    }

    if target_extensions.is_empty() {
        target_extensions.push(String::from("txt"));
    }

    let start_path = Path::new(&search_dir);
    if !start_path.exists() || !start_path.is_dir() {
        eprintln!("Error: The directory '{}' does not exist or is not a directory.", search_dir);
        std::process::exit(1);
    }

    println!("Searching in: {}", search_dir);
    println!("Target extensions: {:?}", target_extensions);
    println!("Output target: {}", output_file);

    fs::File::create(&output_file)?;
    visit_dirs(start_path, &target_extensions, &output_file)?;

    println!("Process complete.");
    Ok(())
}

fn visit_dirs(dir: &Path, extensions: &[String], output_path: &str) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, extensions, output_path)?;
            } else if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if extensions.contains(&ext.to_string()) {
                    append_to_output(&path, output_path)?;
                }
            }
        }
    }
    Ok(())
}

fn append_to_output(source_path: &Path, output_path: &str) -> io::Result<()> {
    // Prevent self-consumption
    if let (Ok(s), Ok(o)) = (fs::canonicalize(source_path), fs::canonicalize(output_path)) {
        if s == o { return Ok(()); }
    }

    let mut source_file = fs::File::open(source_path)?;
    let mut content = String::new();

    if source_file.read_to_string(&mut content).is_ok() {
        let mut output_file = OpenOptions::new().append(true).open(output_path)?;

        // Extract extension and determine language name
        let extension = source_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        
        let language = get_language_name(extension);

        // Updated XML header with language attribute
        writeln!(
            output_file, 
            r#"<file path="{}" language="{}">"#, 
            source_path.display(), 
            language
        )?;
        writeln!(output_file, "{}", content)?;
        writeln!(output_file, "</file>\n")?;

        println!("Appended: {} (Language: {})", source_path.display(), language);
    }

    Ok(())
}

fn get_language_name(ext: &str) -> &str {
    match ext.to_lowercase().as_str() {
        // Logic & Programming Languages
        "rs" => "rust",
        "gd" => "godotscript",
        "lua" => "lua",
        "cs" => "csharp",
        "py" => "python",
        "js" => "javascript",
        "ts" => "typescript",
        "cpp" | "cc" | "cxx" | "hpp" | "h" => "cpp",
        "c" => "c",
        "go" => "go",
        "rb" => "ruby",
        "fs" | "fsi" | "fsx" => "fsharp",
        "java" => "java",
        "kt" => "kotlin",
        "swift" => "swift",

        // Game Engine Specifics & Asset Metadata
        "tscn" | "tres" | "godot" => "godot-data",
        "gdshader" => "godot-shader",
        "unity" | "prefab" | "meta" | "mat" => "unity-data",
        "uproject" | "uasset" | "umap" => "unreal-data",

        // Graphics & Shaders
        "glsl" | "vert" | "frag" | "comp" | "geom" => "glsl",
        "hlsl" | "fx" | "hlsli" => "hlsl",
        "wgsl" => "wgsl",

        // Build Systems & Infrastructure
        "makefile" | "make" | "mk" => "makefile",
        "dockerfile" | "dockerignore" => "dockerfile",
        "env" => "dotenv",
        "lock" => "lockfile",
        "cmake" => "cmake",

        // Shell & Automation
        "sh" | "bash" => "shell",
        "ps1" | "psm1" | "psd1" => "powershell",
        "bat" | "cmd" => "batch",

        // Data & Configuration
        "toml" => "toml",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "xml" | "csproj" | "fsproj" => "xml",
        "ini" | "cfg" | "prefs" => "ini",
        "csv" => "csv",

        // Documentation & Web
        "md" | "markdown" => "markdown",
        "txt" => "text",
        "html" | "htm" => "html",
        "css" => "css",
        "sql" => "sql",

        _ => "text", 
    }
}