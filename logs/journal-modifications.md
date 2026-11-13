# Journal des modifications

Le plus récent en haut. Une entrée par intervention, jamais réécrite.

## 2026-11-13 — Faire la séance 3 de P2 : `if`

- **Quoi** : fin de `p2-04-conversions` (2 tests verts) ; séance sur `if` / `else if` / `else` (un `if` renvoie une valeur) et aperçu de `match` ; exercice `p2-05-decisions` (fonction `categorie`, 3 tests aux bords 12/13 et 17/18). L'humain avait inversé `<` et `>` : trouvé grâce aux tests. Carnet écrit par Claude à la demande de l'humain.
- **Fichiers** : `apprentissage/exercices/p2-05-decisions/`, `apprentissage/exercices/p2-04-conversions/src/main.rs`, `apprentissage/carnet.md`, `logs/reprise.md`, `logs/journal-modifications.md`.
- **Pourquoi** : avancer le cursus P2 ; montrer que les valeurs aux bords révèlent les erreurs.
- **Étapes** : `cargo test` 3 réussis, `cargo clippy --all-targets` sans avertissement, `cargo fmt` appliqué sur `p2-05-decisions` ; commit et push.

## 2026-11-09 — Noter la séance 2 de P2 (types) et les espaces de travail

- **Quoi** : séance sur les types (`i32`, `u8`, `f64`, `bool`, `char`, conversion `as`, division entière, floats inexacts) ; exemple `prof/p2-types/`, exercice `p2-04-conversions` préparé dans `prof/` puis déposé (énoncé, 2 tests, code de départ). L'humain a commité son travail (commit `565fde1`, « suite 1 »), dont deux espaces de travail cargo (`apprentissage/exercices/Cargo.toml`, `prof/Cargo.toml`). Section « Construire » de CLAUDE.md et reprise mises à jour.
- **Fichiers** : `CLAUDE.md`, `logs/reprise.md`, `logs/journal-modifications.md`.
- **Pourquoi** : garder la doc vraie (les exercices sont désormais dans un espace de travail) et pouvoir reprendre l'exercice en cours.
- **Étapes** : `cargo metadata` charge les deux espaces de travail (6 exercices, 4 projets d'atelier) ; `p2-04-conversions` ne compile pas encore (travail en cours de l'humain) ; `cargo fmt --check` propre sur `p2-03-types`, `prof/p2-types`, `prof/p2-04-conversions` ; commit et push.

## 2026-11-08 — Faire la séance 1 de P2 : les variables

- **Quoi** : séance sur `let`, `mut` et `+=`. Exemple dans `prof/p2-variables/` (avec une exception clippy assumée pour la démonstration), exercice `p2-02-tirelire` préparé et vérifié dans `prof/p2-02-tirelire/`, puis déposé (énoncé et trois tests) dans `apprentissage/exercices/p2-02-tirelire/`. Code de l'humain : `p2-01-variables` et `p2-02-tirelire`, 3 tests passent. P2 passe à « en cours ».
- **Fichiers** : `apprentissage/exercices/p2-01-variables/`, `apprentissage/exercices/p2-02-tirelire/`, `prof/p2-variables/`, `prof/p2-02-tirelire/`, `docs/cursus.md`, `logs/reprise.md`, `logs/journal-modifications.md`.
- **Pourquoi** : première notion de P2 (Rust de base).
- **Étapes** : `cargo test` (3 réussis), `cargo fmt --check` et `cargo clippy --all-targets` propres sur les quatre projets, commit et push.

## 2026-11-08 — Valider P1 et écrire l'épisode 1 du blog

