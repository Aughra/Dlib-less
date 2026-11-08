# dlib-less, épisode 1 : un bonjour, une carte de visite et un passager clandestin

Le projet est né le 6 novembre 2026 : réécrire en Rust, sur un Mac, les idées de dlib, la bibliothèque d'apprentissage automatique de Davis King, et celles des extensions Transformer de Cydral. Sans dlib, et sans aucune bibliothèque de ML. Avant les tenseurs et les réseaux de neurones, il fallait d'abord apprendre à marcher : c'est l'étape P1 de la préparation, le Terminal, cargo et git.

## Changer de langue avant d'écrire la première ligne

Le plan initial prévoyait du C++, la langue de dlib. Le 7 novembre, il a changé pour Rust. La raison tient en une image : le compilateur de Rust est un professeur sévère qui corrige avant l'examen plutôt qu'après. Il refuse beaucoup, mais ses refus évitent les erreurs de mémoire qui, en C++, se découvrent au moment où le programme plante. Pour quelqu'un qui débute en programmation système, c'est un meilleur compagnon. Le prix à payer : un écart entre le code qu'on lit (du C++) et celui qu'on écrit (du Rust).

Bonne surprise au moment de commencer : Rust était déjà installé.

## La règle du jeu : un professeur qui ne fait pas les devoirs

dlib-less sert d'abord à apprendre. Claude y tient le rôle d'un professeur particulier, avec une méthode en trois temps : *je fais* (un exemple expliqué), *on fait* (on modifie l'exemple pour voir ce qui casse), *tu fais* (un exercice, avec un test qui dit si c'est juste). Il n'écrit pas la solution à la place de l'élève.

Le Terminal étant déjà connu, la première séance est passée directement à cargo.

## Cargo, l'intendant

Rust a un traducteur, `rustc`, qui transforme le texte du programme en instructions pour le processeur. Mais on ne lui parle presque jamais directement : on passe par **cargo**, l'intendant qui range les fichiers, appelle le traducteur et lance le programme.

La première commande a donné le ton :

```
cargo new P1-01-bonjoug
warning: package name `P1-01-bonjoug` is not snake_case or kebab-case
```

Deux erreurs en une ligne. Une faute de frappe (« bonjoug »), et une majuscule que cargo n'aime pas : il préfère les noms en minuscules reliés par des tirets, comme des morceaux enfilés sur une brochette. La majuscule, elle, venait de la consigne du professeur. Le projet a été effacé, recréé en `p1-01-bonjour`, et la convention corrigée dans la documentation.

## Apprendre à lire les reproches

Le cœur de la séance a consisté à casser exprès le programme, pour lire ce que le compilateur répond. Retirer le `!` de `println!` : Rust explique qu'il ne connaît pas de *fonction* de ce nom, mais qu'une *macro* existe, et propose d'ajouter le `!`. Une macro, c'est un tampon : avant la traduction, elle imprime du code à sa place.

Retirer un guillemet a donné la leçon la plus utile : le compilateur a signalé la faute **une ligne trop bas**. Il montre l'endroit où il s'est aperçu du problème, pas toujours celui où on l'a créé. Comme un correcteur qui ne remarque une phrase sans point qu'en arrivant à la suivante.

Et une surprise : retirer le point-virgule de la dernière ligne… ne change rien. En Rust, la dernière ligne d'un bloc sans `;` devient ce que le bloc rend, et `println!` ne rend qu'une boîte vide. Avec ou sans, le résultat est le même : rien.

## Le premier test vert

Le premier vrai exercice : un programme qui affiche une carte de visite de trois lignes, vérifié par un test automatique qui compare l'affichage au texte attendu. La solution prévue tenait en trois `println!`. Celle de l'élève n'en utilisait qu'un, avec des `\n` pour aller à la ligne, une astuce trouvée seul en lisant le test. Résultat : `test result: ok. 1 passed`.

La définition de « macro » dans le carnet a demandé trois essais : « une suite de commandes », puis « du code binaire », puis la bonne. Elle est copiée ici telle quelle :

> la macro agit pendant la compil pour inserer du code text en rust, et est utilise avec !

## Git, l'album photo, et le passager clandestin

Git aussi était déjà connu. La seconde séance est donc passée directement à l'exercice : faire soi-même un commit, c'est-à-dire une photo datée du projet, et l'envoyer sur GitHub, le coffre où dort la copie de l'album. Un petit script vérificateur cochait quatre cases.

Les quatre cases étaient vertes au deuxième essai. Mais en relisant la photo, on a trouvé un passager clandestin : une copie du script vérificateur, partie avec le reste. Le vérificateur ne pouvait pas la voir, car il vérifiait que rien n'était **oublié**, pas que rien n'était **en trop**. Un second commit l'a retirée. La leçon : regarder `git status` avant de prendre la photo.

Au passage, une idée reçue corrigée dans le lexique : Linus Torvalds a inventé **git** en 2005, pour le code de Linux. **GitHub** est un site fondé en 2008 par d'autres, qui garde des copies de dépôts git. L'appareil photo d'un côté, le coffre de la banque de l'autre.

## Les erreurs du professeur

Elles méritent d'être dites. Le premier exemple a été fabriqué dans un dossier temporaire du système, hors du projet. Il a été effacé, et un atelier dédié est né : le dossier `prof/`. Le nom en majuscules venait de la consigne du professeur. Le premier fichier de test ne respectait pas la mise en forme de Rust. Enfin, le professeur a été jugé « trop pointilleux » sur le carnet, à juste titre : la rigueur est gardée pour le fond, et la forme ne reçoit plus qu'une remarque.

Deux règles sont nées en chemin. Les tableaux Markdown, pénibles à écrire à la main, ne sont plus demandés à l'élève. Et le blog compte désormais un épisode par étape du cursus, pour être relu plus tard comme une madeleine.

## Pour les curieux : les commandes et les messages

Les erreurs lues pendant la première séance, copiées du Terminal :

```
cd: too many arguments
```

```
error[E0423]: cannot find function `println` in this scope
 --> src/main.rs:2:5
  |
2 |     println("Hello, world!");
  |     ^^^^^^^ not found in this scope
  |
  = note: a macro named `println` exists in another namespace
help: use `!` to invoke the macro
```

```
error[E0765]: unterminated double quote string
 --> src/main.rs:3:28
```

La solution de la carte de visite :

```rust
fn main() {
    println!("Projet : dlib-less\nLangage : Rust\nÉtape : P1");
}
```

Le vérificateur de l'exercice git (`prof/p1-git/verifier.sh`), au dernier essai :

```
✅ rien d'oublié : aucun fichier modifié hors d'un commit
✅ le dernier commit contient le carnet
✅ le dernier commit est le tien (pas celui de Claude)
✅ le commit est poussé sur GitHub

Bravo : exercice réussi.
```

Les légendes des trois commits de l'élève, dans l'ordre : « start », « P1 : git », « ajout du p1 ».

## La suite

P2 : les bases de Rust. Variables, types, conditions, boucles, fonctions. Les briques de Lego avec lesquelles tout le reste sera construit.
