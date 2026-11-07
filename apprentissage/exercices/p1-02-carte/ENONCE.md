# P1-02 — Une carte de visite

## Le but

Le programme affiche exactement ces trois lignes, accents et espaces compris :

```
Projet : dlib-less
Langage : Rust
Étape : P1
```

## Comment savoir si c'est juste

```
cargo test
```

- `test result: ok. 1 passed` : c'est juste.
- `FAILED` : regarde `left` (ce que ton programme affiche) et `right` (ce qui est attendu). Le `\n` veut dire « retour à la ligne ».

Tu ne modifies que `src/main.rs`. Le test, dans `tests/sortie.rs`, ne se touche pas.