- **Quoi** : deuxième séance (git déjà connu, exercice direct) : l'humain a fait seul ses commits `1fc5535` (« start », avec une copie involontaire de `prof/p1-git/verifier.sh` à la racine), `fe63136` (retrait de la copie) et `896183c` (lexique de git corrigé). Vérificateur `prof/p1-git/verifier.sh` (écrit par Claude, commit `5db4adb`) tout vert. P1 passe à ✅ ; épisode 1 du blog et sommaire ; reprise mise à jour (prochaine étape : P2).
- **Fichiers** : `blog/01-premiers-pas.md`, `blog/README.md`, `docs/cursus.md`, `logs/reprise.md`, `logs/journal-modifications.md`.
- **Pourquoi** : fin de l'étape P1 ; règle « un épisode par étape ». Décision notée : Claude reste rigoureux sur le fond mais ne fait qu'une remarque sur la forme (l'humain l'a trouvé trop pointilleux).
- **Étapes** : vérificateur relancé (4 ✅), contrôle du blog (titre, sommaire, ni e-mail ni chemin personnel), commit et push.

## 2026-11-07 — Ne plus faire écrire de tableaux à l'humain

- **Quoi** : règle ajoutée au mode prof : ce que l'humain remplit (carnet, quiz, réponses) se fait en listes ; Claude garde les tableaux dans ses explications. Lexique du carnet complété par l'humain (cargo, macro, warning, `\n`).
- **Fichiers** : `CLAUDE.md`, `apprentissage/carnet.md`, `logs/reprise.md`, `logs/journal-modifications.md`.
- **Pourquoi** : les tableaux Markdown sont pénibles à écrire à la main ; ils restent utiles à lire.
- **Étapes** : rédaction, commit et push. Pas de code.

## 2026-11-07 — Faire la première séance : P1, cargo

