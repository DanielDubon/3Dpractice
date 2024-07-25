use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug, Clone)]
pub enum CellType {
    Wall,
    Path,
    Start,
    Goal,
}

impl CellType {
    pub fn color(&self) -> &'static str {
        match self {
            CellType::Wall => "RED",
            CellType::Path => "WHITE",
            CellType::Start => "GREEN",
            CellType::Goal => "BLUE",
        }
    }
}

pub fn make_maze(w: usize, h: usize) -> String {
    let mut vis = vec![vec![CellType::Wall; w]; h];
    vis.push(vec![CellType::Wall; w + 1]);

    let mut ver = vec![vec!["|  ".to_string(); w]; h];
    ver.push(vec!["|".to_string(); w + 1]);

    let mut hor = vec![vec!["+--".to_string(); w]; h + 1];
    hor.push(vec!["+".to_string(); w + 1]);

    fn walk(x: usize, y: usize, vis: &mut Vec<Vec<CellType>>, ver: &mut Vec<Vec<String>>, hor: &mut Vec<Vec<String>>) {
        vis[y][x] = CellType::Path;

        let mut d = vec![
            (x as isize - 1, y as isize), 
            (x as isize, y as isize + 1), 
            (x as isize + 1, y as isize), 
            (x as isize, y as isize - 1)
        ];
        d.shuffle(&mut rand::thread_rng());

        for (xx, yy) in d {
            if xx < 0 || yy < 0 || (xx as usize) >= vis[0].len() || (yy as usize) >= vis.len() {
                continue;
            }
            let xx = xx as usize;
            let yy = yy as usize;
            if let CellType::Path = vis[yy][xx] {
                continue;
            }
            if xx == x {
                hor[std::cmp::max(y, yy)][x] = "+  ".to_string();
            }
            if yy == y {
                ver[y][std::cmp::max(x, xx)] = "   ".to_string();
            }
            walk(xx, yy, vis, ver, hor);
        }
    }

    walk(rand::thread_rng().gen_range(0..w), rand::thread_rng().gen_range(0..h), &mut vis, &mut ver, &mut hor);

    let mut s = String::new();
    for (a, b) in hor.iter().zip(ver.iter()) {
        s.push_str(&(a.join("") + "\n")); 
        s.push_str(&(b.join("") + "\n")); 
    }

    let mut l: Vec<char> = s.chars().collect();
    let mut start_pos: usize;
    let mut goal_pos: usize;

    loop {
        start_pos = rand::thread_rng().gen_range(0..(l.len() / 2));
        if l[start_pos] == ' ' {
            break;
        }
    }

    loop {
        goal_pos = rand::thread_rng().gen_range(0..(l.len() / 2));
        if l[goal_pos] == ' ' && goal_pos != start_pos {
            break;
        }
    }

    if start_pos < l.len() {
        l[start_pos] = 'p';
    }
    if goal_pos < l.len() {
        l[goal_pos] = 'g';
    }

    l.iter().collect()
}
