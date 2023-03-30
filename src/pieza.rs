
pub mod pieza
{
    use crate::posicion::posicion::Posicion;

pub enum Tipo 
{
    Rey,
    Dama,
    Alfil,
    Caballo,
    Torre,
    Peon,
}

pub fn definir_tipo(identificador:&char)-> Tipo
{
    let id = identificador.to_ascii_lowercase();
    if id == 'r'  
    {
        return Tipo::Rey;
    }
    if id == 'd' 
    {
        return Tipo::Dama;

    }
    if id == 'a' 
    {
        return Tipo::Alfil; 
    }
    if id == 'c' 
    {
        return Tipo::Caballo;
    }
    if id== 't'
    {
        return Tipo::Torre;
    }
        Tipo::Peon
}

pub enum Color
{
    Blanco,
    Negro,
}
pub struct Pieza
{
    pub tipo: Tipo,
    pub posicion: Posicion,
    pub color:Color,
   
}
impl Pieza
{
    pub fn new(tipo:Tipo,posicion:Posicion,color:Color)-> Pieza
    {
        Pieza
        {
            tipo,
            posicion,
            color,
        }
    
    }
    pub fn puedo_moverme_desde_a(&self,destino:Posicion)->bool
        {
            match self.tipo {
                Tipo::Rey => Posicion::esta_a_una_casilla_de(self.posicion,destino),
                Tipo::Dama => Posicion::esta_en_diagonal_de(self.posicion, destino) || Posicion::esta_misma_linea_de(self.posicion, destino),
                Tipo::Alfil => Posicion::esta_en_diagonal_de(self.posicion, destino),
                Tipo::Caballo => Posicion::esta_en_l_de(self.posicion, destino),
                Tipo::Torre=> Posicion::esta_misma_linea_de(self.posicion, destino),
                Tipo::Peon=> Posicion::esta_en_frente_de(self.posicion, destino, &self.color),
        }

    }
}
}