- **Quoi** : séance sur cargo (le Terminal était déjà connu). Exercices `p1-01-bonjour` (exploration des erreurs) et `p1-02-carte` (énoncé et test fournis par Claude, code de l'humain, test réussi). Exemple et vérification du test dans `prof/p1-cargo/`. Convention de nommage des exercices passée en minuscules (`pN-xx-nom`), après l'avertissement de cargo. Lexique du carnet passé d'un tableau à une liste. P1 passe à « en cours » dans le cursus.
- **Fichiers** : `apprentissage/exercices/p1-01-bonjour/`, `apprentissage/exercices/p1-02-carte/`, `apprentissage/carnet.md`, `apprentissage/README.md`, `prof/p1-cargo/`, `docs/cursus.md`, `logs/reprise.md`, `logs/journal-modifications.md`.
- **Pourquoi** : premier pas de la préparation ; le nommage en majuscules venait d'une consigne de Claude et provoquait un avertissement ; les tableaux Markdown sont pénibles à écrire à la main.
- **Étapes** : `cargo test` réussi sur `p1-02-carte`, `cargo fmt --check` et `cargo clippy --all-targets` propres sur les deux exercices et l'atelier, commit et push.

## 2026-11-07 — Créer l'atelier du prof

- **Quoi** : nouveau dossier `prof/`, où Claude range tout ce qu'il fabrique (exemples du « je fais », essais, démonstrations), et nulle part ailleurs sur la machine. Règle ajoutée à CLAUDE.md, ligne ajoutée au README.
- **Fichiers** : `prof/README.md`, `CLAUDE.md`, `README.md`, `logs/reprise.md`, `logs/journal-modifications.md`.
- **Pourquoi** : demande de l'humain, qui ne veut pas de fichiers éparpillés hors du projet. Le premier exemple de cargo avait été fait dans un dossier temporaire du système, puis supprimé.
- **Étapes** : rédaction, commit et push. Pas de code, donc pas de build ni de test.

## 2026-11-07 — Donner un épisode de blog à chaque étape

- **Quoi** : la règle du blog passe de « un épisode par module ou grande étape » à « un épisode par étape du cursus (P1 à P6, modules 0 à 10, 3b compris), plus les moments marquants ». Le blog garde aussi les moments vécus (erreurs, blocages, déclics, phrases du carnet). L'écriture de l'épisode entre dans l'étape 5 du rituel de séance.
- **Fichiers** : `CLAUDE.md`, `blog/README.md`, `logs/reprise.md`, `logs/journal-modifications.md`.
- **Pourquoi** : choix de l'humain, qui veut relire le chemin plus tard « comme une madeleine de Proust ». Corrige aussi une contradiction : le sommaire réservait le premier épisode au module 0, alors que P6 est déjà un premier entraînement qui apprend.
- **Étapes** : réécriture des règles, commit et push. Pas de code, donc pas de build ni de test.

## 2026-11-07 — Passer de C++ à Rust

- **Quoi** : le langage du projet devient Rust (stable, édition 2024). CLAUDE.md (mode prof, projet, langage, dépendances, construire, git), cursus réécrit (préparation P1 à P6 pour Rust, boîte à outils, colonne « Rust pratiqué », fil rouge statique/dynamique, conseils, ressources), README, `apprentissage/README.md`, `.gitignore` (`target/`), reprise ; addendum daté dans les notes de départ (comparaison C, C++, Rust).
- **Fichiers** : `CLAUDE.md`, `README.md`, `.gitignore`, `docs/cursus.md`, `docs/notes-de-depart.md`, `apprentissage/README.md`, `logs/reprise.md`, `logs/journal-modifications.md`.
- **Pourquoi** : choix de l'humain après comparaison : un compilateur qui empêche les erreurs de mémoire et un outillage simple (cargo) conviennent mieux à un débutant, au prix d'un écart avec le code C++ de dlib.
- **Étapes** : réécriture des documents, vérification qu'aucun document vivant ne parle encore de C++ pour le projet, commit et push. Pas de code, donc pas de build ni de test.

## 2026-11-06 — Ajouter la préparation et le mode prof

- **Quoi** : préparation P1 à P6 (terminal et git, C++ de base, mémoire, classes et templates, maths, un neurone à la main) ajoutée avant le module 0 ; section « Mode prof » et rituel de séance dans CLAUDE.md ; dossier `apprentissage/` (méthode, carnet avec lexique, quiz) ; ressources pour débutant dans le cursus ; règle « tout en C++, pas de Python ».
- **Fichiers** : `CLAUDE.md`, `README.md`, `docs/cursus.md`, `apprentissage/README.md`, `apprentissage/carnet.md`, `apprentissage/quiz/README.md`, `logs/reprise.md`, `logs/journal-modifications.md`.
- **Pourquoi** : l'humain débute en C++ ; le cursus supposait des bases qu'il n'a pas encore. Le mode prof évite que Claude écrive tout à sa place.
- **Étapes** : rédaction, commit et push. Pas de code, donc pas de build ni de test.

## 2026-11-06 — Adopter la licence Boost

- **Quoi** : ajout du fichier `LICENSE` (Boost Software License 1.0).
- **Fichiers** : `LICENSE`, `README.md`, `CLAUDE.md`, `logs/reprise.md`, `logs/journal-modifications.md`.
- **Pourquoi** : même licence que dlib et Dlib-Transformer-extensions, ce qui simplifie les emprunts de code ; choix validé par l'humain.
- **Étapes** : ajout du fichier, mise à jour des documents qui la disaient « à confirmer », commit et push.

## 2026-11-06 — Créer le projet

- **Quoi** : création du squelette documentaire de dlib-less.
- **Fichiers** : `CLAUDE.md`, `README.md`, `.gitignore`, `docs/cursus.md`, `docs/notes-de-depart.md`, `logs/reprise.md`, `logs/journal-modifications.md`, `blog/README.md`.
- **Pourquoi** : poser les règles (CLAUDE.md adapté de celui du fork Mac de Dlib-Transformer-extensions), garder la trace de la discussion de départ, et fixer le plan en modules avant d'écrire du code.
- **Étapes** : rédaction, premier commit. Pas de code, donc pas de build ni de test.
