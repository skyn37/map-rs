use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::env;

mod draw;
mod random;


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirInfo {
    root: bool,
    name: String,
    size: u64, 
    total_size: u64,
    percent_of_total: f64,
    percent_of_parent: f64,
    root_percent_of_total: f64,
    file_counter: u64,
    path: PathBuf,
    children: Vec<DirInfo>,

}

impl DirInfo {
    fn calculate_total_size(&mut self) -> u64 {
        let own_size = self.size;
        let children_size: u64 = self.children.iter_mut().map(|child| child.calculate_total_size()).sum();
        self.total_size = own_size + children_size;
        self.total_size
    }

    fn calculate_percentages(&mut self, root_total_size: u64) {
        if self.root {
            self.root_percent_of_total = (self.size as f64 / root_total_size as f64) * 100.0;
        }

        self.percent_of_total = if self.root {
            100.0
        } else {
            (self.size as f64 / root_total_size as f64) * 100.0  
        };

        for child in &mut self.children {
            child.calculate_percentages(root_total_size);
        }
    }

    fn calculate_percent_of_parent(&mut self, totalsize: u64) {
        for child in &mut self.children {
            child.percent_of_parent = (child.total_size as f64 /  totalsize as f64) * 100.0;
            child.calculate_percent_of_parent(child.total_size);
        }
    }

}



fn main () {
    const DEPTH_LEVEL: usize = 1;
    let dir_path = env::args().nth(1).unwrap_or_else(|| {
        env::current_dir()
            .unwrap_or_else(|err| panic!("Failed to get current directory: {}", err))
            .into_os_string()
            .into_string()
            .unwrap_or_else(|os_string| {
                panic!("Failed to convert OsString to String: {:?}", os_string)
            })
    });

    let path = Path::new(&dir_path);
    let mut dir_info = DirInfo {
        name: path.file_name().expect("filename Error").to_os_string().into_string().expect("convertion error"),
        size: 0,
        total_size: 0,
        percent_of_total: 0.0,
        percent_of_parent: 0.0,
        root_percent_of_total: 0.0,
        file_counter: 0,
        root: true,
        children: Vec::new(),
        path: path.to_path_buf(),
    };


    eval_dir(path, &mut dir_info, DEPTH_LEVEL);

    dir_info.calculate_total_size();
    dir_info.calculate_percentages(dir_info.total_size);
    dir_info.calculate_percent_of_parent(dir_info.total_size);
println!("{dir_info:#?}");

    sort_dir_info_by_total_size(&mut dir_info);
   println!("{dir_info:#?}");
    draw::noise(dir_info);
}




fn sort_dir_info_by_total_size(dir_info: &mut DirInfo) {
    dir_info.children.sort_by(|a, b| a.total_size.cmp(&b.total_size));

    for child in &mut dir_info.children {
        sort_dir_info_by_total_size(child);
    }
}




fn eval_dir(path: &Path, dir_info: &mut DirInfo,  depth: usize) {
    if path.is_dir() {
        for content in fs::read_dir(&path).expect("read rid error") {
            if let Ok(content) = content {
                if content.path().is_dir() {
                    let new_dir_info = DirInfo {
                        name: content.path().file_name().expect("filename Error").to_os_string().into_string().expect("convertion error"),
                        size: 0,
                        total_size: 0,
                        percent_of_total: 0.0,
                        percent_of_parent: 0.0,
                        root_percent_of_total: 0.0,
                        file_counter: 0,
                        root: false,
                        children: Vec::new(),
                        path: content.path(),
                    };
                    dir_info.children.push(new_dir_info);
                    if let Some(last_child) = dir_info.children.last_mut() {
                        eval_dir(&content.path(), last_child, depth + 1);
                    }
                } else if content.path().is_file() {
                    dir_info.file_counter += 1;
                    if let Ok(metadata) = fs::metadata(content.path()) {
                        dir_info.size += metadata.len();

                    }
                }
            }

        };
    }
}

