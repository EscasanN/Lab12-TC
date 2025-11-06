use std::collections::HashSet;

fn remove_all(list: Vec<String>, to_delete: Vec<String>) -> Vec<String> {
    let del: HashSet<String> = to_delete.into_iter().collect();
    list.into_iter().filter(|x| !del.contains(x)).collect()
}

fn main() {
    let lista = vec![
        "rojo", "verde", "azul", "amarillo", "gris", "blanco", "negro"
    ].into_iter().map(|s| s.to_string()).collect();

    let borrar = vec!["amarillo", "cafe", "blanco"]
        .into_iter().map(|s| s.to_string()).collect();

    let restante = remove_all(lista, borrar);
    println!("{restante:?}"); // ["rojo", "verde", "azul", "gris", "negro"]
}
