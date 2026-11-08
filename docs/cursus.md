# Le cursus dlib-less

> Document vivant : l'état de chaque module est tenu à jour (règle de CLAUDE.md). Le contexte et les choix de départ sont dans [notes-de-depart.md](notes-de-depart.md).

Le principe : un **cursus-projet**. Chaque module apprend un ensemble de concepts de dlib ou de Dlib-Transformer-extensions, et produit du code qui fonctionne **et qu'un test vérifie**. On ne passe au module suivant que quand le précédent est validé.

**Tout se fait en Rust**, y compris les premiers pas et les exercices de maths : pas de Python, même comme brouillon. Les concepts de ML s'abordent d'abord **sur papier**, puis directement en Rust. dlib et l'extension sont écrits en C++ : on **lit** leur code pour comprendre, on **traduit** les idées en Rust.

Le cursus commence par une **préparation** (P1 à P6) : on part de zéro en Rust. La méthode d'apprentissage (séances, exercices, carnet, quiz) est dans [../apprentissage/README.md](../apprentissage/README.md).

---

## La préparation : du zéro au module 0

État : ⬜ à faire · 🔄 en cours · ✅ validé

| # | État | Étape | Ce qu'on apprend | Image | Validé quand… |
|---|---|---|---|---|---|
| P1 | ✅ | **Terminal, git, cargo** | Se déplacer dans le Terminal, `git add` / `commit` / `push`, installer Rust (`rustup`), `cargo new`, `cargo run`, `cargo test`, lire un message d'erreur du compilateur | Le compilateur est un traducteur : ton texte devient des instructions pour le processeur. `cargo` est l'intendant qui appelle le traducteur, range les fichiers et lance les tests | On crée, compile et lance un « bonjour » soi-même, et on explique chaque mot de la commande |
| P2 | ⬜ | **Rust de base** | Variables et mutabilité (`let`, `mut`), types, `if`, `match`, boucles, fonctions, `Vec`, `String`, `println!` | Les briques de Lego | Exercices : moyenne d'une liste, inverser un texte, compter les mots |
| P3 | ⬜ | **Possession et emprunt** (*ownership*, *borrowing*) | Qui possède une valeur, déplacer ou emprunter (`&`, `&mut`), les tranches (`&[T]`), durées de vie, `Box`, pile et tas | Un livre de bibliothèque : un seul propriétaire ; on peut le prêter à plusieurs lecteurs **ou** à un seul qui écrit dedans, jamais les deux à la fois | Exercices qui compilent, et chaque refus du compilateur rencontré est expliqué avec ses mots |
| P4 | ⬜ | **Structures, traits et génériques** | `struct`, `enum`, `impl`, traits, génériques, erreurs avec `Result` et `?`, `Option` | Un moule à gâteaux (générique) et un contrat (« tout ce qui sait se multiplier ») | Une petite `Matrix<T>` générique, avec tests |
| P5 | ⬜ | **Maths minimales, en Rust** | Vecteurs, produit scalaire, matrices, produit de matrices, dérivée, dérivée en chaîne | Une recette appliquée à toute une rangée de cases d'un coup | Chaque notion d'abord à la main sur papier, puis codée et testée (dont une dérivée vérifiée par différences finies) |
| P6 | ⬜ | **Un neurone à la main** | Un neurone, puis un tout petit réseau : calcul avant, erreur, correction des poids | Avant de construire l'usine, fabriquer une pièce à la main | Le réseau apprend la fonction XOR, en Rust, sans aucune bibliothèque |

Puis : **module 0**.

P3 est la marche la plus haute de Rust. On y passe le temps qu'il faut : tout le reste du cursus repose dessus.

---

## La boîte à outils sur Mac

