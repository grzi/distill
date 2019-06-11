use crate::error::Error;

pub fn make_array<A, T>(slice: &[T]) -> A
where
    A: Sized + Default + AsMut<[T]>,
    T: Copy,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
    a
}

pub fn uuid_from_slice(slice: &[u8]) -> Result<uuid::Bytes, Error> {
    const BYTES_LEN: usize = 16;

    let len = slice.len();

    if len != BYTES_LEN {
        return Err(Error::UuidBytesError(uuid::BytesError::new(BYTES_LEN, len)));
    }

    let mut bytes: uuid::Bytes = [0; 16];
    bytes.copy_from_slice(slice);
    Ok(bytes)
}