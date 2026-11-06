# Le cursus dlib-less

> Document vivant : l'état de chaque module est tenu à jour (règle de CLAUDE.md). Le contexte et les choix de départ sont dans [notes-de-depart.md](notes-de-depart.md).

Le principe : un **cursus-projet**. Chaque module apprend un ensemble de concepts de dlib ou de Dlib-Transformer-extensions, et produit du code qui fonctionne **et qu'un test vérifie**. On ne passe au module suivant que quand le précédent est validé.

**Tout se fait en C++**, y compris les premiers pas et les exercices de maths : pas de Python, même comme brouillon. Les concepts de ML s'abordent d'abord **sur papier**, puis directement en C++.

Le cursus commence par une **préparation** (P1 à P6) : on part de zéro en C++. La méthode d'apprentissage (séances, exercices, carnet, quiz) est dans [../apprentissage/README.md](../apprentissage/README.md).

---

## La préparation : du zéro au module 0

État : ⬜ à faire · 🔄 en cours · ✅ validé

| # | État | Étape | Ce qu'on apprend | Image | Validé quand… |
|---|---|---|---|---|---|
| P1 | ⬜ | **Terminal, git, compiler** | Se déplacer dans le Terminal, `git add` / `commit` / `push`, compiler un fichier avec `clang++`, lire un message d'erreur | Le compilateur est un traducteur : ton texte devient des instructions pour le processeur | On compile et lance un « bonjour » soi-même, et on explique chaque mot de la commande |
| P2 | ⬜ | **C++ de base** | Variables et types, `if`, boucles, fonctions, `std::vector`, `std::string`, `std::print` | Les briques de Lego | Exercices : moyenne d'une liste, inverser un texte, compter les mots |
| P3 | ⬜ | **La mémoire** | Pile et tas, références, pointeurs, durée de vie, RAII, `std::unique_ptr`, `std::span` | Une adresse sur une boîte aux lettres, et qui rend la clé en partant | Exercices sans fuite ni erreur sous AddressSanitizer |
| P4 | ⬜ | **Classes et templates** | `struct` et `class`, constructeurs, opérateurs, templates, concepts | Un moule à gâteaux qui marche pour toutes les pâtes | Une petite classe `Matrix` générique, avec tests |
| P5 | ⬜ | **Maths minimales, en C++** | Vecteurs, produit scalaire, matrices, produit de matrices, dérivée, dérivée en chaîne | Une recette appliquée à toute une rangée de cases d'un coup | Chaque notion d'abord à la main sur papier, puis codée et testée (dont une dérivée vérifiée par différences finies) |
| P6 | ⬜ | **Un neurone à la main** | Un neurone, puis un tout petit réseau : calcul avant, erreur, correction des poids | Avant de construire l'usine, fabriquer une pièce à la main | Le réseau apprend la fonction XOR, en C++, sans aucune bibliothèque |

Puis : **module 0**.

---

## La boîte à outils sur Mac

| Outil | Rôle | Image |
|---|---|---|
| **LLVM de Homebrew** (`brew install llvm`) | Compilateur à jour pour le C++23 et une partie du C++26 | Un dictionnaire de l'année, plutôt que celui de l'école primaire (AppleClang suit le C++ récent avec retard) |
| **CMake + Ninja** | Décrire et lancer la construction | La recette (CMake) et le cuisinier rapide (Ninja) |
| **doctest** ou **Catch2** | Tests | Le contrôle technique de chaque pièce |
| **AddressSanitizer, UBSan** | Attraper les erreurs mémoire et les comportements indéfinis | Un détecteur de fumée dans chaque pièce |
| **Instruments** (Xcode) | Mesurer où le temps passe | Le chronomètre et la caméra au ralenti |
| **Accelerate** | Bibliothèque de calcul d'Apple ; `cblas_sgemm` y est très rapide | L'étalon : la voiture du champion à qui l'on compare la nôtre |
| **Intrinsics NEON** | Calcul vectorisé (SIMD) sur ARM, à la main | Une rangée de caisses qui encaissent en même temps |
| **Metal via metal-cpp** | Programmer le GPU en C++ | Embaucher une armée de petits calculateurs |

