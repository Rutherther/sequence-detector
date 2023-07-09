use std::{path::PathBuf, time::Duration, io, thread};

use clap::Parser;
use sequence_cacher::{SequenceCacher, CacheError, SequenceFile};
use sequence_detector::{SequenceDetector, HandleResult};
use settings::Settings;

pub mod sequence_detector;
pub mod settings;
pub mod sequence_cacher;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 'c', long)]
    pub config: Option<PathBuf>,
    #[arg(short = 'd', long)]
    pub debounce_time: Option<u64>,
    #[arg(short = 'g', long, default_value = "default")]
    pub group_id: String,
    #[arg(short = 'f', long, default_value = r"/tmp/{group}.seq_dect")]
    pub sequence_file: PathBuf,
    #[arg(help = "The key to append to the sequence")]
    pub key: String,
}

fn main() {
    let args = Cli::parse();
    let settings = match Settings::new(&args.config, "config.json") {
        Ok(settings) => settings,
        Err(err) => {
            eprintln!(
                "Could not open the settings file. {}",
                err
            );
            return;
        }
    };

    let debounce_time = Duration::from_millis(args.debounce_time.unwrap_or(settings.debounce_time));

    let group = match settings.groups.iter().find(|x| x.group_id == args.group_id) {
        Some(group) => group,
        None => {
            eprintln!("There is no group with the id {} you given.", args.group_id);
            return;
        }
    };

    let mut cacher = SequenceCacher::new(&args.sequence_file, &args.group_id);
    let matcher = SequenceDetector::new(group.sequences.clone());

    let current_sequence = match cacher.try_load(debounce_time) {
        Ok(sequence) => sequence,
        Err(err) => {
            match err {
                CacheError::Expired => (),
                CacheError::IO(err) if err.kind() == io::ErrorKind::NotFound => (),
                _ => eprintln!("Could not load from cache: {}", err)
            };
            SequenceFile::empty()
        }
    };

    let mut handle_result = matcher.handle_next(current_sequence.keys(), &args.key);

    if let HandleResult::Debounce(sequence) = handle_result {
        let mut keys = current_sequence.keys().clone();
        keys.push(args.key);
        if let Err(err) = cacher.try_cache(keys) {
            eprintln!("Could not save cache for debounce, aborting. {}", err);
            return;
        }

        thread::sleep(debounce_time);

        handle_result = match cacher.modified() {
            Ok(modified) if !modified => HandleResult::Execute(sequence),
            Err(err) => {
                eprintln!("Could not check whether the cache is modified. {}", err);
                HandleResult::Exit
            },
            _ => HandleResult::Exit,
        }
    }

    match handle_result {
        HandleResult::Execute(sequence) => {
            if let Err(err) = sequence.execute() {
                eprintln!("Could not execute the action. {}", err);
            } else {
                println!("Found one matching sequence and executed.");
                println!("{:?}", sequence);
            }

            if let Err(err) = cacher.remove() {
                eprintln!("Could not remove the cache. {}", err);
            }
        },
        HandleResult::Exit => (),
        _ => panic!("Unreachable, debounce handled") // debounce already handled
    };
}
