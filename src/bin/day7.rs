use std::{error, fs, path, io::{self, BufRead}};

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

#[derive(Debug)]
struct Dir {
    name: String,
    subdirs: Vec<Dir>,
    files: Vec<File>,
}

#[derive(Debug)]
struct DirReport {
    name: String,
    depth: usize,
    size: u64,
}

fn dir_reports(dir: &Dir) -> Vec<DirReport> {
    let mut reports: Vec<DirReport> = dir.subdirs
        .iter()
        .map(|subdir|
            dir_reports(subdir)
            .iter()
            .map(|r| DirReport{
                name: (dir.name.clone() + "/" + &r.name),
                depth: r.depth + 1,
                size: r.size,
            })
            .collect::<Vec<DirReport>>()
        )
        .flatten()
        .collect();
    let sub_size: u64 = reports.iter().filter(|d| d.depth == 1).map(|d| d.size).sum();
    let local_size: u64 = dir.files.iter().map(|f| f.size).sum();
    reports.push(DirReport{
        name: dir.name.clone(),
        depth: 0,
        size: sub_size + local_size,
    });
    return reports;
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut root = Dir{ name: "".into(), subdirs: Vec::new(), files: Vec::new() };

    let mut cur_path: Vec<String> = Vec::new();
    let file = fs::File::open(path::Path::new("./data/day7.txt"))?;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }

        if line.chars().nth(0) == Some('$') {
            let (_, cmd) = line.split_at(2);
            if cmd.starts_with("cd") {
                let (_, tgt) = cmd.split_at(3);
                if tgt == "/" {
                    cur_path = Vec::new();
                } else if tgt == ".." {
                    cur_path.pop();
                } else {
                    cur_path.push(tgt.into());
                }
            }
        } else {
            let mut cur_dir: &mut Dir = &mut root;
            for entry in &cur_path {
                cur_dir = cur_dir.subdirs.iter_mut().find(|subdir| &subdir.name == entry).unwrap();
            }

            let space_pos = line.find(' ').unwrap();
            let (size, name) = line.split_at(space_pos + 1);
            let size = size.trim();
            if size == "dir" {
                cur_dir.subdirs.push(Dir{ name: name.into(), subdirs: Vec::new(), files: Vec::new() })
            } else {
                let size: u64 = size.parse().unwrap();
                cur_dir.files.push(File{ name: name.into(), size: size});
            }
        }
    }

    dbg!(&root);

    let reports = dir_reports(&root);
    dbg!(&reports);

    let free_space = 70000000 - reports.last().unwrap().size;
    let needed = 30000000 - free_space;
    dbg!(free_space);
    dbg!(needed);

    let delete = reports.iter().filter(|r| r.size > needed).min_by_key(|r| r.size);
    dbg!(delete);

    Ok(())
}
