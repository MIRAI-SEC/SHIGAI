use notify::*;
use std::{path::Path, time::Duration};

fn main() {
    // Criação de um canal (tx, rx) para comunicação entre threads
    let (tx, rx) = std::sync::mpsc::channel();

    // Detecção do tipo de observador recomendado
    let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
        // Configuração customizada para PollWatcher
        let config = Config::default().with_poll_interval(Duration::from_secs(1));
        Box::new(PollWatcher::new(tx, config).unwrap())
    } else {
        // Usa a configuração padrão para outros tipos de observadores
        Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
    };

    // Observa o diretório atual (".") e subdiretórios de forma recursiva
    watcher
        .watch(Path::new("C:/RUSTEST"), RecursiveMode::Recursive)
        .unwrap();

    // Exibe todos os eventos observados em loop infinito (bloqueia a execução)
    for e in rx {
        println!("{:?}", e);
    }
}
