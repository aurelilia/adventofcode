use crate::intcode::{Program, Status};

pub fn a13_2() {
    let mut program = Program::new("inputs/input-13");
    let mut tiles = [[0; 50]; 50];

    let mut status = program.execute_halt();
    while status != Status::Halted {
        let (x, y, tile) = (
            status.into_output() as usize,
            program.resume_output().into_output() as usize,
            program.resume_output().into_output(),
        );
        tiles[x][y] = tile;
        status = program.resume_output();
    }

    let count = tiles.iter().fold(0, |a, r| {
        a + r.iter().fold(0, |a, i| a + ((*i == 2) as i32))
    });
    println!("{}", count);
}

pub fn a13_1() {
    let mut program = Program::new("inputs/input-13");
    let mut tiles = [[0; 50]; 50];

    let mut status = program.execute_halt();
    while status != Status::Halted {
        let (x, y, tile) = (
            status.into_output() as usize,
            program.resume_output().into_output() as usize,
            program.resume_output().into_output(),
        );
        tiles[x][y] = tile;
        status = program.resume_output();
    }

    let count = tiles.iter().fold(0, |a, r| {
        a + r.iter().fold(0, |a, i| a + ((*i == 2) as i32))
    });
    println!("{}", count);
}
