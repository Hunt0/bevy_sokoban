#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    LOADING,
    PAUSED,
    PLAYING,
}
