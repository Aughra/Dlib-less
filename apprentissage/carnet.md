# Mon carnet

Trois lignes par séance, avec mes mots : ce que j'ai compris, ce qui m'a surpris, ce qui reste flou. Le plus récent en haut.

## 2026-11-13 — P2 : les types (écrit par Claude, à la demande de l'humain)

- Compris : chaque valeur a un type (`i32`, `f64`…) et Rust ne les mélange pas tout seul ; on convertit à la main avec `as`. Entre deux entiers, `/` jette la virgule (`7 / 2 = 3`).
- Surpris par : les floats inexacts (`2.4499999999999997`) et le masquage (`let` deux fois avec le même nom).
- Encore flou : à confirmer par l'humain.

---

## 2026-11-13 — P2 : `if` / `else if` / `else` (écrit par Claude, à la demande de l'humain)

- Compris : un `if` choisit un chemin selon un oui/non, et renvoie une valeur (pas de `;` après). `>=` laisse passer la valeur pile, `>` non.
- Surpris par : le sens de `<` et `>` s'est inversé dans l'exercice ; seuls les tests aux bords (12/13, 17/18) l'ont montré.
- Encore flou : `match`, vu en exemple seulement, pas encore pratiqué.

---

```markdown
## 2026-11-08 — P1 : git

- Compris : tout 
- Surpris par : rien
- Encore flou : rien
```

---

## 2026-11-07 — P1 : cargo, créer, lancer, lire une erreur

- Compris : tout
- Surpris par : rien
- Encore flou : rien

---

## Lexique

Chaque mot nouveau, expliqué avec mes mots, et une image si elle me vient. Une ligne par mot :

```markdown
- **mot** : ce que ça veut dire. Image : …
```

- **compiler** : tranformer prgm en binaire.
- **cargo** : lancer les commandes pour tester , exec , compiler
- macro : la macro agit pendant la compil pour inserer du code text en rust, et est utilise avec !
- warning : une erreur non bloquante
- \n : new ligne
- repository : ce que contient le projet et l'histo
- commit ; prendre les fichier choisi pas git add du disk du projet en local
- index : ce qui ira dans le prochain commit.
- github : GitHub est un site, créé par d'autres, qui garde une copie des dépôts.
-
