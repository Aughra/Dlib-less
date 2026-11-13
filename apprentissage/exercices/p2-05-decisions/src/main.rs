fn main() {
    let age: u32 = 25;
    let cat = categorie(age);
    println!("La catégorie pour {} ans est {}", age, cat);
}

fn categorie(age: u32) -> &'static str {
    if age < 13 {
        "enfant"
    } else if age < 18 {
        "ado"
    } else {
        "adulte"
    }
}

// Tests written by Claude: they check the borders (12/13, 17/18), where bugs hide.
#[cfg(test)]
mod tests {
    use super::categorie;

    #[test]
    fn enfant_jusqu_a_12() {
        assert_eq!(categorie(0), "enfant");
        assert_eq!(categorie(12), "enfant");
    }

    #[test]
    fn ado_de_13_a_17() {
        assert_eq!(categorie(13), "ado");
        assert_eq!(categorie(17), "ado");
    }

    #[test]
    fn adulte_des_18() {
        assert_eq!(categorie(18), "adulte");
        assert_eq!(categorie(25), "adulte");
        assert_eq!(categorie(90), "adulte");
    }
}
