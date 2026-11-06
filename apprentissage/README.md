# Apprendre avec dlib-less

Ce dossier est **le tien**, Eliorë : ton carnet, tes exercices, tes quiz. Le code du projet vit ailleurs ; ici, c'est l'atelier où l'on s'entraîne.

## Le principe : je fais, on fait, tu fais

Pour chaque concept nouveau :

| Temps | Qui écrit ? | Ce qui se passe |
|---|---|---|
| **Je fais** | Claude | Un exemple court, expliqué ligne par ligne, avec une image de la vie courante |
| **On fait** | Toi, sur l'exemple | Tu changes une valeur, tu ajoutes un cas, tu regardes ce qui change ou ce qui casse |
| **Tu fais** | Toi, seul | Un exercice court. Un **test** te dit si c'est juste. Claude relit sans réécrire |

Pourquoi pas laisser Claude tout écrire ? Parce que regarder quelqu'un faire des pompes ne muscle pas. Si tu veux la solution, demande-la clairement (« donne-moi la solution ») : c'est permis, mais Claude proposera d'abord un indice.

Bloquer fait partie du travail. Une erreur de compilation n'est pas un échec : c'est le compilateur qui te parle, mal, mais il te parle. On apprend à le lire ensemble.

## Le rituel d'une séance (45 à 90 minutes)

1. Claude lit `logs/reprise.md` et te rappelle où tu en es, en deux phrases.
2. **Un seul concept** par séance, annoncé au début.
3. Je fais, on fait, tu fais. Le test de l'exercice passe.
4. Tu écris **trois lignes dans [carnet.md](carnet.md)**, avec tes mots. Si tu n'y arrives pas, ce n'est pas grave : on reprend ce concept à la séance suivante.
5. Commit et push.

Tous les 3 ou 4 concepts : un **mini-quiz** de 5 questions, dans [quiz/](quiz/). Ce n'est pas une note, c'est un thermomètre.

## Le chemin

Préparation **P1 à P6** (terminal et git, C++ de base, mémoire, classes et templates, maths, un neurone à la main), puis les modules **0 à 10**. Le détail est dans [../docs/cursus.md](../docs/cursus.md).

**Tout en C++.** Les idées de maths et de ML se posent d'abord sur papier, puis on les code en C++.

## Organisation du dossier

| Chemin | Contenu |
|---|---|
| [carnet.md](carnet.md) | Tes trois lignes par séance, et le lexique des mots appris |
| `exercices/PN-xx-nom/` | Un dossier par exercice : l'énoncé (`ENONCE.md`), ton code, son test (créés au fil des séances) |
| [quiz/](quiz/) | Les mini-quiz et tes réponses |

## Ressources

- **[learncpp.com](https://www.learncpp.com)** : le cours de C++ de référence, gratuit.
- **3Blue1Brown** (YouTube) : *Essence of Linear Algebra*, *Neural Networks*.
- **Andrej Karpathy** (YouTube) : *Neural Networks: Zero to Hero*, à regarder, puis recoder en C++.
- **[godbolt.org](https://godbolt.org)** : essayer du C++ dans le navigateur.
