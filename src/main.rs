
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;


#[derive(Clone)]
pub struct Posicion
{
    x: usize,
    y:usize,
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
            Color::Blanco => origen.x == destino.x && origen.y == destino.y +1,
            Color::Negro => origen.x == destino.x && origen.y == destino.y -1,
        
        }
    }
    
    
}
pub fn crear_tabla(nombre_archivo:String)-> Vec<Vec<String>>
    {
        
        let file = File::open(nombre_archivo).expect("Failed to open file :(");
        let reader = BufReader::new(file);

    
        let mut tabla: Vec<Vec<String>> = Vec::new();
        for line in reader.lines() 
        {
            let fila: Vec<String> = line.unwrap()
                .split_whitespace()
                .map(|x| String::from(x))
                .collect();
            tabla.push(fila);
        
        }
        tabla
    }
pub struct Tablero
{
    pieza_blanca :Pieza,
    pieza_negra:Pieza,
}
impl Tablero
    {
        fn new(tabla:&Vec<Vec<String>>)->Result<Tablero,String>
        {
            if Tablero::buscar_pieza(&tabla,|identificador_blanca|Tablero::es_blanca(identificador_blanca),Color::Blanco).is_none()
            {
                Err(String::from("No encontre blanca.\n"))
            }
            else if Tablero::buscar_pieza(&tabla,|identificador_negra|Tablero::es_negra(identificador_negra),Color::Negro).is_none()
            {
                Err(String::from("No encontre negra.\n"))
            }
            else 
            {
                Ok(
                    Tablero
                    {
                        pieza_blanca: Tablero::buscar_pieza(&tabla,|identificador_blanca|Tablero::es_blanca(identificador_blanca),Color::Blanco).unwrap(),
                        pieza_negra : Tablero::buscar_pieza(&tabla,|identificador_negra|Tablero::es_negra(identificador_negra),Color::Negro).unwrap(),
                    }
                )
            }
            
            
        }
        pub fn es_blanca(identificador:&char)->bool
        {
            *identificador == 'r' ||*identificador == 'd' || *identificador == 'p' || *identificador == 'a' || *identificador == 't' || *identificador == 'c'
        }

        pub fn es_negra(identificador:&char)->bool
        {
            *identificador == 'R' || *identificador == 'D' || *identificador == 'P' || *identificador == 'A' || *identificador == 'T' || *identificador == 'C'
        }


        pub fn buscar_pieza<F>(tabla:&Vec<Vec<String>>,condicion:F,color:Color)->Option<Pieza>
            where F: Fn(&char)->bool
        {
            let (mut fila, mut columna) = (0,0);
            let mut encontrado = false;
            let mut identificador:char = '0';
            
            for row in tabla
        {
        
            for casillero in row
            {

                let contenido = casillero.chars().next();
                match contenido {
                    Some(contenido) => 
                    {
                        
                        if condicion(&contenido)
                        {
                            identificador = contenido;
                            encontrado = true;
                            break;
                        }
                            columna = columna +1;
            
                    },
                    None => print!("Error en formato del archivo. \n"),
                }
            }
            if encontrado 
                {
                    break;
                }
            columna = 0;
            fila = fila+1;
        }
        if !encontrado
        {
            return None;
        } 

        let pos_pieza = Posicion{
            x: fila,
            y:columna,
        };

        Some(Pieza::new(definir_tipo(&identificador), pos_pieza, color))


        }
     
    }

#[derive(Debug)]
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
    tipo: Tipo,
    posicion: Posicion,
    color:Color,
   
}
impl Pieza
{
    fn new(tipo:Tipo,posicion:Posicion,color:Color)-> Pieza
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


fn main() 
{
    let args: Vec<String> = env::args().collect();

    let archivo = &args[1];

    let tabla = 
    crear_tabla(archivo.to_string());
    
    match Tablero::new(&tabla)
    {
        Ok(t)=> 
        {
            let puede_comer_blanca:bool = Pieza::puedo_moverme_desde_a(&t.pieza_blanca,t.pieza_negra.posicion);
            let puede_comer_negra:bool = Pieza::puedo_moverme_desde_a(&t.pieza_negra, t.pieza_blanca.posicion);

        if puede_comer_blanca && puede_comer_negra
        {
            println!("empate\n");
        }
    else if puede_comer_blanca && !puede_comer_negra
    {
        println!("gana blanca.\n");
    }
    else if puede_comer_negra && !puede_comer_blanca
    {
        println!("gana  negra.\n");
    }
    else {
        println!("ambos pierden.\n");
    }

        }
        Err(mensaje)=>
        {
            println!("{}",mensaje);
            return;
        }
    }

    

    
    


}
