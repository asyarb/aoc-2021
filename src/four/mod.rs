type Column = Vec<Cell>;
type Row = Vec<Cell>;

#[derive(Debug, Clone)]
struct Board {
    pub rows: Vec<Row>,
    pub cols: Vec<Column>,
}

#[derive(Debug, Clone)]
struct Cell {
    pub on: bool,
    pub value: usize,
    pub position: (usize, usize),
}

pub fn part_one() {
    let (nums, boards) = include_str!("./sample.txt").split_once("\n\n").unwrap();

    let boards = boards
        .split("\n\n")
        .map(|board| {
            board
                .split_whitespace()
                .map(|str_num| str_num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|board| {
            let rows: Vec<Row> = board
                .chunks(5)
                .enumerate()
                .map(|(row_idx, chunk)| {
                    chunk
                        .iter()
                        .enumerate()
                        .map(|(col_idx, num)| Cell {
                            on: false,
                            value: *num,
                            position: (row_idx, col_idx),
                        })
                        .collect::<_>()
                })
                .collect();

            let cols: Vec<Column> = [0 as usize; 4]
                .iter()
                .map(|idx| rows.iter().nth(*idx).unwrap().clone())
                .collect();

            Board { rows, cols }
        })
        .collect::<Vec<_>>();

    for board in boards {
        println!("");
        for row in board.rows {
            println!("");
            for row in row {
                print!("{} ", row.value);
            }
        }
    }

    let nums = nums
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .find_map(|num| unimplemented!())
        .unwrap();

    todo!()
}
