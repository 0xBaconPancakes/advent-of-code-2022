#[derive(Clone, Copy, Debug)]
struct Tree {
    height: u32,
    visible: bool,
}

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<Tree>>,
    width: usize,
    height: usize,
}

// Return number of trees with height but stop first time we see a tree with height = current_height
fn visible_trees<'a>(trees: impl IntoIterator<Item = &'a Tree>, current_height: u32) -> u32 {
    let mut count = 0;
    for tree in trees {
        if tree.height < current_height {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    count
}

impl Forest {
    fn count_visible(&self) -> u32 {
        self.trees
            .iter()
            .map(|row| row.iter().filter(|&t| t.visible).count() as u32)
            .sum()
    }

    fn update_visibility(&mut self) {
        for i in 0..self.height {
            let mut current_max_height = 0;
            for j in 0..self.width {
                let mut current_tree = &mut self.trees[i][j];
                if j == 0 {
                    current_tree.visible = true;
                    current_max_height = current_tree.height;
                } else if current_tree.height > current_max_height {
                    current_tree.visible = true;
                    current_max_height = current_tree.height;
                }
            }
            current_max_height = 0;
            for j in (0..self.width).rev() {
                let mut current_tree = &mut self.trees[i][j];
                if j == self.width - 1 {
                    current_tree.visible = true;
                    current_max_height = current_tree.height;
                } else if current_tree.height > current_max_height {
                    current_tree.visible = true;
                    current_max_height = current_tree.height;
                }
            }
        }
        for j in 0..self.width {
            let mut current_max_height = 0;
            for i in 0..self.height {
                let mut current_tree = &mut self.trees[i][j];
                if i == 0 {
                    current_tree.visible = true;
                    current_max_height = current_tree.height;
                } else if current_tree.height > current_max_height {
                    current_tree.visible = true;
                    current_max_height = current_tree.height;
                }
            }
            current_max_height = 0;
            for i in (0..self.height).rev() {
                let mut current_tree = &mut self.trees[i][j];
                if i == self.height - 1 {
                    current_tree.visible = true;
                    current_max_height = current_tree.height;
                } else if current_tree.height > current_max_height {
                    current_tree.visible = true;
                    current_max_height = current_tree.height;
                }
            }
        }
    }


    fn scenic_score(&self, i: usize, j: usize) -> u32 {
        let current_height = self.trees[i][j].height;
        let score_left = visible_trees(self.trees[i][0..j].iter().rev(), current_height);
        let score_right = visible_trees(self.trees[i][j + 1..].iter(), current_height);
        let score_up = visible_trees(self.trees[0..i].into_iter().map(|row| &row[j]).rev(), current_height);
        let score_down = visible_trees(self.trees[i + 1..].into_iter().map(|row| &row[j]), current_height);
        (score_left * score_right * score_up * score_down) as u32
    }

    fn best_scenic_score(&self) -> u32 {
        let mut best_score = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                let score = self.scenic_score(i, j);
                if score > best_score {
                    best_score = score;
                }
            }
        }
        best_score
    }
}

fn parse_input(input: &str) -> Forest {
    let mut trees = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let height = c.to_digit(10).unwrap();
            row.push(Tree {
                height,
                visible: false,
            });
        }
        trees.push(row);
    }
    let width = trees[0].len();
    let height = trees.len();
    Forest {
        trees,
        height,
        width,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut forest = parse_input(input);
    forest.update_visibility();
    Some(forest.count_visible())
}

pub fn part_two(input: &str) -> Option<u32> {
    let forest = parse_input(input);
    Some(forest.best_scenic_score())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
