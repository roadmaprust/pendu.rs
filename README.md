# 🎮 Jeu du Pendu en Rust

Bienvenue dans le **Jeu du Pendu**, un projet simple et amusant écrit en Rust !
🦀  
Affrontez l'ordinateur ou un ami pour deviner le mot secret lettre par lettre, avec un nombre limité d'erreurs autorisées.

---

## 🚀 Fonctionnalités

- 🧠 **Mode Solo** : un mot est choisi aléatoirement.
- 🧑‍🤝‍🧑 **Mode 2 Joueurs** : un joueur entre le mot secret (masqué).
- 🎯 **Niveaux de difficulté** : Facile (10 erreurs), Moyen (6), Difficile (3).
- 🧩 Affichage progressif du mot.
- 📉 Gestion des erreurs restantes.
- 🏆 Système de score basé sur les erreurs restantes.
- 💾 Sauvegarde des scores dans un fichier `scores.txt`.

---

## 🛠️ Installation

1. **Clone le dépôt :**
   ```bash
   git clone https://github.com/roadmaprust/pendu.rs
   cd pendu-rust
    ```

2. **Compile et lance le jeu :**

   ```bash
   cargo run
   ```

---

## 🧪 Dépendances

Le projet utilise les crates suivantes :

* [`rand`](https://docs.rs/rand/) – Pour choisir un mot aléatoire.
* [`rpassword`](https://docs.rs/rpassword/) – Pour masquer le mot secret dans le mode 2 joueurs.

```toml
[dependencies]
rpassword = "7.4.0"
rand = "0.9.1"
```

---

## 📁 Structure du projet

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── scores.txt
├── src
│   ├── game.rs
│   ├── mod.rs
│   └── pendu.rs
└── tests
    └── tests.rs
```

---

## ✨ Exemple de jeu

```
📌 Bienvenue dans le jeu du pendu !
Choisis la difficulté : 1 - Facile (10 erreurs), 2 - Moyen (6 erreurs), 3 - Difficile (3 erreurs)
👉 Choisis une option (1 ou 2) : 1️⃣ Mode Solo (mot aléatoire)
Mot actuel : _ _ _ _
Erreurs restantes : 10
Entre un caractère :
> e
❌ Mauvaise lettre !
```

---

## 💡 À faire (idées d'amélioration)

* 🧠 Ajouter un dictionnaire plus vaste.
* 🌐 Interface graphique ou en ligne de commande enrichie (avec `crossterm` par exemple).
* 🧾 Affichage des meilleurs scores triés.
* 🌍 Prise en charge de plusieurs langues.

---

## 📜 Licence

Ce projet est sous licence **GPL-v3**.

---

Bon jeu ! 🎉
