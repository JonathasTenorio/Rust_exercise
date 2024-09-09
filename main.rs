fn main() {
    let notas:[f32;4] = [6.7; 4];
    let inteiro:usize =  0;

    println!("{}", notas[inteiro]);

    for indice in 0..notas.len() {
        println!("A nota {} é = {}", indice + 1, notas[indice]);
    }

    matriz();
    println!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));

    //let dia: DiaDaSemana = DiaDaSemana::Sexta;

    cores();
    conteudo_opcional();
    vectors();

}

#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

fn eh_fim_de_semana(dia_da_semana:DiaDaSemana) -> bool 
{
    match dia_da_semana
    {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
}

fn matriz(){

    let matriz:[[f32; 3]; 2] = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4]
    ];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8 }
}

fn cores(){
    let cor = Color::Cmyk{cyan: 100, magenta: 50, yellow: 70,  black: 255};//Color::RgbColor(0,01,0);
    println!("Cor = {}", match cor{
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "azul",
        Color::RgbColor(0, 0, 0) | Color::Cmyk{cyan:_, magenta:_, yellow:_, black:255}  => "preta",
        Color::RgbColor(_, _, _) => "RGB desconhecido",
        Color::Cmyk{cyan:_, magenta:_, yellow:_, black:_} => "CMYK desconhecida"
    })
}

fn conteudo_opcional(){
    let conteudo_arquivo = ler_arquivo(String::from(""));
    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo nao existe")
    };

    println!("{:?}", &conteudo_arquivo);
 
    if let Some(valor) = conteudo_arquivo {
        println!("Agora tenho certeza de que ha o valor {}", valor);
    }
}

fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
}

fn vectors(){
    let mut notas:Vec<f32> = Vec::new(); //vec![10.0, 8.8, 6.5];
    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);

    println!("{:?}", notas);
}