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
    println!("{}, seu símbolo é o: O", jogardor_2);

    let mut linha_s = String::new();
    let mut coluna_s = String::new();
    let mut jogada_s = String::new();
    let mut linha: u8 = 0;
    let mut coluna: u8 = 0;
    let mut jogada: u8 = 1;
    let mut simbolo: char;

    loop {
        if jogada % 2 == 1 {
            println!("{}, é a sua vez, escolha linha e coluna [1 a 3]", jogardor_1);
            simbolo = 'X';
        } else {
            println!("{}, é a sua vez, escolha linha e coluna [1 a 3]", jogardor_2);
            simbolo = 'O';
        }

        let mut linha_valida: bool = false;

        while !linha_valida {
            print!("Digite o número da linha [1,2,3]: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut linha_s).expect("Input error!");
            linha = linha_s.trim().parse::<u8>().unwrap();

            if linha >= 1 && linha <= 3 {
                linha_valida = true;
            } else {
                println!("Entrada inválida. Tente novamente...");
            }
        }

        let mut coluna_valida: bool = false;

        while !coluna_valida {
            print!("Digite o número da coluna [1,2,3]: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut coluna_s).expect("Input error!");
            coluna = coluna_s.trim().parse::<u8>().unwrap();

            if coluna >= 1 && coluna <= 3 {
                coluna_valida = true;
            } else {
                println!("Entrada inválida. Tente novamente...");
            }
        }


    }


    

}

/*
    for i in 0..3 {
        for j in 0..3 {
            print!(" {} | ", tabuleiro[i][j]);
        }
        println!()
    }
    */