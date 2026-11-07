# Journal des modifications

Le plus récent en haut. Une entrée par intervention, jamais réécrite.

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
