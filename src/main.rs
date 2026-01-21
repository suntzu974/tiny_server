use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Serveur démarré sur http://127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn urlencoded_decode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        match c {
            '+' => result.push(' '),
            '%' => {
                if let (Some(h1), Some(h2)) = (chars.next(), chars.next()) {
                    if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                        result.push(byte as char);
                    }
                }
            }
            _ => result.push(c),
        }
    }

    result
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    // Vérifier si c'est une requête GET pour la route "/"
    if request.starts_with("GET / ") {
        // Lire le fichier index.html
        let html_content = match fs::read_to_string("index.html") {
            Ok(content) => content,
            Err(_) => {
                // Fichier de secours si index.html n'est pas trouvé
                String::from(r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <title>Erreur</title>
</head>
<body>
    <h1>Erreur</h1>
    <p>Le fichier index.html est introuvable.</p>
</body>
</html>"#)
            }
        };

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            html_content.len(),
            html_content
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if request.starts_with("POST /submit ") {
        // Extraire les données du formulaire
        let body_start = request.find("\r\n\r\n").unwrap_or(0) + 4;
        let body = &request[body_start..];

        // Parser les données (format: name=value&message=value)
        let mut name = String::new();
        let mut message = String::new();

        for param in body.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                let decoded_value = urlencoded_decode(value);
                match key {
                    "name" => name = decoded_value,
                    "message" => message = decoded_value,
                    _ => {}
                }
            }
        }

        let html_content = format!(r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Données reçues</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background-color: #f5f5f5;
        }}
        .container {{
            background-color: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        h1 {{
            color: #4CAF50;
        }}
        .data {{
            background-color: #f9f9f9;
            padding: 15px;
            border-left: 4px solid #4CAF50;
            margin: 20px 0;
        }}
        a {{
            color: #4CAF50;
            text-decoration: none;
        }}
        a:hover {{
            text-decoration: underline;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Données reçues avec succès!</h1>
        <div class="data">
            <p><strong>Nom:</strong> {}</p>
            <p><strong>Message:</strong> {}</p>
        </div>
        <p><a href="/">← Retour à l'accueil</a></p>
    </div>
</body>
</html>"#, name, message);

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            html_content.len(),
            html_content
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // Page 404 pour les autres routes
        let html_content = r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <title>404 - Page non trouvée</title>
</head>
<body>
    <h1>404 - Page non trouvée</h1>
    <p>La page que vous recherchez n'existe pas.</p>
</body>
</html>"#;

        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            html_content.len(),
            html_content
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
