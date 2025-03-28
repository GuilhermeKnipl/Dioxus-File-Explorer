use dioxus::prelude::*;
use std::fs::{self,ReadDir};
use std::collections::BTreeMap;

static CSS: Asset = asset!("/assets/main.css");
#[derive(Clone)]
struct FileInfo{
    file_name: String,
    file_type: String,
    file_path: String,
}

struct TreePath{
    id: i16,
    path: String
}



fn main() {
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    //let mut path_tree: BTreeMap<i16, String> = BTreeMap::new();

    let mut path_tree = use_signal(|| BTreeMap::<i16, String>::new());

    let initial_path = use_signal(||"C:\\Users\\LynWin\\Desktop".to_string());
    if path_tree.read().len() == 0 {
        path_tree.write().insert(1, initial_path.read().to_string());
    }
    //path_tree.set(path_tree.read().insert(1, initial_path.read().to_string()));

    let cwd_dir = fs::read_dir(initial_path()).unwrap();

    //table_insert(path_tree, initial_path.to_string());

    let cwd_files = get_fdir(cwd_dir);
    println!("{:?}",path_tree.read());

    //let mut path: Signal<String> = use_signal(|| "C:\\Users\\LynWin\\Desktop".to_string());
    rsx! {
        document::Stylesheet { href: CSS }
        h1 { "Lyn File Explorer" } 
        h1 { "{initial_path:?}"}
        button {onclick: move |_| down_dir(initial_path.clone()), "Page down" }
        button {onclick: move |_| arrow_nav(path_tree.clone(), initial_path.clone()), "Last Page (BTree)" }
        h2 { "{path_tree.read().len()}" }
        ul {
            for file in cwd_files {
                if file.file_type == "Dir"{
                    button {
                        id: "path_button",
                        onclick: move |_| button_path(initial_path, path_tree, file.file_path.clone()),
                        "📂{file.file_name}",
                        }br {}}
                
            }
        }
    }
}


fn down_dir(mut path:Signal<String>){
    let path_str = path.read().clone();
    if let Some(last_backslash_pos) = path_str.rfind("\\") {
        let parent_dir = &path_str[0..last_backslash_pos].to_string();
        path.set(parent_dir.to_string());
    }
}

fn arrow_nav(mut path_tree: Signal<BTreeMap<i16, String>>, mut path:Signal<String>){

    let key: i16 = path_tree.read().keys().last().copied().unwrap_or(0) - 1;

    let last_path = path_tree.read().get(&key).map(|p| p.to_string());

    if let Some(last_path) = last_path {
        if key != 0 {
            path.set(last_path.to_string());
            table_insert(&mut path_tree, last_path);  // Pass mutable reference to `table_insert`
        }
    } 
    
}

fn button_path(mut initial_path: Signal<String>, mut path_tree: Signal<BTreeMap<i16, String>>, new_path: String){

    initial_path.set(new_path.clone());

    table_insert(&mut path_tree, initial_path.to_string());
}

fn table_insert(table: &mut Signal<BTreeMap<i16, String>>, clicked_path :String){
    let last_key: i16 = table.read().keys().last().copied().unwrap_or(0);
    let id: i16 = last_key + 1;

    table.write().insert(id, clicked_path);

}

fn get_fdir(dir: ReadDir) -> Vec<FileInfo>{
    let mut file_infos: Vec<FileInfo> = Vec::new();

    for path in dir{

        if let Ok(dir_file) = path {
            let ftype_bool = dir_file.file_type().unwrap();
            let fname = dir_file.file_name().into_string().unwrap();
            let fpath = dir_file.path().to_str().unwrap().to_string(); 

            let ftype = match (ftype_bool.is_dir(), ftype_bool.is_file(), ftype_bool.is_symlink()) {
                (true, false, false) => "Dir",
                (false, true, false) => "File",
                (false, false, true) => "Symlink",
                _ => "None",
            }.to_string();

            file_infos.push(FileInfo { 
                file_name: fname, 
                file_type: ftype, 
                file_path: fpath });
                
            
    }}
    println!("Dir has {} files ",file_infos.len());

    return file_infos;
}

