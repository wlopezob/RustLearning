pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn puede_contener(&self, otro: &Rectangulo) -> bool {
        self.ancho > otro.ancho && self.alto > otro.alto
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn grande_puede_contener_pequeno() {
        let grande = Rectangulo{
            ancho: 8, 
            alto: 7
        };
        let pequeno = Rectangulo {
            ancho: 5,
            alto: 1
        };
        assert!(grande.puede_contener(&pequeno));
    }
}
