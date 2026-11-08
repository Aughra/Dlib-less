# P2-04 — Les conversions

## Le but

Le programme affiche exactement :

```
37 °C = 98.6 °F
moyenne = 13.5
```

## Les règles

- Les trois boîtes de départ restent telles quelles, des entiers (`i32`) : `celsius`, `note1`, `note2`.
- Formule : °F = °C × 9 / 5 + 32.
- La moyenne est celle des deux notes.
- Convertis avec `as f64` : en entiers, on obtient `98 °F` et `moyenne = 13`.

## Comment savoir si c'est juste

```
cargo test
```

Deux tests doivent passer. Tu ne modifies que `src/main.rs`.
