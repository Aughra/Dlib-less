# P2-02 — La tirelire de la semaine

## Le but

Le programme affiche exactement :

```
lundi : 2
mardi : 7
mercredi : 10
reste à économiser : 10
```

## Les règles

- Une boîte `objectif` qui contient 20 et ne change jamais.
- Une boîte `tirelire` qui commence à 0. On y ajoute 2 lundi, 5 mardi, 3 mercredi, avec `+=`.
- Le reste est **calculé** à partir des deux boîtes, pas écrit à la main.

Indice : entre les accolades de `println!`, on ne met qu'un nom de boîte, pas un calcul.

## Comment savoir si c'est juste

```
cargo test
```

Trois tests doivent passer : l'affichage, `objectif` scellée, `tirelire` mutable. Tu ne modifies que `src/main.rs`.
