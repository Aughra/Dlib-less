# Reprise — dlib-less

> À lire en premier à chaque session (règle de CLAUDE.md). Dernière mise à jour : 2026-11-06.

## Objectif

Apprendre en profondeur les concepts de dlib et de Dlib-Transformer-extensions en les réécrivant from scratch, en C++23, sur Mac Apple Silicon. Plan : [docs/cursus.md](../docs/cursus.md).

## Module en cours

Aucun. Prochaine étape : **P1, Terminal, git et compilation** (préparation, voir [docs/cursus.md](../docs/cursus.md)). Méthode : « mode prof » de CLAUDE.md et [apprentissage/README.md](../apprentissage/README.md).

## Fait

- 2026-11-06 : projet créé : CLAUDE.md (adapté de celui du fork Mac de Dlib-Transformer-extensions), README, cursus, notes de départ, journal, sommaire du blog, `.gitignore`.
- 2026-11-06 : dépôt GitHub `Aughra/Dlib-less` créé, premier commit poussé. Licence Boost (BSL-1.0) ajoutée.
- 2026-11-06 : préparation P1 à P6 ajoutée au cursus, « mode prof » dans CLAUDE.md, dossier `apprentissage/` (méthode, carnet, quiz).

## En cours

Rien.

## À faire, dans l'ordre proposé

1. **Préparation P1 à P6**, sur le M2, à partir de P1. Chaque exercice dans `apprentissage/exercices/`.
2. **Module 0** :
   - installer LLVM de Homebrew et vérifier quelles fonctions C++23 sont réellement disponibles (`std::expected`, `std::mdspan`, `std::print`) ; décider entre LLVM de Homebrew et AppleClang, et l'écrire dans CLAUDE.md ;
   - `CMakeLists.txt` + `CMakePresets.json` (debug, release, asan) ;
   - choisir doctest ou Catch2 ; un premier test qui passe ;
   - réécrire la section « Construire » de CLAUDE.md avec les vraies commandes.
3. **Module 1** : le tenseur.
4. À l'arrivée du Mac mini M5 Pro (fin novembre) : refaire le build complet dessus, relancer les tests, et noter les premières mesures des deux machines.

## Décisions

- Option C : tout écrire soi-même, aucune bibliothèque de ML dans le code (références de comparaison seulement).
- Mac Apple Silicon uniquement.
- C++23 de base, C++26 au cas par cas ; modules C++20 reportés.
- Licence : Boost Software License 1.0 (fichier `LICENSE`).
- L'humain débute en C++ : préparation P1 à P6 avant le module 0.
- **Tout en C++, pas de Python**, même comme brouillon.
- Mode prof par défaut : Claude n'écrit pas la solution des exercices sans demande explicite.

## Problèmes ouverts

- Choix doctest ou Catch2 à faire au module 0.

## Commandes pour reprendre

Aucune pour l'instant (pas encore de build).
