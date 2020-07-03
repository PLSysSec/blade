#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum BladeType {
    None,
    Baseline,
    Lfence,
    LfencePerBlock,
    SLH,
}
