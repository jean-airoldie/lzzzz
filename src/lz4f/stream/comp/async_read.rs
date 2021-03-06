#![cfg(feature = "async-io")]

use super::{AsyncBufReadCompressor, Dictionary, Preferences};
use crate::lz4f::Result;
use async_std::io::BufReader;
use futures_lite::AsyncRead;
use pin_project::pin_project;
use std::{
    fmt, io,
    pin::Pin,
    task::{Context, Poll},
};

/// The [`AsyncRead`]-based streaming compressor.
///
/// # Example
///
/// ```
/// # use std::env;
/// # use std::path::Path;
/// # use lzzzz::{Error, Result};
/// # use assert_fs::prelude::*;
/// # let tmp_dir = assert_fs::TempDir::new().unwrap().into_persistent();
/// # env::set_current_dir(tmp_dir.path()).unwrap();
/// #
/// # tmp_dir.child("foo.txt").write_str("Hello").unwrap();
/// #
/// # smol::run(async {
/// use async_std::{fs::File, prelude::*};
/// use lzzzz::lz4f::AsyncReadCompressor;
///
/// let mut f = File::open("foo.txt").await?;
/// let mut r = AsyncReadCompressor::new(&mut f, Default::default())?;
///
/// let mut buf = Vec::new();
/// r.read_to_end(&mut buf).await?;
/// # Ok::<(), std::io::Error>(())
/// # }).unwrap();
/// # tmp_dir.close().unwrap();
/// ```
///
/// [`AsyncRead`]: https://docs.rs/futures-io/0.3.5/futures_io/trait.AsyncRead.html

#[cfg_attr(docsrs, doc(cfg(feature = "async-io")))]
#[pin_project]
pub struct AsyncReadCompressor<R: AsyncRead + Unpin> {
    #[pin]
    inner: AsyncBufReadCompressor<BufReader<R>>,
}

impl<R: AsyncRead + Unpin> AsyncReadCompressor<R> {
    /// Creates a new `AsyncBufReadCompressor<R>`.
    pub fn new(reader: R, prefs: Preferences) -> Result<Self> {
        Ok(Self {
            inner: AsyncBufReadCompressor::new(BufReader::new(reader), prefs)?,
        })
    }

    /// Creates a new `AsyncBufReadCompressor<R>` with a dictionary.
    pub fn with_dict(reader: R, prefs: Preferences, dict: Dictionary) -> Result<Self> {
        Ok(Self {
            inner: AsyncBufReadCompressor::with_dict(BufReader::new(reader), prefs, dict)?,
        })
    }

    /// Returns a mutable reference to the reader.
    pub fn get_mut(&mut self) -> &mut R {
        self.inner.get_mut().get_mut()
    }

    /// Returns a shared reference to the reader.
    pub fn get_ref(&self) -> &R {
        self.inner.get_ref().get_ref()
    }

    /// Returns ownership of the reader.
    pub fn into_inner(self) -> R {
        self.inner.into_inner().into_inner()
    }
}

impl<R> fmt::Debug for AsyncReadCompressor<R>
where
    R: AsyncRead + Unpin + fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("AsyncReadCompressor")
            .field("reader", &self.inner.inner.get_ref())
            .field("prefs", &self.inner.comp.prefs())
            .finish()
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for AsyncReadCompressor<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        self.project().inner.poll_read(cx, buf)
    }
}
