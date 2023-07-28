fn main() {
    println!("cargo:rerun-if-changed=src/button.cpp");
    println!("cargo:rerun-if-changed=src/button.h");
    println!("cargo:rerun-if-changed=src/input.cpp");
    println!("cargo:rerun-if-changed=src/input.h");
    let out = std::process::Command::new("qmake")
        .args(["-query", "QT_INSTALL_HEADERS"])
        .output()
        .expect("Failed to run qmake!");
    let p = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let mut incs = vec![];
    incs.push(p.clone());
    incs.push(p.clone() + "/QtWidgets");
    incs.push(p.clone() + "/QtCore");
    incs.push(p.clone() + "/QtGui");
    cc::Build::new()
        .cpp(true)
        .includes(incs)
        .files(["src/button.cpp", "src/input.cpp"])
        .flag_if_supported("-w")
        .flag_if_supported("-std=c++17")
        .compile("binder");
}
