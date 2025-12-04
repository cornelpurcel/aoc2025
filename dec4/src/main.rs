use std::env;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
enum MyError {
    GridCreationError(String),
    ContainerCreationError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::GridCreationError(msg) => write!(f, "Could not create grid: {}", msg),
            MyError::ContainerCreationError(msg) => {
                write!(f, "Could not create container: {}", msg)
            }
        }
    }
}

impl std::error::Error for MyError {}

#[derive(Debug, PartialEq)]
enum Container {
    ToiletPaper,
    Empty,
}

impl Container {
    fn from_char(c: char) -> Result<Self, MyError> {
        match c {
            '@' => Ok(Container::ToiletPaper),
            '.' => Ok(Container::Empty),
            _ => Err(MyError::ContainerCreationError(format!("unknown char {c}"))),
        }
    }
}

struct Row {
    containers: Vec<Container>,
}

impl Row {
    fn get_container(&self, index: usize) -> Option<&Container> {
        self.containers.get(index)
    }

    fn len(&self) -> usize {
        self.containers.len()
    }
}

struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    fn from_rows(rows: Vec<Row>) -> Result<Grid, MyError> {
        let row_lengths: Vec<usize> = rows.iter().map(|row| row.len()).collect();
        let row_length = row_lengths
            .get(0)
            .ok_or(MyError::GridCreationError("no rows given".to_string()))?;
        if row_lengths.iter().any(|length| length != row_length) {
            return Err(MyError::GridCreationError(
                "row lengths are not equal".to_string(),
            ));
        }

        Ok(Grid { rows: rows })
    }

    fn get_size(&self) -> (usize, usize) {
        (self.rows.len(), self.rows.get(0).map_or(0, |row| row.len()))
    }

    fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    fn get_container(&self, row_index: usize, col_index: usize) -> Option<&Container> {
        let row = self.get_row(row_index);
        match row {
            None => None,
            Some(row) => row.get_container(col_index),
        }
    }

    fn is_toilet_paper_reachable(&self, row_index: usize, col_index: usize) -> bool {
        let container = self.get_container(row_index, col_index);
        match container {
            None => return false,
            Some(Container::Empty) => return false,
            _ => (),
        }

        let starting_row_index = if row_index == 0 { 0 } else { row_index - 1 };
        let starting_col_index = if col_index == 0 { 0 } else { col_index - 1 };

        let mut adjacent_toilet_papers = 0;
        for y in starting_row_index..=(row_index + 1) {
            for x in starting_col_index..=(col_index + 1) {
                if y == row_index && x == col_index {
                    continue;
                }

                let adjacent_container = self.get_container(y, x);
                match adjacent_container {
                    Some(Container::ToiletPaper) => adjacent_toilet_papers += 1,
                    _ => (),
                }
            }
        }
        adjacent_toilet_papers < 4
    }

    fn remove_toilet_paper(&mut self, row_index: usize, col_index: usize) {
        self.rows[row_index].containers[col_index] = Container::Empty;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("reading the toilet paper grid from {file_path}");

    let mut grid = parse_file(file_path)?;

    let (row_count, col_count) = grid.get_size();

    let mut reachable_tp_history: Vec<i32> = Vec::new();
    let mut reachable_toilet_papers = 1;
    while reachable_toilet_papers > 0 {
        reachable_toilet_papers = 0;

        for row_index in 0..row_count {
            for col_index in 0..col_count {
                if grid.is_toilet_paper_reachable(row_index, col_index) {
                    reachable_toilet_papers += 1;
                    grid.remove_toilet_paper(row_index, col_index);
                }
            }
        }
        reachable_tp_history.push(reachable_toilet_papers);
    }

    let total_removed_papers = reachable_tp_history.iter().sum::<i32>();
    let iterations = reachable_tp_history.len();
    println!(
        "analyzed {row_count} rows with {col_count} spots each found out that we have {total_removed_papers} reachable toilet papers in {iterations} iterations"
    );
    Ok(())
}

fn parse_file(file_path: &str) -> Result<Grid, MyError> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut rows: Vec<Row> = Vec::new();
    for line in contents.split_whitespace() {
        let mut row: Vec<Container> = Vec::new();
        for character in line.chars() {
            let container = Container::from_char(character)?;
            row.push(container);
        }
        rows.push(Row { containers: row });
    }

    Grid::from_rows(rows)
}
