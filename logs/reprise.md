# Reprise — dlib-less

> À lire en premier à chaque session (règle de CLAUDE.md). Dernière mise à jour : 2026-11-07.

## Objectif

Apprendre en profondeur les concepts de dlib et de Dlib-Transformer-extensions en les réécrivant from scratch, **en Rust**, sur Mac Apple Silicon. Plan : [docs/cursus.md](../docs/cursus.md).

## Module en cours

Aucun. Prochaine étape : **P1, Terminal, git et cargo** (préparation, voir [docs/cursus.md](../docs/cursus.md)). Méthode : « mode prof » de CLAUDE.md et [apprentissage/README.md](../apprentissage/README.md).

## Fait

- 2026-11-06 : projet créé : CLAUDE.md (adapté de celui du fork Mac de Dlib-Transformer-extensions), README, cursus, notes de départ, journal, sommaire du blog, `.gitignore`.
- 2026-11-06 : dépôt GitHub `Aughra/Dlib-less` créé, premier commit poussé. Licence Boost (BSL-1.0) ajoutée.
- 2026-11-06 : préparation P1 à P6 ajoutée au cursus, « mode prof » dans CLAUDE.md, dossier `apprentissage/` (méthode, carnet, quiz).
- 2026-11-07 : **passage de C++ à Rust** : CLAUDE.md, cursus (préparation, boîte à outils, modules, ressources), README, `apprentissage/`, `.gitignore` adaptés ; addendum dans les notes de départ.
- 2026-11-07 : règle du blog élargie : un épisode par étape du cursus, préparation comprise.
- 2026-11-07 : dossier `prof/` créé, l'atelier de Claude.

## En cours

Rien.

## À faire, dans l'ordre proposé

1. **Préparation P1 à P6**, sur le M2, à partir de P1. Installer Rust (`rustup`), VS Code avec rust-analyzer et CodeLLDB. Chaque exercice dans `apprentissage/exercices/`, un projet cargo chacun.
2. **Module 0** :
   - espace de travail cargo à la racine, profils debug et release ;
   - un premier test qui passe (`cargo test`), `cargo clippy` et `cargo fmt --check` propres ;
   - réécrire la section « Construire » de CLAUDE.md avec les vraies commandes.
3. **Module 1** : le tenseur.
4. À l'arrivée du Mac mini M5 Pro (fin novembre) : refaire le build complet dessus, relancer les tests, et noter les premières mesures des deux machines.

## Décisions

- Option C : tout écrire soi-même, aucune bibliothèque de ML dans le code (références de comparaison seulement).
- Mac Apple Silicon uniquement.
- **Rust stable, édition 2024** (décision du 2026-11-07, à la place du C++23). `unsafe` rare, enfermé, commenté (`// SAFETY:`).
- Licence : Boost Software License 1.0 (fichier `LICENSE`).
- L'humain débute : préparation P1 à P6 avant le module 0.
- **Tout en Rust, pas de Python**, même comme brouillon. Le C++ de dlib et de l'extension se lit, il ne s'écrit pas.
- Mode prof par défaut : Claude n'écrit pas la solution des exercices sans demande explicite.
- **Blog : un épisode par étape du cursus** (P1 à P6 et chaque module), plus les moments marquants (décision du 2026-11-07). Le blog est aussi la mémoire du chemin, à relire « comme une madeleine de Proust ».
- **À chaque fin d'étape : écrire son épisode** dans le même commit.
- **Atelier du prof** : tout ce que Claude fabrique va dans `prof/`, suivi par git (décision du 2026-11-07).

## Problèmes ouverts

- Metal depuis Rust : `objc2-metal` prévue, à valider au module 8.
- Projection de fichiers en mémoire (module 7) : crate à proposer, ou appel direct à `mmap` en FFI.
- Demi-précision (`f16`, `bf16`) : vérifier si le type natif de Rust est stable au moment du besoin ; sinon proposer la crate `half`.

## Commandes pour reprendre

Aucune pour l'instant (pas encore de code).
