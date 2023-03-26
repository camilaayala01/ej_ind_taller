pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
use ajedrez::Tablero;
mod tests {
    

    #[test]
    fn no_se_puede_crear_tablero_con_archivo_invalido() {
        assert!(Tablero::new(String::from("")));
    }
}
