
// Importando crate da winapi
extern crate winapi;

// Importando módulos da winapi
use winapi::um::winnt::{ PROCESS_TERMINATE, PROCESS_QUERY_INFORMATION};
use winapi::um::processthreadsapi::{TerminateProcess, OpenProcess,};

// Importando módulos da biblioteca padrão
use std::{fs::File, io::Write, sync::{Arc, Mutex}, collections::HashSet};
use evtx::EvtxParser;
use std::error::Error;
use rayon::prelude::*;

// Importando módulo de ponteiro da biblioteca padrão
use std::ptr::null_mut;


// Função para verificar os logs que foram gerados pelo Sysmon
pub fn verificar_log() -> Result<(), Box<dyn Error>> {

    // Caminho padrão do arquivo de log do Sysmon
    let caminho = "C:\\Windows\\System32\\winevt\\Logs\\Microsoft-Windows-Sysmon%4Operational.evtx";

    // Gerar um analisador para abrir os logs do {caminho}
    let mut analisador = EvtxParser::from_path(caminho)?;

    // O programa realiza a obtenção dos dados relativos ao registro dos logs
    let mut registros = analisador.records();

    // Cria um arquivo para a saída do programa
    let arquivo_saida = Arc::new(Mutex::new(File::create("log.txt")?));

    // Define uma lista de processos seguros para que sejam excluidos de serm mortos
    let processos_seguros: HashSet<&str> = [
        "System", "smss.exe", "csrss.exe", "wininit.exe", "services.exe",
        "lsass.exe", "lsm.exe", "winlogon.exe", "svchost.exe", "spoolsv.exe",
        "explorer.exe", "taskmgr.exe", "audiodg.exe", "dwm.exe",
        "chrome.exe", "firefox.exe", "edge.exe", "iexplore.exe",
        "conhost.exe", "SearchIndexer.exe", "SearchProtocolHost.exe",
        "WINWORD.EXE", "EXCEL.EXE", "POWERPNT.EXE",
        "devenv.exe",
        "shigai-sysmon.exe"
    ].iter().cloned().collect();

    // Faz com que os registros do log sejam processados em paralelo
    registros.par_bridge().for_each(|registro| {
        let registro = registro.unwrap();
        let mut arquivo_saida = arquivo_saida.lock().unwrap();

        // Verifica se o Evento é o 11, o evento 11 é o evento de criação de arquivos no Sysmon
        if let Some(inicio) = registro.data.find("<EventID>") {
            let inicio = inicio + "<EventID>".len();

            if let Some(fim) = registro.data[inicio..].find("</EventID>") {
                // Aqui ele verifica se o evento é o 11, caso não seja ele passa para o próximo
                if &registro.data[inicio..inicio + fim] != "11" {
                    return;
                }

                writeln!(arquivo_saida, "Detectado EventID: {}", &registro.data[inicio..inicio + fim]).unwrap();

                // Extrai o caminho da imagem do registro
                if let Some(inicio) = registro.data.find("<Data Name=\"Image\">") {
                    let inicio = inicio + "<Data Name=\"Image\">".len();
                    if let Some(fim) = registro.data[inicio..].find("</Data>") {
                        let image_path = &registro.data[inicio..inicio + fim];
                        writeln!(arquivo_saida, "Caminho da Imagem: {}", image_path).unwrap();

                        // Verifica se a imagem é um processo seguro
                        if !processos_seguros.contains(&image_path.rsplit('\\').next().unwrap()) {

                            // Extrai o diretório que foi criado
                            if let Some(inicio) = registro.data.find("<Data Name=\"TargetFilename\">") {
                                let inicio = inicio + "<Data Name=\"TargetFilename\">".len();
                                if let Some(fim) = registro.data[inicio..].find("</Data>") {
                                    let target_filename = &registro.data[inicio..inicio + fim];
                                    writeln!(arquivo_saida, "Diretório Criado: {}", target_filename).unwrap();

                                    // Extrai o ID do processo
                                    if let Some(inicio) = registro.data.find("<Data Name=\"ProcessId\">") {
                                        let inicio = inicio + "<Data Name=\"ProcessId\">".len();
                                        if let Some(fim) = registro.data[inicio..].find("</Data>") {
                                            let process_id: u32 = registro.data[inicio..inicio + fim].parse().unwrap();
                                            writeln!(arquivo_saida, "Matando o processo: {}", process_id).unwrap();

                                            // Chama a função para matar o processo
                                            matar_processo(process_id);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    Ok(())
}

// Função responsavel por abrir e matar o processo, a função é setada com unsafe pois é uma função da api Windows
fn matar_processo(process_id: u32) {

    // Tenta abrir o processo

    let handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_TERMINATE, 0, process_id) };

    if handle != null_mut() {

        // Se o processo puder ser aberto, tenta matá-lo, a função é setada com unsafe pois é uma função da api Windows

        unsafe { TerminateProcess(handle, 0) };
        println!("Processo {} foi morto.", process_id);
    }
}

