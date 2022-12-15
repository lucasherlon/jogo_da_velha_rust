use std::io;
use std::io::prelude::*;

fn main() {
    let mut tabuleiro: [[char;3];3] = [['*';3];3];
    let mut jogardor_1 = String::new();
    let mut jogardor_2 = String::new();

    print!("Jogador 1, digite seu nome: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut jogardor_1).expect("Input error!");

    print!("Jogador 2, digite seu nome: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut jogardor_2).expect("Input error!");

    println!("{}, seu símbolo é o: X", jogardor_1);
    println!("{}, seu símbolo é o: X", jogardor_2);

    let mut linha: u8 = 0;
    let mut coluna: u8 = 0;
    let mut simbolo: char;

    


    

}

/*
    for i in 0..3 {
        for j in 0..3 {
            print!(" {} | ", tabuleiro[i][j]);
        }
        println!()
    }
    */