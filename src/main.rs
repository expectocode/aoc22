use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::ops::AddAssign;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    day9();
}

#[allow(dead_code)]
fn day1() {
    let input = read_file("day1input.txt");
    let mut elves_cals = Vec::new();

    let mut current_elf_cal = 0;
    for line in input.split('\n') {
        if line == "" {
            elves_cals.push(current_elf_cal);
            current_elf_cal = 0;
            continue;
        }
        let value: i32 = line.parse().expect("bad parse");
        current_elf_cal += value;
    }
    elves_cals.sort();
    dbg!(elves_cals[elves_cals.len() - 3..elves_cals.len()]
        .iter()
        .sum::<i32>());
}

#[derive(Debug)]
struct Folder {
    files_size: i32,
    subfolders: Vec<PathBuf>,
}

#[allow(dead_code)]
fn day7() {
    let input = read_file("day7input.txt");
    let mut cur_path: PathBuf = "/".into();
    let mut dirs: HashMap<PathBuf, Folder> = HashMap::new();

    for command in input.split("$ ") {
        if command.starts_with("cd /") {
            cur_path = "/".into();
            continue;
        }
        if command.starts_with("cd ..") {
            cur_path.pop();
            continue;
        }
        if command.starts_with("cd ") {
            let folder = command.split(' ').nth(1).expect("bad cd").trim_end();
            cur_path.push(folder);
            continue;
        }
        if command.starts_with("ls") {
            let results: Vec<_> = command.trim().split('\n').skip(1).collect();
            let mut subdirs = vec![];
            let mut files_size = 0;
            for result in results {
                if result.starts_with("dir") {
                    let dir = result.split(' ').nth(1).unwrap();
                    let mut result_path = cur_path.clone();
                    result_path.push(dir);
                    subdirs.push(result_path);
                } else {
                    let size: i32 = result.split(' ').next().unwrap().parse().unwrap();
                    files_size += size;
                }
            }
            let dir = Folder {
                files_size,
                subfolders: subdirs,
            };
            dirs.insert(cur_path.clone(), dir);
        }
    }

    // go through all dirs, find sizes
    let mut sizes: Vec<_> = dirs
        .values()
        .map(|dir| day7_dir_size(dir, &dirs))
        .filter(|&x| x <= 100000)
        .collect();
    sizes.sort();
    dbg!(sizes.iter().sum::<i32>());

    // part 2
    let used_space = day7_dir_size(dirs.get(Path::new("/")).unwrap(), &dirs);
    let space_needed = -70000000 + used_space + 30000000;
    let mut sizes_above: Vec<_> = dirs
        .values()
        .map(|dir| day7_dir_size(dir, &dirs))
        .filter(|&x| x >= space_needed)
        .collect();
    sizes_above.sort();
    dbg!(sizes_above.iter().next().unwrap());
}

fn day7_dir_size(folder: &Folder, dirs: &HashMap<PathBuf, Folder>) -> i32 {
    let mut size = folder.files_size;
    for subdir in folder.subfolders.iter() {
        size += day7_dir_size(dirs.get(subdir).unwrap(), dirs);
    }

    size
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl AddAssign<(i32, i32)> for Position {
    fn add_assign(self: &mut Position, (dx, dy): (i32, i32)) {
        self.x += dx;
        self.y += dy;
    }
}

#[allow(dead_code)]
fn day9() {
    let input = read_file("day9input.txt");
    let mut head_pos = Position { x: 0, y: 0 };
    let mut tail_pos = Position { x: 0, y: 0 };
    let mut tail_positions: HashSet<Position> = HashSet::new();
    tail_positions.insert(tail_pos);

    for line in input.trim().split('\n') {
        let direction = line.chars().next().unwrap();
        let count: i32 = line.split(' ').nth(1).unwrap().parse().unwrap();

        for _ in 0..count {
            match direction {
                'R' => head_pos.x += 1,
                'L' => head_pos.x -= 1,
                'U' => head_pos.y += 1,
                'D' => head_pos.y -= 1,
                _ => panic!(),
            }
            if day9_string_touching(head_pos, tail_pos) {
                continue;
            }

            let (dx, dy) = match day9_pos_diff(head_pos, tail_pos) {
                (-1, 2) => (-1, 1),
                (0, 2) => (0, 1),
                (1, 2) => (1, 1),

                (2, 1) => (1, 1),
                (2, 0) => (1, 0),
                (2, -1) => (1, -1),

                (1, -2) => (1, -1),
                (0, -2) => (0, -1),
                (-1, -2) => (-1, -1),

                (-2, -1) => (-1, -1),
                (-2, 0) => (-1, 0),

                (-2, 1) => (-1, 1),

                _ => panic!(),
            };
            tail_pos += (dx, dy);
            tail_positions.insert(tail_pos);
        }
    }
    dbg!(tail_positions.len());
}

fn day9_string_touching(head: Position, tail: Position) -> bool {
    (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1
}

fn day9_pos_diff(a: Position, b: Position) -> (i32, i32) {
    (a.x - b.x, a.y - b.y)
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("bad file")
}
