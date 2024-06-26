use daemonize::Daemonize;
use gnss_share::cmdline_config;
use gnss_share::config::Config;
use gnss_share::server::Server;
use signal_hook::consts as signals;
use std::io;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

enum DoneReason {
    Signal(i32),
    Success,
}

/// Stolen directly from crate chan-signal.
fn notify(signals: &[i32], s: mpsc::Sender<DoneReason>) -> Result<(), io::Error> {
    let mut signals = signal_hook::iterator::Signals::new(signals)?;
    thread::spawn(move || {
        for signal in signals.forever() {
            if s.send(DoneReason::Signal(signal)).is_err() {
                break;
            }
        }
    });
    Ok(())
}

fn main() {
    env_logger::init();
    let config = cmdline_config::config_from_cmdline();

    if config.daemonize {
        let daemonize = Daemonize::new();
        match daemonize.start() {
            Ok(_) => println!("Daemonized!"),
            Err(e) => eprintln!("Error, {}", e),
        }
    }

    let (sdone, rdone) = mpsc::channel();
    notify(&[signals::SIGINT, signals::SIGTERM], sdone.clone()).unwrap();

    thread::spawn(move || run(sdone, Rc::new(config)));

    match rdone.recv().unwrap() {
        DoneReason::Signal(signals::SIGINT) => {
            println!("Interrupt from keyboard. Exitting..");
        }
        DoneReason::Signal(signals::SIGTERM) => {
            println!("Kill signal received. Exitting..");
        }
        DoneReason::Signal(_) => (),
        DoneReason::Success => {
            println!("Program completed normally.");
        }
    };
}

fn run(sdone: mpsc::Sender<DoneReason>, config: Rc<Config>) {
    let mut server = Server::new(config.clone()).unwrap();
    server.run();
    sdone.send(DoneReason::Success).unwrap();
}
