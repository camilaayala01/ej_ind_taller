

pub(crate) mod tablero
{
  
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    use ej_ind_ajedrez::pieza::pieza::{Pieza, Color, definir_tipo};
    use ej_ind_ajedrez::posicion::posicion::Posicion;
    
    pub fn crear_tabla(nombre_archivo:String)-> Result<Vec<Vec<String>>,String>
    {
        
        let identificadores_blancos = "RDACTP";
        let identificadores_negros = "rdactp";
        match File::open(nombre_archivo)
        {
            Ok(file) =>
            {
                
                let reader = BufReader::new(file);
                let mut tabla: Vec<Vec<String>> = Vec::new();
                let mut cantidad_filas = 0;
                let mut cantidad_blancas = 0;
                let mut cantidad_negras = 0;
                for line in reader.lines() 
                { 
                    match line
                    {
                        Ok(linea) =>
                        {
                            if linea.chars().count() > 16 // 8 caracteres, 8 espacios y un \n 
                            {
                                return Err(String::from("formato incorrecto en archivo."));
                            }
                            if linea.chars().count() > 1 && linea != String::from("\n")//admito un \n
                            {
                                cantidad_filas = cantidad_filas + 1;
                            }
                            cantidad_blancas = cantidad_blancas  + linea.chars().filter(|&c| identificadores_blancos.contains(c)).count();
                            cantidad_negras = cantidad_negras  + linea.chars().filter(|&c| identificadores_negros.contains(c)).count();
                            
                            
                            tabla.push(
                                linea
                                .split_whitespace()
                                .map(|x| String::from(x))
                                .collect()
                            );
                        }
                        Err(_) => return Err(String::from("no se pudo leer la linea."))
                    };
                }
                if cantidad_filas != 8 || cantidad_blancas !=1 || cantidad_negras !=1
                {
                    return Err(String::from("formato incorrecto archivo "));
                }
                return Ok(tabla);
            }
        Err(_) => return Err(String::from("No se pudo abrir su archivo."))
        } 
        
    }
pub struct Tablero
{
    pub pieza_blanca :Pieza,
    pub pieza_negra:Pieza,
}
impl Tablero
    {
        pub fn new(tabla:&Vec<Vec<String>>)->Result<Tablero,String>
        {
            let pieza_blanca:Pieza;
            let pieza_negra:Pieza;
            match Tablero::buscar_pieza(&tabla,|identificador_blanca|Tablero::es_blanca(identificador_blanca),Color::Blanco) 
            {
                Some(b) => pieza_blanca = b,
                None => 
                {return Err(String::from("Error al leer tablero."));}
            }
            match Tablero::buscar_pieza(&tabla,|identificador_negra|Tablero::es_negra(identificador_negra),Color::Negro)
            {
                Some(n) => pieza_negra= n,
                None => {return Err(String::from("Error al leer tablero."));}
            }
        
            Ok(Tablero { pieza_blanca,pieza_negra,})
            
            
        }
        pub fn es_blanca(identificador:&char)->bool
        {
            let identificadores_blancos = "RDACTP";
            identificadores_blancos.contains(*identificador)
        }

        pub fn es_negra(identificador:&char)->bool
        {
            let identificadores_negros = "rdactp";
            identificadores_negros.contains(*identificador)
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
                match contenido
                {
                    Some(c) => 
                    { 
                        if condicion(&c)
                        {
                            identificador = c;
                            encontrado = true;
                            break;
                        }
                        columna = columna +1;
                    },
                    None => break,
                }
            }
            if encontrado 
            {
                break;
            }
            columna = 0;
            fila = fila+1;
        }
        if !encontrado {
            return None
        }
        let pos_pieza = Posicion{
            x: fila,
            y:columna,
        };

        Some(Pieza::new(definir_tipo(&identificador), pos_pieza, color))


        }
     
    }
}