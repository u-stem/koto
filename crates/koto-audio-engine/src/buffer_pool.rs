//! Pre-allocated buffer pool for real-time audio processing

use koto_core::{AudioBuffer, ChannelCount};
use std::collections::VecDeque;

/// A pool of pre-allocated audio buffers to avoid allocations in the audio thread
pub struct BufferPool {
    buffers: VecDeque<AudioBuffer>,
    channels: ChannelCount,
    frames: usize,
}

impl BufferPool {
    /// Create a new buffer pool with the specified number of pre-allocated buffers
    pub fn new(num_buffers: usize, channels: ChannelCount, frames: usize) -> Self {
        let mut buffers = VecDeque::with_capacity(num_buffers);
        for _ in 0..num_buffers {
            buffers.push_back(AudioBuffer::new(channels, frames));
        }

        Self {
            buffers,
            channels,
            frames,
        }
    }

    /// Acquire a buffer from the pool
    ///
    /// Returns None if no buffers are available (should not happen in normal operation)
    pub fn acquire(&mut self) -> Option<AudioBuffer> {
        self.buffers.pop_front()
    }

    /// Release a buffer back to the pool
    pub fn release(&mut self, mut buffer: AudioBuffer) {
        buffer.clear();
        self.buffers.push_back(buffer);
    }

    /// Get the number of available buffers
    pub fn available(&self) -> usize {
        self.buffers.len()
    }

    /// Get the channel count for buffers in this pool
    pub fn channels(&self) -> ChannelCount {
        self.channels
    }

    /// Get the frame count for buffers in this pool
    pub fn frames(&self) -> usize {
        self.frames
    }
}

/// A buffer that automatically returns to the pool when dropped
pub struct PooledBuffer<'a> {
    buffer: Option<AudioBuffer>,
    pool: &'a mut BufferPool,
}

impl<'a> PooledBuffer<'a> {
    pub fn new(pool: &'a mut BufferPool) -> Option<Self> {
        let buffer = pool.acquire()?;
        Some(Self {
            buffer: Some(buffer),
            pool,
        })
    }

    pub fn buffer(&self) -> &AudioBuffer {
        self.buffer.as_ref().unwrap()
    }

    pub fn buffer_mut(&mut self) -> &mut AudioBuffer {
        self.buffer.as_mut().unwrap()
    }
}

impl Drop for PooledBuffer<'_> {
    fn drop(&mut self) {
        if let Some(buffer) = self.buffer.take() {
            self.pool.release(buffer);
        }
    }
}