**Référence à lire en parallèle : [ml-explore/mlx](https://github.com/ml-explore/mlx)**. Cœur en C++, conçu par Apple pour Apple Silicon, lisible. Le meilleur professeur silencieux pour les modules 1, 2 et 8.

### Les deux machines

| | Mac M2 (actuel) | Mac mini M5 Pro (fin novembre 2026) |
|---|---|---|
| CPU | 8 cœurs | 18 cœurs |
| GPU | (selon modèle) | 20 cœurs |
| Mémoire unifiée | 24 Go | **64 Go** |
| Usage dans le cursus | Modules 0 à 6 sans souci ; module 7 avec de petits modèles | Tout le cursus, dont le fine-tuning LoRA du module 9 |

Repères pour la taille des modèles avec **notre** moteur (moins optimisé que llama.cpp) :
- M2, 24 Go : modèles de 0,5 à 3 milliards de paramètres en inférence ; entraînement de petits modèles maison.
- M5 Pro, 64 Go : inférence jusqu'à 7-8 milliards confortablement (plus en quantifié) ; LoRA sur 1 à 3 milliards en bf16.

Ce sont des ordres de grandeur, à remplacer par des mesures réelles.

À explorer sur le M5 Pro : Apple annonce des accélérateurs neuronaux dans les cœurs GPU des puces M5, accessibles par les API tenseur de Metal 4. **À vérifier** sur la machine avant d'en dépendre ; candidat pour un module 8b.

### Le C++ pratiqué

Base **C++23** : `std::expected` (erreurs sans exceptions), `std::mdspan` (vues multidimensionnelles), `std::span`, `std::print`, *deducing this*, concepts, ranges, `constexpr`. Le **C++26** (dont la réflexion) seulement quand le compilateur le supporte vraiment, isolé et signalé. Les modules C++20 (`import std;`) sont reportés : en-têtes classiques au départ.

---

## Les modules

État : ⬜ à faire · 🔄 en cours · ✅ validé

| # | État | Module | Concepts dlib / extension | C++ pratiqué | Livrable | Critère de validation |
|---|---|---|---|---|---|---|
| 0 | ⬜ | **Fondations** | Organisation d'une bibliothèque | CMake, presets, sanitizers, tests | Projet qui compile, tests qui passent, config ASan | `ctest` à 100 % en Debug, Release et ASan |
| 1 | ⬜ | **Tenseur** | `tensor`, `resizable_tensor`, `alias_tensor` | `mdspan`, `span`, concepts, alignement mémoire | Tenseur avec formes, *strides* et vues | Tests de formes, vues, transpositions sans copie |
| 2 | ⬜ | **Kernels CPU** | `cpu_dlib`, abstraction de backend | NEON, `std::jthread`, pool de threads | Multiplication de matrices naïve → par tuiles → NEON → multithread | Résultats identiques à Accelerate (tolérance) ; GFLOPS mesurés et comparés |
| 3 | ⬜ | **Gradients statiques** | Pile de couches compilée par templates (style dlib : `add_layer`, `fc`, `relu`…) | Templates variadiques, concepts, `constexpr` | Réseau statique avec `forward`/`backward` | Gradients vérifiés par différences finies |
| 3b | ⬜ | **Gradients dynamiques** | Moteur dynamique de l'extension | Graphe à l'exécution, *type erasure* | Graphe enregistré puis parcouru à l'envers | **Mêmes résultats que le module 3** sur le même réseau |
| 4 | ⬜ | **Entraînement classique** | Couches de perte, `adam`, `dnn_trainer`, sérialisation | `std::expected`, RAII | Classifieur MNIST de bout en bout | Précision de test > 97 % ; sauvegarde puis rechargement identiques |
| 5 | ⬜ | **Transformer** | Couches Transformer de l'extension | Templates de couches | **Mini-GPT au niveau caractère** entraîné sur Molière | La perte baisse ; le texte généré ressemble (de loin) à du Molière |
| 6 | ⬜ | **Tokenizer BPE** | Tokenizer entraînable de l'extension | Ranges, `string_view` | BPE : entraînement, encodage, décodage | `decode(encode(x)) == x` ; mêmes jetons qu'un tokenizer de référence sur un vocabulaire GGUF |
| 7 | ⬜ | **GGUF et quantification** | Import GGUF, Q8_0, Q4_K, cache KV, échantillonnage | `mmap`, parsing binaire défensif, `std::expected` | Inférence d'un vrai petit modèle (Qwen 0.5B, SmolLM…) | Logits à moins d'une tolérance fixée de ceux de llama.cpp, même prompt |
| 8 | ⬜ | **Backend Metal** | Routage CPU/GPU par opération | metal-cpp, shaders de calcul, mémoire unifiée | Multiplication de matrices, softmax, attention sur GPU | Mêmes résultats que le CPU ; gain de vitesse mesuré |
| 9 | ⬜ | **Fine-tuning** | SFT avec masquage, LoRA, DoRA, *gradient checkpointing*, précision mixte | — | Fine-tuning LoRA du modèle du module 7 | La perte baisse sur le jeu d'entraînement ; le comportement change sur des prompts témoins |
| 10 | ⬜ | **Bonus** | Distillation, élagage, embeddings et recherche, compression par LLM, vision, serveur compatible OpenAI | Coroutines (serveur) | Au choix | À définir au moment venu |

### Le fil rouge : statique contre dynamique

Les modules **3 et 3b** reproduisent la **dualité centrale** de Dlib-Transformer-extensions :
- un réseau **statique**, dont la forme est connue à la compilation, comme dans dlib : le compilateur voit tout et optimise (image : un meuble en kit déjà assemblé à l'usine) ;
- un moteur **dynamique**, dont le graphe se construit à l'exécution : plus souple, comme PyTorch (image : des briques de Lego qu'on assemble au moment de jouer).

Les deux doivent donner le **même résultat**. C'est ce test qui fait comprendre les deux approches.

### Le moment de vérité

Le **module 5** : voir sa propre implémentation, sans aucune bibliothèque de ML, apprendre à écrire du Molière approximatif. Le Molière sera mauvais, mais ce sera le nôtre.

---

## Conseils

- **Ne pas sauter les modules 1 à 4** pour courir vers les LLM. Tout ce qui est difficile ensuite (gradients faux, *strides* mal gérés, instabilité numérique) vient de là.
- **Accelerate d'abord comme étalon, Metal ensuite.** Sur puce M, Accelerate passe par les unités matricielles du CPU : il sera très difficile à battre avec du NEON maison. C'est une leçon d'humilité, et une bonne mesure.
- **Vérifier chaque gradient par différences finies.** Sans ça, on entraîne des bugs.
- **Parser le GGUF de façon défensive** : chaque taille et chaque décalage lu dans le fichier est une donnée non fiable.
- Les vrais pièges, dans l'ordre où ils arrivent : la rétropropagation de l'attention (mémoire des activations), la précision mixte (bf16/fp16), les performances des kernels quantifiés. C'est là que les semaines disparaissent.
- Compter **plusieurs mois** à rythme raisonnable : c'est un cursus de master en ML doublé de C++ avancé.

## Lectures de référence

### Pour la préparation

| Pour | Ressource |
|---|---|
| Apprendre le C++ pas à pas | **[learncpp.com](https://www.learncpp.com)** : le meilleur cours gratuit, progressif et moderne (en anglais ; la traduction du navigateur marche bien) |
| Voir les maths | **3Blue1Brown** sur YouTube : *Essence of Linear Algebra* et *Neural Networks* (sous-titres français) |
| Comprendre les réseaux de neurones | Andrej Karpathy, *Neural Networks: Zero to Hero* (YouTube) : on regarde, on recode en C++ |
| Petites expériences C++ dans le navigateur | **[godbolt.org](https://godbolt.org)** : on tape du C++, on voit ce que le compilateur en fait |
| Chercher une fonction de la bibliothèque standard | [cppreference.com](https://en.cppreference.com) |

### Pour les modules

| Pour | Lire |
|---|---|
| Concepts de dlib (couches, tenseurs, entraînement) | Le code de [davisking/dlib](https://github.com/davisking/dlib), dossier `dlib/dnn/` |
| Transformers dans dlib | [Cydral/Dlib-Transformer-extensions](https://github.com/Cydral/Dlib-Transformer-extensions) |
| Clarté d'un LLM en C | [karpathy/llama2.c](https://github.com/karpathy/llama2.c), [karpathy/llm.c](https://github.com/karpathy/llm.c) |
| Calcul de gradients minimal | La vidéo *micrograd* d'Andrej Karpathy (série *Neural Networks: Zero to Hero*) : on **regarde** l'explication au tableau, on **recode en C++** |
| Tenseurs et Metal sur Apple Silicon | [ml-explore/mlx](https://github.com/ml-explore/mlx) |
| GGUF et quantification | [ggml-org/ggml](https://github.com/ggml-org/ggml), [ggml-org/llama.cpp](https://github.com/ggml-org/llama.cpp) |
| Tokenizer BPE | [karpathy/minbpe](https://github.com/karpathy/minbpe) |
