use std::env;

fn main() {
    let vitasdk = env::var_os("VITASDK").map(|s| s.into_string().unwrap()).unwrap_or("/usr/local/vitasdk".to_string());
    println!("cargo:rustc-link-lib=static=SceSysmodule_stub");
    println!("cargo:rustc-link-lib=static=SceNet_stub");
    println!("cargo:rustc-link-lib=static=SceNetCtl_stub");
    println!("cargo:rustc-link-lib=static=SceKernel_stub");
    println!("cargo:rustc-link-lib=static=ScePower_stub");
    println!("cargo:rustc-link-lib=static=SceAppMgr_stub");
    println!("cargo:rustc-link-lib=static=psp2shell");
    println!("cargo:rustc-link-search={}/arm-vita-eabi/lib", vitasdk);
}