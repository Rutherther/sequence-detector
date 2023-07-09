use crate::settings::Sequence;

pub enum HandleResult {
    Execute(Sequence),
    Debounce(Sequence),
    Exit
}

pub struct SequenceDetector {
    sequences: Vec<Sequence>
}

impl SequenceDetector {
    pub fn new(sequences: Vec<Sequence>) -> Self {
        Self {
            sequences
        }
    }

    pub fn match_sequences(&self, keys: &Vec<String>) -> Vec<&Sequence> {
        let mut matched_sequences: Vec<&Sequence> = Vec::new();

        for sequence in &self.sequences {
            if sequence.keys.len() < keys.len() {
                continue;
            }

            let mut matches = true;
            for (i, key) in keys.iter().enumerate() {
                let match_key = &sequence.keys[i];

                if match_key != key {
                    matches = false;
                    break;
                }
            }

            if matches {
                matched_sequences.push(sequence);
            }
        }

        matched_sequences.sort_by(|&x, &y| x.keys.len().cmp(&y.keys.len()));
        matched_sequences
    }

    pub fn handle_next(&self, current_keys: &Vec<String>, key: &str) -> HandleResult {
        let mut keys = current_keys.clone();
        keys.push(key.to_string());

        let matched = self.match_sequences(&keys);

        match matched.len() {
            0 => HandleResult::Exit,
            1 => HandleResult::Execute(matched[0].clone()),
            _ => HandleResult::Debounce(matched[0].clone())
        }
    }
}
