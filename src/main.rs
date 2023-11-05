use clap::{App, Arg};
use rand::{seq::SliceRandom};
use clipboard::ClipboardProvider;

fn generate_password(length: usize, use_special_chars: bool, use_uppercase: bool, use_numbers: bool, use_all: bool) -> String {
    let mut rng = rand::thread_rng();

    let mut charset = Vec::from("abcdefghijklmnopqrstuvwxyz".as_bytes());
    if use_uppercase || use_all {
        charset.extend_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_special_chars || use_all {
        charset.extend_from_slice(b"!@#$%^&*()_+-=[]{};:',.<>/?");
    }
    if use_numbers || use_all {
        charset.extend_from_slice(b"0123456789");
    }

    let password: String = (0..length)
        .map(|_| {
            let &char_byte = charset.choose(&mut rng).unwrap();
            char_byte as char
        })
        .collect();

    password
}

fn main() {
    let matches = App::new("Genp Password Generator")
        .version("0.1")
        .author("crashunix")
        .about("Gerar senha papai")
        // .arg(
        //     Arg::with_name("LENGTH")
        //         .help("Define o tamanho da senha")
        //         .default_value("8")
        //         .index(1),
        // )
        .arg(
            Arg::with_name("length")
                .short('l')
                .long("length")
                .takes_value(true)
                .value_name("LENGTH")
                .default_value("10")
                .help("Define o tamanho da senha")
                
        )
        .arg(
            Arg::with_name("uppercase")
                .short('u')
                .long("uppercase")
                .help("Inclui letras maiúsculas na senha")
        )
        .arg(
            Arg::with_name("special")
                .short('s')
                .long("special")
                .help("Inclui caracteres especiais na senha"),
        )
        .arg(
            Arg::with_name("number")
                .short('n')
                .long("number")
                .help("Inclui números na senha"),
        )
        .arg(
            Arg::with_name("all")
                .short('a')
                .long("all")
                .help("Inclui caracteres especiais, números e letras maiúsculas na senha"),
        )
        .arg(
            Arg::with_name("copy")
                .short('c')
                .long("copy")
                .help("Copia a senha para o clipboard após gerar"),
        )
        .get_matches();

    let length = matches
        .value_of("length")
        .unwrap()
        .parse::<usize>()
        .expect("O comprimento deve ser um número");

    let use_uppercase = matches.is_present("uppercase");
    let use_special_chars = matches.is_present("special");
    let use_numbers = matches.is_present("number");
    let use_all = matches.is_present("all");

    let copy = matches.is_present("copy");

    let password = generate_password(length, use_special_chars, use_uppercase, use_numbers, use_all);

    if copy {
        let mut ctx = clipboard::ClipboardContext::new().unwrap();
        if ctx.set_contents(password.to_owned()).is_ok() {
            println!("Senha copiada para a área de transferência.");
        } else {
            println!("Falha ao copiar a senha para a área de transferência.");
        }
    }

    println!("Senha: {}", password);
}
