/*
# Eu vou explicar o assunto de uma forma que seja acessível para todos. Vou usar exemplos práticos e linguagem simples para que todos possam entender. Não vou usar termos técnicos ou complicados, mas sim explicar de forma clara e objetiva. Vou me esforçar para que todos possam compreender o assunto de forma humana e intuitiva.


# #  Data de Criação: 18/07/2023 
# #  Horas: 10:33
# # Criado: Lanbyshell

Antes de tudo quero falar algo, Me contrate pois não tenho trabalho.


*/


use rusqlite::{Connection, Result}; /*Se você deseja usar a biblioteca rusqlite, a primeira coisa que você precisa fazer é adicioná-la ao seu arquivo Cargo.toml. Para fazer isso, abra o arquivo Cargo.toml e adicione a seguinte linha: rusqlite = { version = "0.29.0", features = ["bundled"] }. Isso permitirá que o Cargo baixe e instale a biblioteca rusqlite na versão 0.29.0 com todos os recursos embutidos. Depois de adicionar a linha, salve o arquivo Cargo.toml e execute o comando cargo build para compilar o seu projeto. Agora você está pronto para usar a biblioteca rusqlite em seu projeto. */
use std::process::exit; // Essa é uma lib padrão, vamos usar ela caso retonar um erro irrecuperável

fn main(){
    let conn = Connection::open("Banco.dbs"); //  o código cria um arquivo de banco de dados SQLite chamado "Banco.dbs" na mesma pasta onde o programa é executado. Vamos avançar para o próximo nível.
    // Obs: Podemos cria qualquer nome o nosso banco de dados '.db,.dbs, .teste', Sua preferecia.

    /* A variável  vai retornar um resultado que pode ser um "OK" ou um "Erro". Se o resultado for "OK", significa que a variável foi processada com sucesso. Se o resultado for "Erro", significa que houve algum problema durante o processamento da variável */
    // Quando eu falei um Ok estou falando da trait Connection 

    match conn {
        Ok(_) => { // Caso da um ok 
            println!("Tudo ok , Conexão feita.");
        },
        Err(_) => { // Caso da um erro
            println!("Pois ouve um erro ao conectar o banco de dados");
            exit(1); // o codigo vai dar pau ,ou melhor o codigo vai terminar.
        }
    }

    // Agora vamos criar uma função para executar a nosso sql.

    let conn = conn.unwrap();
    Sql_Exec(&conn); // aqui eu tenho que passar um referecia ao conn
    

    // add informaçoes 

    Add_Sql(&conn, "LANBY@GMAILTESTE.com", "lanbyteste@gmail.com", "Lanby20202020");

}
fn Sql_Exec(conn: &Connection) {// referencia da trait &Connection.

    conn.execute("CREATE TABLE IF NOT EXISTS logins ( 
        HOST TEXT NOT NULL,
        MAIL TEXT NOT NULL,
        PASSWORD TEXT NOT NULL
    )", []).unwrap_or_else(| _ | { // caso da um erro eu quero para o codigo 
        println!("Erro ao executar o Sql ao criar a tabela");
        exit(1);
    });
}

fn Add_Sql(conn: &Connection, HOST: &str, MAIL: &str ,PASSOWORD: &str){
    conn.execute("INSERT INTO logins (HOST, MAIL, PASSWORD) VALUES (?1, ?2 , ?3)",&[HOST, MAIL,PASSOWORD]).unwrap(); // Vai retonar um Result , mas usei um unwrap para descapsular.
    println!("Valores add: HOST : {} MAIL : {} PASSOWORD : {}",HOST,MAIL,PASSOWORD);
}
