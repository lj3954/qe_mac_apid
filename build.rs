fn main() {
    cc::Build::new()
        .file("OpenCorePkg/Utilities/macserial/macserial.c")
        .file("OpenCorePkg/User/Library/UserPseudoRandom.c")
        .file("wrapper.c")
        .include("OpenCorePkg/User/Include")
        .flag("-Wno-unused-variable")
        .define("main", "ignored_main")
        .compile("macserial");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-IOpenCorePkg/User/Include/")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/modelinfo.rs")
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.c");
    println!("cargo:rerun-if-changed=OpenCorePkg/Utilities/macserial/modelinfo_autogen.h");
}
