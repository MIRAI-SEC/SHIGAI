use std::error::Error;

// Importando a função para verificar os logs
mod verificar_log;



fn main() -> Result<(), Box<dyn Error>> {
    loop {
        // Chama a função para verificar o log
        verificar_log::verificar_log()?;


    }
}
