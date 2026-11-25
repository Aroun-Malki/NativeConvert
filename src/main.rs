use std::env;
use std::io;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::HKEY_CURRENT_USER;

const EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "webp"];

fn emplacements() -> Vec<String> {
    let mut liste = vec![r"Software\Classes\Directory\shell\NativeConvert".to_string()];
    for ext in EXTENSIONS {
        liste.push(format!(
            r"Software\Classes\SystemFileAssociations\.{}\shell\NativeConvert",
            ext
        ));
    }
    liste
}

fn installer() -> std::io::Result<()> {
    let exe = env::current_exe()?;
    let chemin_icone = exe.with_file_name("logo.ico");
    verifier_fichier_requis(&chemin_icone)?;
    let icone = chemin_icone.display().to_string();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let commande = format!("\"{}\" \"%1\"", exe.display());

    // Nettoyage de l'ancienne version (entrée pour tous les fichiers)
    let _ = hkcu.delete_subkey_all(r"Software\Classes\*\shell\NativeConvert");

    for emplacement in emplacements() {
        let (cle, _) = hkcu.create_subkey(&emplacement)?;
        cle.set_value("Icon", &icone)?;

        let (cle_cmd, _) = hkcu.create_subkey(format!(r"{}\command", emplacement))?;
        cle_cmd.set_value("", &commande)?;
    }

    Ok(())
}

fn verifier_fichier_requis(chemin: &Path) -> io::Result<()> {
    if chemin.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Fichier requis introuvable : {}", chemin.display()),
        ))
    }
}

fn desinstaller() -> std::io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for emplacement in emplacements() {
        let _ = hkcu.delete_subkey_all(&emplacement);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.get(1).map(String::as_str) == Some("--install") {
        match installer() {
            Ok(()) => println!("NativeConvert ajouté au clic droit !"),
            Err(e) => {
                eprintln!("Installation impossible : {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    if args.get(1).map(String::as_str) == Some("--uninstall") {
        match desinstaller() {
            Ok(()) => println!("NativeConvert retiré du clic droit."),
            Err(e) => println!("Erreur : {}", e),
        }
        return;
    }

    println!("==========================");
    println!("       NativeConvert");
    println!("==========================");
    println!();

    match args.get(1) {
        Some(chemin) => {
            println!("Fichier sélectionné :");
            println!("{}", chemin);
        }
        None => {
            println!("Aucun fichier reçu.");
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
