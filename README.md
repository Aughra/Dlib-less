# dlib-less

Réécrire **from scratch, en Rust et sur Mac Apple Silicon**, les concepts de [dlib](https://github.com/davisking/dlib) et de [Dlib-Transformer-extensions](https://github.com/Cydral/Dlib-Transformer-extensions) : tenseurs, couches, calcul de gradients, entraînement, Transformers, GGUF, quantification, LoRA… **sans dlib**.

Un framework qui se définit par ce qu'il n'a pas.

C'est un projet **d'apprentissage** : le code vise d'abord à être compris, ensuite à être rapide.

## Où lire quoi

| Document | Contenu |
|---|---|
| [docs/cursus.md](docs/cursus.md) | Le plan : préparation P1 à P6, puis modules 0 à 10 ; la boîte à outils ; l'état d'avancement |
| [apprentissage/](apprentissage/) | La méthode (je fais, on fait, tu fais), le carnet, les exercices et les quiz |
| [prof/](prof/) | L'atelier de Claude : exemples, essais et démonstrations des séances |
| [docs/notes-de-depart.md](docs/notes-de-depart.md) | Ce que sont dlib et l'extension, leurs équivalents, et pourquoi tout réécrire |
| [logs/reprise.md](logs/reprise.md) | Où en est le chantier, maintenant |
| [logs/journal-modifications.md](logs/journal-modifications.md) | L'historique des changements |
| [blog/](blog/) | Le carnet de bord, épisode par épisode |
| [CLAUDE.md](CLAUDE.md) | Les règles du projet |

## Plate-forme

macOS sur Apple Silicon (arm64) uniquement. Rust stable, édition 2024.

## Licence

[Boost Software License 1.0](LICENSE), comme dlib et Dlib-Transformer-extensions.