| Outil | Rôle | Image |
|---|---|---|
| **rustup** | Installer et mettre à jour Rust | Le magasin officiel où l'on va chercher la boîte à outils |
| **cargo** | Construire, lancer, tester, gérer les dépendances (*crates*) | L'intendant qui fait tout le rangement |
| **clippy** | Conseils de style et pièges courants | Un relecteur pointilleux mais bienveillant |
| **rustfmt** | Mise en forme automatique du code | Le correcteur qui aligne les marges |
| **Miri** (`cargo +nightly miri test`) | Vérifier le code `unsafe` | Un détecteur de fumée, pour les rares pièces où l'on manipule le feu |
| **VS Code + rust-analyzer + CodeLLDB** | Éditer (complétion, erreurs en direct) et déboguer pas à pas | Le carnet, le stylo et la loupe |
| **Instruments** (Xcode) | Mesurer où le temps passe | Le chronomètre et la caméra au ralenti |
| **Accelerate** (appelé en FFI) | Bibliothèque de calcul d'Apple ; `cblas_sgemm` y est très rapide | L'étalon : la voiture du champion à qui l'on compare la nôtre |
| **NEON** (`std::arch::aarch64`) | Calcul vectorisé (SIMD) sur ARM, à la main | Une rangée de caisses qui encaissent en même temps |
| **Metal** (via la crate `objc2-metal`, à valider au module 8) | Programmer le GPU depuis Rust | Embaucher une armée de petits calculateurs |

Les outils de compilation d'Apple (`xcode-select --install`) restent nécessaires : Rust s'en sert pour assembler le programme final et pour parler aux frameworks Apple.

