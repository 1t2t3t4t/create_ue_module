mod path_utils;

use std::path::Path;

use crate::path_utils::FsHandle;

static MODULE_NAME: &str = "{MODULE_NAME}";

macro_rules! apply_name_to_template {
    ($name:tt, $module_name:tt) => {{
        let template = include_str!($name);
        template.replace(MODULE_NAME, $module_name)
    }};
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let module_name = args.get(1).expect("Missing module name");
    let module_path = Path::new(module_name);

    make_necessary_folders(module_path);

    write_build_cs(module_name, module_path);
    write_module_interface(module_name, module_path);
    write_uprojcet(module_name, module_path);
}

fn make_necessary_folders(module_path: &Path) {
    module_path.create_dir_if_not_exist();

    const NECESSARY_FOLDERS: [&str; 2] = ["Public", "Private"];

    for folder in NECESSARY_FOLDERS {
        let path = module_path.join(folder);
        path.create_dir_if_not_exist();
    }
}

fn write_build_cs(module_name: &str, module_path: &Path) {
    let result = apply_name_to_template!("template/BuildCsTemplate", module_name);

    let output = module_path.join(format!("{module_name}.Build.cs"));
    output.write_content(result);
}

fn write_module_interface(module_name: &str, module_path: &Path) {
    let header = apply_name_to_template!("template/ModuleInterfaceHeader", module_name);
    let cpp = apply_name_to_template!("template/ModuleInterfaceCpp", module_name);

    module_path.join(format!("Public/{module_name}Module.h")).write_content(header);
    module_path.join(format!("Private/{module_name}Module.cpp")).write_content(cpp);
}

fn write_uprojcet(module_name: &str, module_path: &Path) {
    let uproject = apply_name_to_template!("template/UProject", module_name);
    module_path.join(format!(".uproject")).write_content(uproject);
}