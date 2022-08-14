use crate::types::Matrix;

pub fn parse_input(input: &String) -> (Matrix<char>, Vec<&str>) {
    let chunks: Vec<&str> = input.split("\r\n\r\n").collect();

    if chunks.len() != 2 {
        panic!("Length is {}", chunks.len());
    }

    let matrix = parse_character_matrix(chunks.get(0).unwrap());
    let words = parse_words(chunks.get(1).unwrap());

    (matrix, words)
}

fn parse_character_matrix(input: &str) -> Matrix<char> {
    let rows: Vec<&str> = input.split("\r\n").collect();
    let cols: Vec<&str> = rows.get(0).unwrap().split(" ").collect();

    let mut result: Matrix<char> = Matrix::new(rows.len(), cols.len());

    for i in 0..rows.len() {
        let cols: Vec<&str> = rows.get(i).unwrap().split(" ").collect();

        for j in 0..cols.len() {
            let elem: Vec<char> = cols.get(j).unwrap().to_owned().chars().collect();
            if elem.len() != 1 {
                panic!()
            }

            result.set(i, j, elem[0]);
        }
    }

    result
}

fn parse_words(input: &str) -> Vec<&str> {
    input.split("\r\n").collect()
}
