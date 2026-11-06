# Journal des modifications

Le plus récent en haut. Une entrée par intervention, jamais réécrite.

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
