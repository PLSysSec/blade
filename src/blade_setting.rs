#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum BladeType {
    None,
    BaselineFence,
    BaselineSLH,
    Lfence,
    LfencePerBlock,
    SLH,
}
