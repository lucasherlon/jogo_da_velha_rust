use std::io;
use std::io::prelude::*;

fn main() {
    let mut tabuleiro: [[char;3];3] = [['*';3];3];
    let mut jogador_1 = String::new();
    let mut jogador_2 = String::new();

    println!("{:-^40}", "BEM-VINDOS AO JOGO DA VELHA");

    print!("Jogador 1, digite seu nome: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut jogador_1).expect("Input error!");
    let jogador_1: &str = jogador_1.trim();

    print!("Jogador 2, digite seu nome: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut jogador_2).expect("Input error!");
    let jogador_2: &str = jogador_2.trim();

    println!("{}, seu símbolo é o: X", jogador_1);
    println!("{}, seu símbolo é o: O", jogador_2);

    let mut linha: usize = 0;
    let mut coluna: usize = 0;
    let mut rodada: usize = 1;
    let mut simbolo: char;

    'jogo: loop {

        simbolo = if rodada % 2 == 1 {
            println!("{}, é a sua vez!", jogador_1);
            'X'
        } else {
            println!("{}, é a sua vez!", jogador_2);
            'O'
        };

        let mut linha_valida: bool = false;

        while !linha_valida {
            let mut linha_s = String::new();

            print!("Digite o número da linha [1,2 ou 3]: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut linha_s).expect("Input error!");
            linha = match linha_s.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("Parsing error"),
            };

            if linha >= 1 && linha <= 3 {
                linha_valida = true;
            } else {
                println!("Entrada inválida. Tente novamente...");
                io::stdout().flush().unwrap();
            }
        }

        let mut coluna_valida: bool = false;

        while !coluna_valida {
            let mut coluna_s = String::new();

            print!("Digite o número da coluna [1,2 ou 3]: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut coluna_s).expect("Input error!");
            coluna = match coluna_s.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("Parsing error!"),
            };
            
            if coluna >= 1 && coluna <= 3 {
                coluna_valida = true;
            } else {
                println!("Entrada inválida. Tente novamente...");
            }
        }

        //Ajustando linha e coluna para usa-los como indice
        linha -= 1;
        coluna -= 1;
        
        //Verificando se a posição digitada já está preenchida
        if tabuleiro[linha][coluna] == 'X' || tabuleiro[linha][coluna] == 'O' {
            println!("Posição já preenchida. Tente novamente...");
        } else {
            tabuleiro[linha][coluna] = simbolo;
         rodada += 1;
        }

        //imprimir tabuleiro
        for i in 0..3 {
            for j in 0..3 {
                print!("[ {} ]  ", tabuleiro[i][j]);
            }
            println!()
        }

        //Verificando se há ganhador
        if tabuleiro[0][0] == 'X' && tabuleiro[0][1] == 'X' && tabuleiro[0][2] == 'X' || 
        (tabuleiro[1][0] == 'X' && tabuleiro[1][1] == 'X' && tabuleiro[1][2] == 'X') ||
        (tabuleiro[2][0]  == 'X' && tabuleiro[2][1] == 'X' && tabuleiro[2][2] == 'X') ||
        (tabuleiro[0][0] == 'X' && tabuleiro[1][0] == 'X' && tabuleiro[2][0] == 'X') ||
        (tabuleiro[0][1] == 'X' && tabuleiro[1][1] == 'X' && tabuleiro[2][1] == 'X') ||
        (tabuleiro[0][2] == 'X' && tabuleiro[1][2] == 'X' && tabuleiro[2][2] == 'X') ||
        (tabuleiro[0][0] == 'X' && tabuleiro[1][1] == 'X' && tabuleiro[2][2] == 'X') ||
        (tabuleiro[0][2] == 'X' && tabuleiro[1][1] == 'X' && tabuleiro[2][0] == 'X') {
            println!("Parabéns, {}, você ganhou!", jogador_1);
            break 'jogo;
        } else if tabuleiro[0][0] == 'O' && tabuleiro[0][1] == 'O' && tabuleiro[0][2] == 'O' || 
        (tabuleiro[1][0] == 'O' && tabuleiro[1][1] == 'O' && tabuleiro[1][2] == 'O') ||
        (tabuleiro[2][0]  == 'O' && tabuleiro[2][1] == 'O' && tabuleiro[2][2] == 'O') ||
        (tabuleiro[0][0] == 'O' && tabuleiro[1][0] == 'O' && tabuleiro[2][0] == 'O') ||
        (tabuleiro[0][1] == 'O' && tabuleiro[1][1] == 'O' && tabuleiro[2][1] == 'O') ||
        (tabuleiro[0][2] == 'O' && tabuleiro[1][2] == 'O' && tabuleiro[2][2] == 'O') ||
        (tabuleiro[0][0] == 'O' && tabuleiro[1][1] == 'O' && tabuleiro[2][2] == 'O') ||
        (tabuleiro[0][2] == 'O' && tabuleiro[1][1] == 'O' && tabuleiro[2][0] == 'O') {
            println!("Parabéns, {}, você ganhou!", jogador_2);
            break 'jogo;
        } else if rodada > 9 {
            println!("Deu velha. Ninguém ganhou!");
            break 'jogo;
        }

    }

}