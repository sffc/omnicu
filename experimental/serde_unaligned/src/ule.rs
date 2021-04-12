pub trait ULE
where
    Self: Sized,
{
    type Error;

    fn parse_bytes(bytes: &[u8]) -> Result<&[Self], Self::Error>;
    fn as_bytes(slice: &[Self]) -> &[u8];
}

pub trait AsULE {
    type ULE: ULE;

    fn as_unaligned(&self) -> Self::ULE;
    fn from_unaligned(unaligned: &Self::ULE) -> Self;
}