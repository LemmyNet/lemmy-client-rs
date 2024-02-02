#[macro_export]
macro_rules! impl_marker_trait {
    ($trait_name:ty, [$( $impler:ty ),+$(,)?]) => {
        $(
            impl $trait_name for $impler {}
        )*
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientOptions {
    pub domain: String,
    pub secure: bool,
}
