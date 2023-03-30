
pub mod tablero;
use ej_ind_ajedrez::pieza::pieza::Pieza;
use tablero::tablero::Tablero;
use tablero::tablero::crear_tabla;
use std::env;
fn main() 
{
    let args: Vec<String> = env::args().collect();

    let archivo = &args[1];

    match crear_tabla(archivo.to_string())
    {
        Ok(tabla) => 
        {
                match Tablero::new(&tabla)
            {
                Ok(t)=> 
                {
                    let puede_comer_blanca:bool = Pieza::puedo_moverme_desde_a(&t.pieza_blanca,t.pieza_negra.posicion);
                    let puede_comer_negra:bool = Pieza::puedo_moverme_desde_a(&t.pieza_negra, t.pieza_blanca.posicion);

                    if puede_comer_blanca && puede_comer_negra
                    {
                        println!("empate.");
                    }
                    else if puede_comer_blanca && !puede_comer_negra
                    {
                        println!("gana blanca.");
                    }
                    else if puede_comer_negra && !puede_comer_blanca
                    {
                        println!("gana  negra.");
                    }
                    else {
                        println!("ambos pierden.");
                    }   

                }
                Err(mensaje)=>
                {
                    println!("{}",mensaje);
                    return;
                }
            }
       
        }
        
        Err(mensaje) =>
        {
            println!("{}",mensaje);
            return;
        }
    
    
    }
}
