//! Session-based orchestration for `@` file searches.
//!
//! `ChatComposer` publishes every change of the `@token` as
//! `AppEvent::StartFileSearch(query)`. This manager owns a single
//! `codex-file-search` session for the current search REDACTED_SECRET, updates the query
//! on every keystroke, and drops the session when the query becomes empty.

use codex_file_search as file_search;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::app_event::AppEvent;
use crate::app_event_sender::AppEventREDACTED_SECRETer;

pub(crate) struct FileSearchManager {
    state: Arc<Mutex<SearchState>>,
    search_dir: PathBuf,
    app_tx: AppEventREDACTED_SECRETer,
}

struct SearchState {
    latest_query: String,
    session: Option<file_search::FileSearchSession>,
    session_token: usize,
}

impl FileSearchManager {
    pub fn new(search_dir: PathBuf, tx: AppEventREDACTED_SECRETer) -> Self {
        Self {
            state: Arc:REDACTED_SECRETMutex:REDACTED_SECRETSearchState {
                latest_query: String:REDACTED_SECRET),
                session: None,
                session_token: 0,
            })),
            search_dir,
            app_tx: tx,
        }
    }

    /// Updates the directory used for file searches.
    /// This should be called when the session's CWD changes on resume.
    /// Drops the current session so it will be recreated with the new directory on next query.
    pub fn update_search_dir(&mut self, new_dir: PathBuf) {
        self.search_dir = new_dir;
        #[expect(clippy::unwrap_used)]
        let mut st = self.state.lock().unwrap();
        st.session.take();
        st.latest_query.clear();
    }

    /// Call whenever the user edits the `@` token.
    pub fn on_user_query(&self, query: String) {
        #[expect(clippy::unwrap_used)]
        let mut st = self.state.lock().unwrap();
        if query == st.latest_query {
            return;
        }
        st.latest_query.clear();
        st.latest_query.push_str(&query);

        if query.is_empty() {
            st.session.take();
            return;
        }

        if st.session.is_none() {
            self.start_session_locked(&mut st);
        }
        if let Some(session) = st.session.as_ref() {
            session.update_query(&query);
        }
    }

    fn start_session_locked(&self, st: &mut SearchState) {
        st.session_token = st.session_token.wrapping_add(1);
        let session_token = st.session_token;
        let reporter = Arc:REDACTED_SECRETTuiSessionReporter {
            state: self.state.clone(),
            app_tx: self.app_tx.clone(),
            session_token,
        });
        let session = file_search::create_session(
            vec![self.search_dir.clone()],
            file_search::FileSearchOptions {
                compute_indices: true,
                ..Default::default()
            },
            reporter,
            /*cancel_flag*/ None,
        );
        match session {
            Ok(session) => st.session = Some(session),
            Err(err) => {
                tracing::warn!("file search session failed to start: {err}");
                st.session = None;
            }
        }
    }
}

struct TuiSessionReporter {
    state: Arc<Mutex<SearchState>>,
    app_tx: AppEventREDACTED_SECRETer,
    session_token: usize,
}

impl TuiSessionReporter {
    fn send_snapshot(&self, snapshot: &file_search::FileSearchSnapshot) {
        #[expect(clippy::unwrap_used)]
        let st = self.state.lock().unwrap();
        if st.session_token != self.session_token
            || st.latest_query.is_empty()
            || snapshot.query.is_empty()
        {
            return;
        }
        let query = snapshot.query.clone();
        drop(st);
        self.app_tx.send(AppEvent::FileSearchResult {
            query,
            matches: snapshot.matches.clone(),
        });
    }
}

impl file_search::SessionReporter for TuiSessionReporter {
    fn on_update(&self, snapshot: &file_search::FileSearchSnapshot) {
        self.send_snapshot(snapshot);
    }

    fn on_complete(&self) {}
}
