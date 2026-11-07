# Notes de départ

> Archive de la discussion du 6 novembre 2026 qui a donné naissance à dlib-less. Ce document ne se réécrit pas, sauf erreur de fait. Le plan vivant est dans [cursus.md](cursus.md).
>
> Les informations sur les dépôts tiers (étoiles, activité, fonctionnalités) datent de ce jour et peuvent avoir changé. Celles sur Dlib-Transformer-extensions viennent de son README, non vérifiées dans le code.

---

## 1. dlib, c'est quoi ?

[davisking/dlib](https://github.com/davisking/dlib) est une **bibliothèque C++ généraliste, sans dépendances obligatoires**, sous licence Boost (très permissive, utilisable en commercial), avec des bindings Python.

| Domaine | Ce que dlib propose |
|---|---|
| **Visage** (sa célébrité) | Détecteur HOG+SVM, détecteur CNN (MMOD), **68 points de repère faciaux** (*shape predictor*), **reconnaissance faciale** par ResNet (vecteurs à 128 dimensions) |
| **Machine learning** | SVM, régression, clustering, réseaux de neurones (son propre framework DNN en C++), *structural SVM* |
| **Optimisation** | BFGS, L-BFGS, optimisation sans dérivée, `find_min_global` pour régler des hyperparamètres |
| **Algèbre linéaire** | Matrices, décompositions (avec BLAS/LAPACK en option) |
| **Traitement d'image** | Filtres, transformations, suivi d'objets (*correlation tracker*) |
| **Utilitaires** | Threads, réseau, sérialisation, une petite interface graphique |

La bibliothèque Python `face_recognition` est en fait une surcouche de dlib.

### Équivalents de dlib sur GitHub

- **Visage** : [opencv/opencv](https://github.com/opencv/opencv) (+ opencv_contrib : YuNet, SFace), [deepinsight/insightface](https://github.com/deepinsight/insightface) (état de l'art), [serengil/deepface](https://github.com/serengil/deepface), [google-ai-edge/mediapipe](https://github.com/google-ai-edge/mediapipe), [timesler/facenet-pytorch](https://github.com/timesler/facenet-pytorch), [1adrianb/face-alignment](https://github.com/1adrianb/face-alignment), [ShiqiYu/libfacedetection](https://github.com/ShiqiYu/libfacedetection), [TadasBaltrusaitis/OpenFace](https://github.com/TadasBaltrusaitis/OpenFace).
- **ML / deep learning en C++** : [mlpack/mlpack](https://github.com/mlpack/mlpack) (le plus proche de dlib pour le ML classique), LibTorch, [microsoft/onnxruntime](https://github.com/microsoft/onnxruntime), [Tencent/ncnn](https://github.com/Tencent/ncnn), [alibaba/MNN](https://github.com/alibaba/MNN). En Python : scikit-learn.
- **Algèbre linéaire** : Eigen, [xtensor-stack/xtensor](https://github.com/xtensor-stack/xtensor), Armadillo.
- **Optimisation** : [ceres-solver/ceres-solver](https://github.com/ceres-solver/ceres-solver), [stevengj/nlopt](https://github.com/stevengj/nlopt), [yixuan/LBFGSpp](https://github.com/yixuan/LBFGSpp).
- **Image** : OpenCV, [scikit-image](https://github.com/scikit-image/scikit-image), [ukoethe/vigra](https://github.com/ukoethe/vigra).

---

## 2. Dlib-Transformer-extensions, c'est quoi ?

[Cydral/Dlib-Transformer-extensions](https://github.com/Cydral/Dlib-Transformer-extensions) est une **extension de dlib en C++14** (licence Boost) qui apporte les **Transformers et les LLM** à dlib. Dépôt très confidentiel (environ 5 étoiles), mais très ambitieux. D'après son README :

- **Import GGUF** (llama, mistral, gemma, qwen), décompression des formats quantifiés Q et k-quants (IQ1 à IQ3 refusés plutôt que devinés).
- **Deux moteurs d'exécution** : un moteur dynamique pour l'inférence, et un réseau dlib compilé statiquement qu'on peut entraîner. Les deux sont tenus à un accord bit à bit.
- **Fine-tuning** : SFT avec masquage du prompt, LoRA et DoRA. Prévus : QLoRA, DPO.
- **Distillation** (à partir de traces d'un modèle professeur) et **élagage en profondeur** (*depth pruning*).
- **Vision et multimodal** (ViT + fusion image/texte).
- **Embeddings et recherche documentaire** (RAG).
- **Tokenizer BPE** entraînable.
- **Anonymisation** de données personnelles (Office, PDF, texte) dans 5 langues, réversible.
- **Compression sans perte** de texte : LLM + codeur arithmétique.
- **Matériel** : CPU, CUDA, et NPU ou GPU intégré via OpenVINO, avec routage par opération.
- **Serveur** compatible OpenAI (`/v1/chat/completions`) avec interface de chat web.
- Deux applications Windows : **Albireo** (inférence locale) et **Pseudalis** (anonymisation).

### Équivalents de l'extension sur GitHub

**Moteurs LLM en C/C++ :**

| Dépôt | Pourquoi c'est proche |
|---|---|
| [ggml-org/llama.cpp](https://github.com/ggml-org/llama.cpp) | Le jumeau évident : origine du GGUF, quantification, multimodal, embeddings, serveur compatible OpenAI. Fine-tuning encore limité. |
| [ggml-org/ggml](https://github.com/ggml-org/ggml) | La bibliothèque de tenseurs sous llama.cpp : son rôle pour llama.cpp est celui de dlib pour l'extension. |
| [google/gemma.cpp](https://github.com/google/gemma.cpp) | Inférence Gemma en C++ pur, minimaliste. |
| [OpenNMT/CTranslate2](https://github.com/OpenNMT/CTranslate2) | Moteur Transformer en C++, quantifié. |
| [karpathy/llm.c](https://github.com/karpathy/llm.c) | Entraînement de GPT-2 de bout en bout en C/CUDA. |
| [openvinotoolkit/openvino.genai](https://github.com/openvinotoolkit/openvino.genai) | LLM sur CPU, GPU intégré et NPU Intel. |
| [microsoft/onnxruntime-genai](https://github.com/microsoft/onnxruntime-genai), [alibaba/MNN](https://github.com/alibaba/MNN), [pytorch/executorch](https://github.com/pytorch/executorch) | Inférence locale et embarquée. |
| [huggingface/candle](https://github.com/huggingface/candle), [tracel-ai/burn](https://github.com/tracel-ai/burn) | Même philosophie (natif, sans Python), en Rust. |

**Brique par brique :**

- **SFT, LoRA, DoRA, DPO** : [huggingface/peft](https://github.com/huggingface/peft), [huggingface/trl](https://github.com/huggingface/trl), [unslothai/unsloth](https://github.com/unslothai/unsloth), [hiyouga/LLaMA-Factory](https://github.com/hiyouga/LLaMA-Factory).
- **Distillation et élagage** : [horseee/LLM-Pruner](https://github.com/horseee/LLM-Pruner), [NVIDIA/TensorRT-Model-Optimizer](https://github.com/NVIDIA/TensorRT-Model-Optimizer).
- **Tokenizer BPE** : [google/sentencepiece](https://github.com/google/sentencepiece), [huggingface/tokenizers](https://github.com/huggingface/tokenizers), [karpathy/minbpe](https://github.com/karpathy/minbpe).
- **Embeddings et RAG** : [UKPLab/sentence-transformers](https://github.com/UKPLab/sentence-transformers), [facebookresearch/faiss](https://github.com/facebookresearch/faiss).
- **Anonymisation** : [microsoft/presidio](https://github.com/microsoft/presidio), la référence du domaine.
- **Compression par LLM** : [google-deepmind/language_modeling_is_compression](https://github.com/google-deepmind/language_modeling_is_compression) ; hors GitHub, *ts_zip* et *nncp* de Fabrice Bellard.
- **Serveur et interface** : [open-webui/open-webui](https://github.com/open-webui/open-webui), [ollama/ollama](https://github.com/ollama/ollama), [janhq/jan](https://github.com/janhq/jan).

**Avis :** llama.cpp + ggml couvrent peut-être 70 % du périmètre. Ce que l'extension a de singulier, c'est le **réseau statique entraînable dans dlib**, avec LoRA, DoRA et distillation en C++ pur. Contrepartie : la maturité (un développeur principal, des promesses de README à vérifier).

---

## 3. Inférence locale et fine-tuning : la combinaison « outil prêt à l'emploi »

Pour mémoire, si un jour on veut juste **utiliser** un modèle fine-tuné sans tout réécrire :

```
Fine-tuning (Python, GPU)          Conversion           Inférence locale (C++)
Unsloth / TRL + PEFT (QLoRA)  →  fusion LoRA → GGUF  →  llama.cpp  (ou Ollama)
                                 + quantification Q4_K_M / Q5_K_M
```

| Situation | Fine-tuning | Inférence |
|---|---|---|
| GPU NVIDIA | Unsloth, ou LLaMA-Factory (interface graphique) | llama.cpp / Ollama |
| **Mac Apple Silicon** | **[ml-explore/mlx-lm](https://github.com/ml-explore/mlx-lm)** (LoRA et inférence dans le même outil) | mlx-lm ou llama.cpp (Metal) |
| Serveur GPU, plusieurs utilisateurs | Unsloth / TRL | [vllm-project/vllm](https://github.com/vllm-project/vllm) |
| PC Intel avec NPU | Unsloth (ailleurs) | openvino.genai |
| Tout en C++ | Dlib-Transformer-extensions, ou `llama-finetune` (expérimental) | llama.cpp |

Conseils : commencer petit (1 à 4 milliards de paramètres), soigner le jeu de données (80 % du résultat), préférer QLoRA au fine-tuning complet.

Sur le Mac de dlib-less, **mlx-lm et llama.cpp** serviront de **références** pour vérifier nos propres résultats.

---

## 4. Refaire l'extension sans dlib

### Ce que dlib fournissait à l'extension

| Brique dlib | Rôle | Remplaçant dans dlib-less |
|---|---|---|
| `tensor` / `resizable_tensor` | Stockage des données, CPU et GPU | Notre propre tenseur (module 1) |
| `forward` / `backward` par couche | Rétropropagation | Notre calcul de gradients, statique et dynamique (modules 3 et 3b) |
| Wrappers cuDNN et cuBLAS | Accélération GPU | Metal via metal-cpp (module 8) |
| Bindings BLAS (oneMKL…) | Multiplication de matrices sur CPU | Kernels NEON maison, Accelerate comme étalon (module 2) |
| `adam`, `dnn_trainer` | Optimiseur et boucle d'entraînement | AdamW et notre boucle (module 4) |
| Sérialisation | Sauvegarde des modèles | GGUF et/ou safetensors (modules 4 et 7) |
| Threads | Parallélisme | `std::jthread` et un pool de threads (module 2) |

### Les trois fondations envisagées

| Option | Pour | Contre |
|---|---|---|
| **A. ggml** | Tenseurs, graphe, backends, GGUF, quantification, gradients : gratuits | Le projet devient un llama.cpp bis ; API C, graphe figé |
| **B. LibTorch** | Calcul de gradients solide, entraînement simple | Très lourd, ni GGUF ni k-quants |
| **C. Tout écrire soi-même** | **Le plus formateur** : on comprend chaque étage | Le plus long ; la vitesse des formats quantifiés est un vrai chantier |

**Choix retenu : C**, parce que l'objectif est d'**apprendre en profondeur** les concepts de dlib et de l'extension, en C++ récent, sur Mac.

### Points d'attention relevés

- Sans dlib, le projet tend naturellement vers llama.cpp. Sa différence assumée : **la pédagogie** (code lisible et expliqué) et **le Mac comme seule cible**, avec un entraînement LoRA en C++ pur.
- Les vrais pièges : la rétropropagation de l'attention (mémoire des activations → *gradient checkpointing*), la précision mixte (bf16/fp16), et les performances des kernels quantifiés.
- Licence : dlib et l'extension sont sous Boost (BSL-1.0) ; un emprunt de code garde sa mention de licence et d'auteur.

---

## 5. Addendum du 7 novembre 2026 : Rust plutôt que C++

> Ajouté après coup, sans réécrire ce qui précède : le choix du langage a changé le lendemain. Partout plus haut où il est écrit « C++ » pour dlib-less, lire désormais « Rust ».

L'humain débute ; trois langages ont été comparés pour ce projet :

| | **C++** | **C** | **Rust** |
|---|---|---|---|
| Proximité avec dlib et l'extension | ✅ Leur langue : on transpose presque mot pour mot | ⚠️ Il faut traduire | ⚠️ Il faut traduire |
| Réseau « statique » à la dlib (modules 3 et 3b) | ✅ Les templates sont faits pour ça | ❌ Impossible sans génériques | ✅ Faisable avec génériques et traits, autrement |
| Facilité pour un débutant | ❌ Langage immense, plein de pièges | ✅ Petit langage… | ⚠️ Compilateur strict, première marche raide |
| Erreurs de mémoire | ❌ Possibles (sanitizers) | ❌❌ Partout, silencieuses | ✅ Empêchées par le compilateur |
| Outils | ⚠️ CMake | ⚠️ Make ou CMake | ✅ cargo, tout en une commande |
| Mac : Accelerate, Metal | ✅ metal-cpp officiel | ⚠️ Pont Objective-C | ✅ Crates de la communauté (`objc2-metal`) |
| Références | dlib, l'extension, MLX, ggml | llm.c, llama2.c, ggml | candle, burn |

En images : le C est une boîte à outils de base, où l'on taille chaque planche à la main ; le C++, un atelier de cinq cents machines dont certaines coupent si on ne lit pas la notice ; Rust, un atelier aussi complet avec un chef d'atelier qui refuse d'allumer une machine tant que les protections ne sont pas en place.

**Choix retenu : Rust.** Le compilateur joue au prof sévère, et les erreurs de mémoire sont arrêtées avant l'exécution. Ce qu'on accepte en échange :
- on s'éloigne du code de dlib : on le lit en C++, on traduit en Rust ;
- le graphe de gradients dynamique (module 3b) se bat avec les règles d'emprunt : la solution passe par une « arène » et des indices, ce qui est un cours en soi ;
- Metal passe par des crates de la communauté, pas par une enveloppe officielle d'Apple.

Le C a été écarté : le plus simple à apprendre, mais il fait perdre le réseau statique (le concept phare de dlib) et laisse toute la mémoire à la main.
