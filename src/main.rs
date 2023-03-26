use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
pub struct Posicion
{
    x: usize,
    y:usize,
}
pub struct Tablero
{
    tabla: Vec<Vec<String>>,
}
impl Tablero
    {
        pub fn crear_tabla(nombre_archivo:String)-> Vec<Vec<String>>
    {
        
        let file = File::open(nombre_archivo).expect("Failed to open file :(");
        let reader = BufReader::new(file);

    
    let mut tabla: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        let fila: Vec<String> = line.unwrap()
            .split_whitespace()
            .map(|x| String::from(x))
            .collect();
        tabla.push(fila);
        
    }
    tabla
    }
        fn new(archivo: String)->Tablero
        {
            let tabla = Self::crear_tabla(archivo);
           
            Tablero
            {
                tabla,

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


        pub fn buscar_pieza<F>(&self,condicion:F,color:&str)->Result<(Posicion,char),String>
            where F: Fn(&char)->bool
        {
            let (mut fila, mut columna) = (0,0);
            let mut encontrado = false;
            let mut identificador:char = '0';
            
            for row in &self.tabla
        {
        
            for casillero in row
            {

                let contenido = casillero.chars().next();
                match contenido {
                    Some(contenido) => 
                    {
                        
                        if contenido != '_' && condicion(&contenido)
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
        if encontrado && identificador != '0' 
        {
            let pos_pieza = Posicion{
                x: fila,
                y:columna,
            };
            Ok((pos_pieza,identificador))
        }
        else
        {
            Err(String::from("No se encontro ninguna pieza en el tablero de color ") + color + ".\n")
        }


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
    if *identificador == 'r'  || *identificador == 'R'
    {
        return Tipo::Rey;
    }
    if *identificador == 'd'  || *identificador == 'D'
    {
        return Tipo::Dama;

    }
    if *identificador == 'a'  || *identificador == 'A'
    {
        return Tipo::Alfil; 
    }
    if *identificador == 'c'  || *identificador == 'C'
    {
        return Tipo::Caballo;
    }
    if *identificador == 't'  || *identificador == 'T'
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
                Tipo::Rey => self.posicion.x == destino.x +1 || self.posicion.x == destino.x -1 || self.posicion.y == destino.y +1 || self.posicion.y == destino.y -1 || self.posicion.x == destino.x ,
                Tipo::Dama => true,
                Tipo::Alfil => true,
                Tipo::Caballo => true,
                Tipo::Torre=> true,
                Tipo::Peon=> 
                {
                    match self.color
                    {
                        Color::Blanco => true,
                        Color::Negro => true,
                    }
                
                }

        }

    }
}


fn main() 
{
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let t = Tablero::new(String::from(file_path));

    
    
    match Tablero::buscar_pieza(&t,|identificador_blanca|Tablero::es_blanca(identificador_blanca),"blanca")
    {
        Ok((pos_blanca,identificador)) => 
        {
            let pieza_blanca = Pieza::new(definir_tipo(&identificador),pos_blanca,Color::Blanco);
            println!("La pieza blanca es un {:?} y esta en {},{}",pieza_blanca.tipo,pieza_blanca.posicion.x,pieza_blanca.posicion.y);
        },
        Err(mensaje) => println!("{}",mensaje),
    }

    match Tablero::buscar_pieza(&t,|identificador_negra|Tablero::es_negra(identificador_negra),"negra")
    {
        Ok((pos_negra,identificador)) => 
        {
            let pieza_negra = Pieza::new(definir_tipo(&identificador),pos_negra,Color::Negro);
            println!("La pieza negra es un {:?} y esta en {},{}",pieza_negra.tipo,pieza_negra.posicion.x,pieza_negra.posicion.y);
        }
        Err(mensaje) => println!("{}",mensaje),
    }


    
    


}
