# Tiny Server

Un serveur HTTP minimal écrit en Rust utilisant uniquement la bibliothèque standard.

## Description

Ce projet implémente un serveur HTTP simple qui démontre les concepts de base du réseau en Rust sans dépendances externes. Il gère les requêtes GET et POST avec des pages HTML stylisées.

## Fonctionnalités

- **Serveur HTTP léger** : Utilise uniquement `std::net` et `std::fs` de la bibliothèque standard
- **Route GET `/`** : Page d'accueil servie depuis un fichier HTML externe
- **Route POST `/submit`** : Traitement des données de formulaire
- **Gestion d'erreurs 404** : Page d'erreur pour les routes non définies
- **Décodage URL** : Parsing des données URL-encoded
- **Interface stylisée** : Pages HTML avec CSS intégré
- **Fichiers statiques** : Lecture du fichier index.html depuis le disque

## Prérequis

- Rust 1.56 ou supérieur
- Cargo (inclus avec Rust)

## Installation

1. Clonez le dépôt :
```bash
git clone <votre-repo>
cd tiny_server
```

2. Compilez le projet :
```bash
cargo build --release
```

## Utilisation

### Démarrer le serveur

```bash
cargo run
```

Le serveur démarre sur `http://127.0.0.1:8080`

**Important** : Le fichier `index.html` doit être présent dans le répertoire de travail (à la racine du projet) pour que le serveur fonctionne correctement.

### Accéder au serveur

Ouvrez votre navigateur et accédez à :
- **Page d'accueil** : http://127.0.0.1:8080
- **Autres routes** : Retourneront une page 404

### Tester le formulaire POST

1. Accédez à la page d'accueil
2. Remplissez le formulaire avec un nom et un message
3. Cliquez sur "Envoyer"
4. Visualisez les données reçues sur la page de confirmation

## Routes disponibles

| Méthode | Route      | Description                                    |
|---------|------------|------------------------------------------------|
| GET     | `/`        | Page d'accueil avec formulaire                 |
| POST    | `/submit`  | Traite les données du formulaire               |
| *       | `*`        | Page 404 pour toutes les autres routes        |

## Structure du code

```
tiny_server/
├── Cargo.toml          # Configuration du projet
├── README.md           # Documentation
├── index.html          # Page d'accueil HTML
└── src/
    └── main.rs         # Code source principal
```

### Fichiers importants

#### main.rs
- **`main()`** : Configure le TcpListener et accepte les connexions
- **`handle_connection()`** : Traite chaque requête HTTP et route vers les handlers appropriés
- **`urlencoded_decode()`** : Décode les données URL-encoded des formulaires

#### index.html
- Page d'accueil avec formulaire de test
- Contient tout le HTML et CSS pour l'interface utilisateur
- Chargé dynamiquement par le serveur à chaque requête GET /

## Exemples de requêtes

### Requête GET

```bash
curl http://127.0.0.1:8080/
```

### Requête POST

```bash
curl -X POST http://127.0.0.1:8080/submit \
  -d "name=Jean&message=Bonjour"
```

## Limitations

Ce serveur est conçu à des fins éducatives et présente plusieurs limitations :

- Pas de support HTTPS
- Gestion basique des erreurs
- Pas de threading (traitement séquentiel des requêtes)
- Buffer de lecture limité à 1024 octets
- Pas de gestion de sessions ou de cookies
- Pas de protection CSRF

## Améliorations possibles

- [ ] Ajouter le support multi-thread avec `std::thread`
- [ ] Implémenter un pool de threads
- [ ] Ajouter plus de méthodes HTTP (PUT, DELETE, etc.)
- [ ] Servir plusieurs types de fichiers statiques (CSS, JS, images)
- [ ] Ajouter le logging des requêtes
- [ ] Améliorer le parsing HTTP
- [ ] Ajouter des tests unitaires
- [ ] Détection automatique du MIME type
- [ ] Cache pour les fichiers statiques

## Licence

Ce projet est libre d'utilisation à des fins éducatives.

## Auteur

Créé avec Rust et la bibliothèque standard.
