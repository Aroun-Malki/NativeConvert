use std::env;

fn main() {
    println!("==========================");
    println!("       NativeConvert");
    println!("==========================");
    println!();

    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(dossier) => {
            println!("Dossier sélectionné :");
            println!("{}", dossier);
        }
        None => {
            println!("Aucun dossier reçu.");
            println!("Lance le programme avec : NativeConvert.exe \"chemin\"");
        }
    }

    println!();
    println!("Conversion engine: Not implemented yet.");

    println!();
    println!("Appuie sur Entrée pour fermer...");
    let mut reponse = String::new();
    std::io::stdin().read_line(&mut reponse).unwrap();
}
