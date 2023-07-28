fn main() {
    println!("cargo:rerun-if-changed=cqt/button.cpp");
    println!("cargo:rerun-if-changed=cqt/button.h");
    println!("cargo:rerun-if-changed=cqt/input.cpp");
    println!("cargo:rerun-if-changed=cqt/input.h");
    let out = std::process::Command::new("qmake")
        .args(["-query", "QT_INSTALL_HEADERS"])
        .output()
        .expect("Failed to run qmake!");
    let p = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let mut incs = vec![p.clone()];
    incs.push(p.clone() + "/QtWidgets");
    incs.push(p.clone() + "/QtCore");
    incs.push(p + "/QtGui");
    cc::Build::new()
        .cpp(true)
        .includes(incs)
        .files(["cqt/button.cpp", "cqt/input.cpp"])
        .flag_if_supported("-w")
        .flag_if_supported("-std=c++17")
        .compile("binder");
}
