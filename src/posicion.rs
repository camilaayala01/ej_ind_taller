pub mod posicion

{
use crate::pieza::pieza::Color;

#[derive(Clone)]
pub struct Posicion
{
    pub x: usize,
    pub y:usize,
}
impl Copy for Posicion{}
impl Posicion
{
    pub fn esta_en_diagonal_de(origen:Self,destino:Posicion)->bool
    { 
        origen.x.abs_diff(origen.y) == destino.x.abs_diff(destino.y)
    }
    pub fn esta_misma_linea_de(origen:Self,destino:Posicion)-> bool
    {
        origen.x == destino.x || origen.y == destino.y
    }

    pub fn esta_a_una_casilla_de(origen:Self,destino:Posicion)-> bool
    {
        (origen.x == destino.x  && origen.y.abs_diff(destino.y) == 1)
        || (origen.y == destino.y && origen.x.abs_diff(destino.x)==1)
    }
    pub fn esta_en_l_de(origen:Self,destino:Posicion)-> bool
    {
        (origen.x.abs_diff(destino.x)==2 && origen.y.abs_diff(destino.y)==3)
        || (origen.y.abs_diff(destino.y)==2 && origen.x.abs_diff(destino.x) == 3)
    }
    pub fn esta_en_frente_de(origen:Self,destino:Posicion,color:&Color)-> bool
    {
        match color
        {
            Color::Blanco => origen.x == destino.x && origen.y +1 == destino.y,
            Color::Negro => origen.x == destino.x && origen.y == destino.y +1,
        
        }
    }
    
    
}
#[cfg(test)]
mod tests {
    use crate::pieza::pieza::Color;

    use super::Posicion;

    #[test]
    fn puede_determinarse_si_esta_en_diagonal_de()
    {
        let p1 = Posicion
        {
            x: 0,
            y:0,
        };
        let p2 = Posicion
        {
            x:1,
            y:1,
        };
        assert!(Posicion::esta_en_diagonal_de(p1,p2));
        let p3 = Posicion
        {
            x:2,
            y:3,
        };
        let p4 = Posicion
        {
            x:4,
            y:5,
        };
        assert!(Posicion::esta_en_diagonal_de(p3,p4));
        assert!(!Posicion::esta_en_diagonal_de(p1, p3));
    }
    #[test]
    fn puede_determinarse_si_esta_en_frente_de()
    {
        let p1 = Posicion
        {
            x: 0,
            y:0,
        };
        let p2 = Posicion
        {
            x:0,
            y:1,
        };
        assert!(Posicion::esta_en_frente_de(p1,p2,&Color::Blanco));
        let p3 = Posicion
        {
            x:2,
            y:3,
        };
        let p4 = Posicion
        {
            x:2,
            y:4,
        };
        assert!(Posicion::esta_en_frente_de(p4,p3,&Color::Negro));
        assert!(!Posicion::esta_en_frente_de(p2, p3, &Color::Blanco));
        assert!(!Posicion::esta_en_frente_de(p2, p3, &Color::Negro));
        assert!(!Posicion::esta_en_frente_de(p1, p2, &Color::Negro));

    }
    #[test]
    fn puede_determinarse_si_en_misma_linea_de()
    {
        let p1 = Posicion
        {
            x: 0,
            y:0,
        };
        let p2 = Posicion
        {
            x:0,
            y:7,
        };
        assert!(Posicion::esta_misma_linea_de(p1,p2));
        let p3 = Posicion
        {
            x:2,
            y:3,
        };
        let p4 = Posicion
        {
            x:3,
            y:3,
        };
        assert!(Posicion::esta_misma_linea_de(p4,p3));
        assert!(Posicion::esta_misma_linea_de(p3,p4));
        assert!(!Posicion::esta_misma_linea_de(p2, p3));
        assert!(!Posicion::esta_misma_linea_de(p3, p2));
        assert!(!Posicion::esta_misma_linea_de(p1, p4));

    }
    #[test]
    fn puede_determinarse_si_esta_al_lado_de()
    {
        let p1 = Posicion
        {
            x: 0,
            y:0,
        };
        let p2 = Posicion
        {
            x:0,
            y:1,
        };
        assert!(Posicion::esta_misma_linea_de(p1,p2));
        let p3 = Posicion
        {
            x:1,
            y:1,
        };
        let p4 = Posicion
        {
            x:2,
            y:1,
        };
        assert!(Posicion::esta_a_una_casilla_de(p4,p3));
        assert!(Posicion::esta_a_una_casilla_de(p3,p4));
        assert!(Posicion::esta_a_una_casilla_de(p1,p2));
        assert!(Posicion::esta_a_una_casilla_de(p2,p1));
        assert!(Posicion::esta_a_una_casilla_de(p2, p3));
        assert!(Posicion::esta_a_una_casilla_de(p3, p2));
        assert!(!Posicion::esta_a_una_casilla_de(p1, p4));

    }
}
}
