const PI:f32 = 3.14;
static mut VARIAVEL_GLOBAL:u8 = 1;

fn soma(a:i32, b:i32) -> i32
{
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn sombra(){
    let a = 123;

    {
        let b = 456;
        println!("Dentro, b = {}", b);
        
        let a = 777;
        println!("Dentro, a = {}", a);        
    }

    println!("Fora, a = {}", a);
}

fn escopo(){
    println!("PI = {}", PI);

    unsafe{
    println!("variavel_global = {}", VARIAVEL_GLOBAL);
    }   
    let variavel:i32 = 300;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}",decimal);

    let booleana = false;
    //booleana = true;
    println!("Booleana = {}, Tamanho booleana = {}", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho do char = {}", std::mem::size_of_val(&letra));
}

fn main(){
    escopo();
    sombra();
    println!("Soma = {}", soma(2,2));

   condicionais();
   repeticoes();

   ownership();

   pattern_matching();
   erros();

}

fn condicionais(){

    let idade:u8 = 17;
    let responsavel_autorizou = true;
    let e_maior = idade >= 18;

    if e_maior {
        println!("Pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou{
        println!("Pode entrar com assinatura do responsável");
    } else {
        println!("Não pode entrar na balada");
    }

    let condicao = if e_maior { "maior" } else { "menor" };

    println!("É {} de idade", condicao);

    let linguagem = "PHP";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };

    println!("O proposito de {} é {}", linguagem, proposito);

}

fn repeticoes(){

    let multiplicador:u8 = 5;

    let mut contador:u8 = 0;

    while contador <10 {
    contador += 1;
    if contador == 5 {
        continue
    }

    println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    
    }

    contador = 0;
    loop {
        contador +=1;

        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);

        if contador == 10{
            break;
        }   
    }
    
    for i in 1..=10 {//for i in 1..11
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }
}

fn ownership(){

    let mut uma_string = String::from("Vinicius");
    rouba(&mut uma_string);
    println!("{}",uma_string);

}

fn rouba(string: &mut String){
    string.push_str(" Dias");
    println!("{}", string)
}

fn pattern_matching(){
    for x in 1..=20{
        println!("{} : {}", x, match x{
            1 => "Pouco",
            2 | 3 => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if x % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        });
    }
}

fn erros(){
   // panic!("Erro Proposital");
    match resultado(){
        Ok(s) => println!("String de sucesso = {}", s),
        Err(numero) => println!("Codigo de erro = {}", numero)
    };
}

fn resultado() -> Result<String, u8>{
    //Ok(String::from("Tudo de certo"))
    Err(42)
}