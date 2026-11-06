# dlib-less

Réécrire **from scratch, en C++ moderne et sur Mac Apple Silicon**, les concepts de [dlib](https://github.com/davisking/dlib) et de [Dlib-Transformer-extensions](https://github.com/Cydral/Dlib-Transformer-extensions) : tenseurs, couches, calcul de gradients, entraînement, Transformers, GGUF, quantification, LoRA… **sans dlib**.

Un framework qui se définit par ce qu'il n'a pas.

C'est un projet **d'apprentissage** : le code vise d'abord à être compris, ensuite à être rapide.

## Où lire quoi

| Document | Contenu |
|---|---|
| [docs/cursus.md](docs/cursus.md) | Le plan en modules, la boîte à outils, l'état d'avancement |
| [docs/notes-de-depart.md](docs/notes-de-depart.md) | Ce que sont dlib et l'extension, leurs équivalents, et pourquoi tout réécrire |
| [logs/reprise.md](logs/reprise.md) | Où en est le chantier, maintenant |
| [logs/journal-modifications.md](logs/journal-modifications.md) | L'historique des changements |
| [blog/](blog/) | Le carnet de bord, épisode par épisode |
| [CLAUDE.md](CLAUDE.md) | Les règles du projet |

## Plate-forme

macOS sur Apple Silicon (arm64) uniquement. C++23.

## Licence

À choisir (Boost Software License 1.0 proposée, comme dlib et l'extension).