**Références à lire en parallèle :**
- **[huggingface/candle](https://github.com/huggingface/candle)** et **[tracel-ai/burn](https://github.com/tracel-ai/burn)** : des frameworks de ML en Rust. On y voit comment d'autres ont résolu les mêmes problèmes dans la même langue.
- **[ml-explore/mlx](https://github.com/ml-explore/mlx)** (C++) : conçu par Apple pour Apple Silicon. Le meilleur professeur silencieux pour les modules 1, 2 et 8, même dans une autre langue.

### Les deux machines

| | Mac M2 (actuel) | Mac mini M5 Pro (fin novembre 2026) |
|---|---|---|
| CPU | 8 cœurs | 18 cœurs |
| GPU | (selon modèle) | 20 cœurs |
| Mémoire unifiée | 24 Go | **64 Go** |
| Usage dans le cursus | Préparation et modules 0 à 6 sans souci ; module 7 avec de petits modèles | Tout le cursus, dont le fine-tuning LoRA du module 9 |

Repères pour la taille des modèles avec **notre** moteur (moins optimisé que llama.cpp) :
- M2, 24 Go : modèles de 0,5 à 3 milliards de paramètres en inférence ; entraînement de petits modèles maison.
- M5 Pro, 64 Go : inférence jusqu'à 7-8 milliards confortablement (plus en quantifié) ; LoRA sur 1 à 3 milliards en bf16.

Ce sont des ordres de grandeur, à remplacer par des mesures réelles.

À explorer sur le M5 Pro : Apple annonce des accélérateurs neuronaux dans les cœurs GPU des puces M5, accessibles par les API tenseur de Metal 4. **À vérifier** sur la machine avant d'en dépendre ; candidat pour un module 8b.

### Le Rust pratiqué

- **Rust stable, édition 2024.** Les fonctions réservées à la version *nightly* (expérimentale) sont évitées ; si l'une devient indispensable, l'isoler et le signaler.
- **`unsafe` rare et enfermé** : seulement pour la FFI (Accelerate, Metal), le SIMD et le parsing de fichiers projetés en mémoire. Chaque bloc `unsafe` porte un commentaire `// SAFETY:` qui dit pourquoi il est correct, et passe sous Miri quand c'est possible.
- Demi-précision (`f16`, `bf16`) : le type natif `f16` de Rust n'était pas stable au départ du projet. Vérifier au moment du besoin ; sinon, proposer la crate `half`.

---

## Les modules

État : ⬜ à faire · 🔄 en cours · ✅ validé

| # | État | Module | Concepts dlib / extension | Rust pratiqué | Livrable | Critère de validation |
|---|---|---|---|---|---|---|
| 0 | ⬜ | **Fondations** | Organisation d'une bibliothèque | Espace de travail cargo (*workspace*), tests, clippy, rustfmt, profils debug et release | Projet qui compile, tests qui passent | `cargo test` à 100 % en debug et en release ; `cargo clippy` sans avertissement |
| 1 | ⬜ | **Tenseur** | `tensor`, `resizable_tensor`, `alias_tensor` | Tranches, traits `Index`, emprunts pour les vues, génériques constants | Tenseur avec formes, *strides* et vues | Tests de formes, vues, transpositions sans copie |
| 2 | ⬜ | **Kernels CPU** | `cpu_dlib`, abstraction de backend | `std::arch::aarch64` (NEON), `std::thread::scope`, FFI vers Accelerate | Multiplication de matrices naïve → par tuiles → NEON → multithread | Résultats identiques à Accelerate (tolérance) ; GFLOPS mesurés et comparés |
| 3 | ⬜ | **Gradients statiques** | Pile de couches compilée par templates (style dlib : `add_layer`, `fc`, `relu`…) | Génériques et traits : une couche = un type, un réseau = des types emboîtés, que le compilateur spécialise (monomorphisation) | Réseau statique avec `forward`/`backward` | Gradients vérifiés par différences finies |
| 3b | ⬜ | **Gradients dynamiques** | Moteur dynamique de l'extension | Graphe dans une « arène » avec des indices, objets de trait (`dyn Trait`) | Graphe enregistré puis parcouru à l'envers | **Mêmes résultats que le module 3** sur le même réseau |
| 4 | ⬜ | **Entraînement classique** | Couches de perte, `adam`, `dnn_trainer`, sérialisation | `Result` et erreurs typées, trait `Drop`, lecture et écriture binaires | Classifieur MNIST de bout en bout | Précision de test > 97 % ; sauvegarde puis rechargement identiques |
| 5 | ⬜ | **Transformer** | Couches Transformer de l'extension | Composition de couches génériques | **Mini-GPT au niveau caractère** entraîné sur Molière | La perte baisse ; le texte généré ressemble (de loin) à du Molière |
| 6 | ⬜ | **Tokenizer BPE** | Tokenizer entraînable de l'extension | `&str` et UTF-8, itérateurs, `HashMap` | BPE : entraînement, encodage, décodage | `decode(encode(x)) == x` ; mêmes jetons qu'un tokenizer de référence sur un vocabulaire GGUF |
| 7 | ⬜ | **GGUF et quantification** | Import GGUF, Q8_0, Q4_K, cache KV, échantillonnage | Projection du fichier en mémoire (`mmap`, crate à proposer), parsing binaire défensif, `Result` | Inférence d'un vrai petit modèle (Qwen 0.5B, SmolLM…) | Logits à moins d'une tolérance fixée de ceux de llama.cpp, même prompt |
| 8 | ⬜ | **Backend Metal** | Routage CPU/GPU par opération | FFI Objective-C (`objc2-metal`), shaders en Metal Shading Language, mémoire unifiée | Multiplication de matrices, softmax, attention sur GPU | Mêmes résultats que le CPU ; gain de vitesse mesuré |
| 9 | ⬜ | **Fine-tuning** | SFT avec masquage, LoRA, DoRA, *gradient checkpointing*, précision mixte | — | Fine-tuning LoRA du modèle du module 7 | La perte baisse sur le jeu d'entraînement ; le comportement change sur des prompts témoins |
| 10 | ⬜ | **Bonus** | Distillation, élagage, embeddings et recherche, compression par LLM, vision, serveur compatible OpenAI | Réseau avec la bibliothèque standard, ou `async` (dépendance à proposer) | Au choix | À définir au moment venu |

### Le fil rouge : statique contre dynamique

Les modules **3 et 3b** reproduisent la **dualité centrale** de Dlib-Transformer-extensions :
- un réseau **statique**, dont la forme est connue à la compilation, comme dans dlib : le compilateur voit tout et optimise (image : un meuble en kit déjà assemblé à l'usine). En Rust, ce sont les **génériques** : dlib le fait avec des templates C++, nous avec des types emboîtés et des traits ;
- un moteur **dynamique**, dont le graphe se construit à l'exécution : plus souple, comme PyTorch (image : des briques de Lego qu'on assemble au moment de jouer). En Rust, un graphe avec des liens dans tous les sens se bat avec les règles d'emprunt. La solution classique : ranger tous les nœuds dans un grand tableau (une « arène ») et les relier par leur **numéro** plutôt que par des pointeurs. Comme des places de cinéma : on se retrouve « rangée F, siège 12 », pas en se tenant par la main.

Les deux doivent donner le **même résultat**. C'est ce test qui fait comprendre les deux approches.

### Le moment de vérité

Le **module 5** : voir sa propre implémentation, sans aucune bibliothèque de ML, apprendre à écrire du Molière approximatif. Le Molière sera mauvais, mais ce sera le nôtre.

---

## Conseils

- **Ne pas sauter les modules 1 à 4** pour courir vers les LLM. Tout ce qui est difficile ensuite (gradients faux, *strides* mal gérés, instabilité numérique) vient de là.
- **Ne pas se battre contre le compilateur : l'écouter.** Quand Rust refuse, c'est souvent qu'il a vu un vrai problème. Si on se bat trop longtemps, c'est en général la structure qu'il faut changer (des indices plutôt que des références, posséder plutôt qu'emprunter), pas la syntaxe.
- **`clone()` est permis pour débloquer** pendant l'apprentissage. On optimise plus tard, quand le code marche et qu'une mesure dit que ça compte.
- **Accelerate d'abord comme étalon, Metal ensuite.** Sur puce M, Accelerate passe par les unités matricielles du CPU : il sera très difficile à battre avec du NEON maison. C'est une leçon d'humilité, et une bonne mesure.
- **Vérifier chaque gradient par différences finies.** Sans ça, on entraîne des bugs.
- **Parser le GGUF de façon défensive** : chaque taille et chaque décalage lu dans le fichier est une donnée non fiable, même en Rust (un `unsafe` mal gardé suffit).
- Les vrais pièges, dans l'ordre où ils arrivent : la rétropropagation de l'attention (mémoire des activations), la précision mixte (bf16/fp16), les performances des kernels quantifiés. C'est là que les semaines disparaissent.
- Compter **plusieurs mois** à rythme raisonnable : c'est un cursus de master en ML doublé d'un apprentissage sérieux de Rust.

## Lectures de référence

### Pour la préparation

| Pour | Ressource |
|---|---|
| Apprendre Rust pas à pas | **Le livre officiel, *The Rust Programming Language*** ([doc.rust-lang.org/book](https://doc.rust-lang.org/book/)), avec une **traduction française** : [*Le langage de programmation Rust*](https://jimskapt.github.io/rust-book-fr/) |
| S'entraîner par petits exercices | **[rustlings](https://github.com/rust-lang/rustlings)** : des centaines de mini-exercices que le compilateur corrige. Parfait pour P2 à P4 |
| Voir des exemples courts | [*Rust by Example*](https://doc.rust-lang.org/rust-by-example/) |
| Un cours complet et structuré | [*Comprehensive Rust*](https://google.github.io/comprehensive-rust/) (Google), disponible en français |
| Voir les maths | **3Blue1Brown** sur YouTube : *Essence of Linear Algebra* et *Neural Networks* (sous-titres français) |
| Comprendre les réseaux de neurones | Andrej Karpathy, *Neural Networks: Zero to Hero* (YouTube) : on regarde, on recode en Rust |
| Petites expériences dans le navigateur | **[play.rust-lang.org](https://play.rust-lang.org)** : on tape du Rust, on lance, sans rien installer |
| Chercher une fonction de la bibliothèque standard | [doc.rust-lang.org/std](https://doc.rust-lang.org/std/) |

### Pour les modules

| Pour | Lire |
|---|---|
| Concepts de dlib (couches, tenseurs, entraînement) | Le code de [davisking/dlib](https://github.com/davisking/dlib), dossier `dlib/dnn/` (C++) |
| Transformers dans dlib | [Cydral/Dlib-Transformer-extensions](https://github.com/Cydral/Dlib-Transformer-extensions) (C++) |
| ML en Rust | [huggingface/candle](https://github.com/huggingface/candle), [tracel-ai/burn](https://github.com/tracel-ai/burn) |
| Clarté d'un LLM en C | [karpathy/llama2.c](https://github.com/karpathy/llama2.c), [karpathy/llm.c](https://github.com/karpathy/llm.c) |
| Calcul de gradients minimal | La vidéo *micrograd* d'Andrej Karpathy (série *Neural Networks: Zero to Hero*) : on **regarde** l'explication au tableau, on **recode en Rust** |
| Tenseurs et Metal sur Apple Silicon | [ml-explore/mlx](https://github.com/ml-explore/mlx) (C++) |
| GGUF et quantification | [ggml-org/ggml](https://github.com/ggml-org/ggml), [ggml-org/llama.cpp](https://github.com/ggml-org/llama.cpp) |
| Tokenizer BPE | [karpathy/minbpe](https://github.com/karpathy/minbpe) |
