// // use std::path::{PathBuf, Path};

// // use walkdir::WalkDir;

// // use anyhow::Result;

// // fn get_repository_root_path() -> Result<PathBuf> {
// //     let project_directory = std::env::var("CARGO_MANIFEST_DIR")?;
// //     let path = PathBuf::from(project_directory);
// //     match path.parent() {
// //         Some(path) => Ok(path.to_path_buf()),
// //         None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "The parent of the project directory was not found").into())
// //     }
// // }

// // fn collect_source_files_by_extensions(extensions: &[&str]) -> Vec<PathBuf> {
// //     let exlaunch_root = match get_repository_root_path().map(|repo_root| repo_root.join("exlaunch")) {
// //         Ok(root) => root,
// //         Err(e) => panic!("Failed to get the root path of exlaunch: {:?}", e)
// //     };

// //     let src_dir = exlaunch_root.join("source");
// //     let mut files = vec![];
// //     for entry in WalkDir::new(src_dir) {
// //         let entry = match entry {
// //             Ok(entry) => entry,
// //             Err(e) => {
// //                 eprintln!("Entry was invalid: {:?}", e);
// //                 continue;
// //             }
// //         };

// //         if !entry.file_type().is_file() {
// //             continue;
// //         }

// //         let extension = match entry.path().extension().map(|ext| ext.to_str()).flatten() {
// //             Some(ext) => ext,
// //             None => {
// //                 eprintln!("Entry '{}' does not have an extension, skipping", entry.path().display());
// //                 continue;
// //             }
// //         };

// //         if extensions.contains(&extension) {
// //             files.push(entry.path().to_path_buf());
// //         }
// //     }

// //     files
// // }

// // fn collect_source_files() -> Vec<PathBuf> {
// //     collect_source_files_by_extensions(&["c", "cc", "cpp", "s"])
// // }

// // fn collect_header_files() -> Vec<PathBuf> {
// //     collect_source_files_by_extensions(&["h", "hpp"])
// // }

// // fn collect_include_directories() -> Vec<PathBuf> {
// //     let exlaunch_root = match get_repository_root_path().map(|repo_root| repo_root.join("exlaunch/source")) {
// //         Ok(root) => root,
// //         Err(e) => panic!("Failed to get the root path of exlaunch: {:?}", e)
// //     };

// //     WalkDir::new(exlaunch_root)
// //         .min_depth(1)
// //         .max_depth(1)
// //         .into_iter()
// //         .filter_map(|x| {
// //             match x {
// //                 Ok(entry) if entry.file_type().is_dir() => Some(entry.path().to_path_buf()),
// //                 _ => None
// //             }
// //         })
// //         .collect()
// // }

// fn get_libgcc_folder() -> PathBuf {
//     if !Path::new("/opt/devkitpro/devkitA64/lib/gcc/aarch64-none-elf").exists() {
//         panic!("Could not find installation for devkitA64");
//     }

//     for entry in WalkDir::new("/opt/devkitpro/devkitA64/lib/gcc/aarch64-none-elf") {
//         let entry = match entry {
//             Ok(entry) => entry,
//             Err(e) => {
//                 eprintln!("Entry was invalid: {:?}", e);
//                 continue;
//             }
//         };

//         if !entry.file_type().is_dir() {
//             continue;
//         }

//         let path = entry.path().join("pic");
//         if path.exists() {
//             return path;
//         }
//     }

//     panic!("No libgcc folder found in devkitA64 installation");
// }

fn main() {
    ex_build::copy_link_script();
    ex_build::compile();
    // panic!();

    // let sources = collect_source_files();
    // let headers = collect_header_files();

    // let src_dir = match get_repository_root_path().map(|root| root.join("exlaunch/source")) {
    //     Ok(dir) => dir,
    //     Err(e) => panic!("Failed to get the source directory of exlaunch")
    // };
    
    // println!("cargo:rerun-if-changed=build.rs");
    // for file in &sources {
    //     println!("cargo:rerun-if-changed={}", file.display());
    // }

    // for file in headers {
    //     println!("cargo:rerun-if-changed={}", file.display());
    // }

    // println!("cargo:rustc-link-search={}", get_libgcc_folder().display());
    // println!("cargo:rustc-link-search=/opt/devkitpro/devkitA64/aarch64-none-elf/lib/pic");

    // println!("cargo:rustc-link-lib=static=stdc++");
    // println!("cargo:rustc-link-lib=static=gcc");

    // println!("cargo:rustc-cdylib-link-args=--shared --export-dynamic -nodefaultlibs");

    // cc::Build::new()
    //     .compiler("/opt/devkitpro/devkitA64/bin/aarch64-none-elf-g++")
    //     .cpp(true)
    //     .shared_flag(true)
    //     .static_flag(true)
    //     .no_default_flags(true)
    //     .warnings(false)

    //     .define("__SWITCH__", None)
    //     .define("__RTLD_6XX__", None)
    //     .define("EXL_LOAD_KIND", "Module")
    //     .define("EXL_LOAD_KIND_ENUM", "2")
    //     .define("EXL_PROGRAM_ID", "0x01006a800016e000")

    //     .flag("-g")
    //     .flag("-fPIC")
    //     .flag("-Wall")
    //     .flag("-Werror")
    //     .flag("-O3")
    //     .flag("-ffunction-sections")
    //     .flag("-fdata-sections")
    //     .flag("-fno-rtti")
    //     .flag("-fno-exceptions")
    //     .flag("-fno-asynchronous-unwind-tables")
    //     .flag("-fno-unwind-tables")
    //     .flag("-std=gnu++20")

    //     .files(sources)

    //     .include(src_dir)
    //     .includes(collect_include_directories())
    //     .include("/opt/devkitpro/libnx/include")
    //     .compile("skyline");
}