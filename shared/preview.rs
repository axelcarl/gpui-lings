//! Small, atomic file messages between the guide and its native preview.
use std::{fs, io, path::Path};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refresh {
    Current,
    Checking,
    Building,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewState {
    pub index: usize,
    pub status: String,
    pub completed: usize,
    pub refresh: Refresh,
}

impl PreviewState {
    #[allow(dead_code)] // Used by the playground; the guide only writes.
    pub fn read(path: &Path) -> Option<Self> {
        let text = fs::read_to_string(path).ok()?;
        let mut lines = text.lines();
        let index = lines.next()?.parse().ok()?;
        let status = lines.next()?.to_owned();
        if !matches!(status.as_str(), "" | "passed" | "failed" | "error") {
            return None;
        }
        let refresh = match lines.next()? {
            "current" => Refresh::Current,
            "checking" => Refresh::Checking,
            "building" => Refresh::Building,
            "failed" => Refresh::Failed,
            _ => return None,
        };
        let completed = lines.next()?.parse().ok()?;
        Some(Self {
            index,
            status,
            completed,
            refresh,
        })
    }

    #[allow(dead_code)] // Used by the guide; the playground only reads.
    pub fn write(&self, path: &Path) -> io::Result<()> {
        let refresh = match self.refresh {
            Refresh::Current => "current",
            Refresh::Checking => "checking",
            Refresh::Building => "building",
            Refresh::Failed => "failed",
        };
        let temporary = path.with_extension("tmp");
        fs::write(
            &temporary,
            format!(
                "{}\n{}\n{refresh}\n{}\n",
                self.index, self.status, self.completed
            ),
        )?;
        fs::rename(temporary, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_round_trip_and_incomplete_messages() {
        let path = std::env::temp_dir().join(format!("gpui-preview-state-{}", std::process::id()));
        for refresh in [
            Refresh::Current,
            Refresh::Checking,
            Refresh::Building,
            Refresh::Failed,
        ] {
            for status in ["", "passed", "failed", "error"] {
                let state = PreviewState {
                    index: 7,
                    status: status.into(),
                    completed: 4,
                    refresh,
                };
                state.write(&path).unwrap();
                assert_eq!(PreviewState::read(&path), Some(state));
            }
        }
        fs::write(&path, "7\npassed\n").unwrap();
        assert_eq!(PreviewState::read(&path), None);
        fs::remove_file(path).unwrap();
    }
}
