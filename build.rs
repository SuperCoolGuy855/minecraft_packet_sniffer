use std::env;
use std::path::Path;
use std::fs;

fn main() {
    // Specify link information
    // println!("cargo:rustc-link-search=native=path/to/your/dlls");
    // println!("cargo:rustc-link-lib=dylib=your_library_name");

    // Get the output directory from Cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(4).unwrap();
    let target_path = target_dir.join(profile);

    // Copy the DLL to the output directory
    let wpcap_dll_src_path = "C:/Windows/System32/Npcap/wpcap.dll";
    let dll_dst_path = target_path.join("wpcap.dll");
    fs::copy(wpcap_dll_src_path, dll_dst_path).expect("Failed to copy DLL");

    // Copy the DLL to the output directory
    let packet_dll_src_path = "C:/Windows/System32/Npcap/Packet.dll";
    let dll_dst_path = target_path.join("Packet.dll");
    fs::copy(packet_dll_src_path, dll_dst_path).expect("Failed to copy DLL");

    // Ensure the build script runs again if the DLL changes
    println!("cargo:rerun-if-changed={}", wpcap_dll_src_path);
    println!("cargo:rerun-if-changed={}", packet_dll_src_path);
}
