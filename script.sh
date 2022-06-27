docker build --progress=plain -t sudoku-rust .
docker run --rm --name sudoku sudoku-rust