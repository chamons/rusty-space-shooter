use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use anyhow::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};

pub struct FileWatcher {
    _debouncer: Debouncer<RecommendedWatcher, FileIdMap>,
    changed: Arc<AtomicBool>,
}

impl FileWatcher {
    pub fn new(file: PathBuf) -> Result<FileWatcher> {
        let changed = Arc::new(AtomicBool::new(false));

        let mut debouncer = {
            let changed = changed.clone();
            new_debouncer(
                Duration::from_millis(200),
                None,
                move |_: DebounceEventResult| changed.store(true, Ordering::SeqCst),
            )?
        };

        debouncer
            .watcher()
            .watch(&file, RecursiveMode::NonRecursive)?;

        debouncer
            .cache()
            .add_root(&file, RecursiveMode::NonRecursive);

        Ok(FileWatcher {
            _debouncer: debouncer,
            changed,
        })
    }

    pub fn changed(&self) -> bool {
        self.changed.fetch_and(false, Ordering::SeqCst)
    }
}
