use crate::mapa_troco::TrocoMoeda;

mod mapa_troco;

fn main() {
    println!("Hello, world!");
    let moedas = vec![1, 2, 5, 10, 20, 50, 100, 200];
    println!("Entre com o valor a calcula o mapa de troco: ");
    let mut valor = String::new();
    std::io::stdin().read_line(&mut valor).expect("Erro ao ler o valor");
    let valor: i32 = valor.trim().parse().expect("Erro ao converter o valor");
    let troco = troco(moedas, valor);
    println!("O troco é: ");
    for troco_moeda in troco {
        println!("{} moedas de {}", troco_moeda.quantidade, troco_moeda.moeda);
    }
}

fn troco(moedas: Vec<i32>, valor: i32) -> Vec<TrocoMoeda> {
    let mut troco = Vec::new();
    let mut valor = valor;
    for moeda in moedas.iter().rev() {
        let quantidade = valor / moeda;
        if quantidade > 0 {
            troco.push(TrocoMoeda::new(*moeda, quantidade));
            valor -= moeda * quantidade;
        }
    }
    troco
}


