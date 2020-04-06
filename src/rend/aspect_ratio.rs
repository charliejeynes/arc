//! Aspect-ratio implementation.

use crate::rend::settings::Quality;
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Aspect-ratio enumeration.
#[json]
#[derive(Clone)]
pub enum AspectRatio {
    /// Square. 1:1.
    Square,
    /// Classic photographic film. 3:2.
    Classic,
    /// Standard. 16:9
    Standard,
    /// Widescreen. 43:18
    Widescreen,
    /// IPhone XS. 375:812 (1125 x 2436)
    IPhoneXS,
}

impl AspectRatio {
    /// Get the pixel ratios.
    #[inline]
    #[must_use]
    pub fn ratio(&self) -> (usize, usize) {
        match self {
            Self::Square => (1, 1),
            Self::Classic => (3, 2),
            Self::Standard => (16, 9),
            Self::Widescreen => (43, 18),
            Self::IPhoneXS => (375, 812),
        }
    }

    /// Generate a resolution for a given quality level.
    #[inline]
    #[must_use]
    pub fn gen_resolution(&self, quality: Quality) -> (usize, usize) {
        match quality {
            Quality::Super => self.super_res(),
            Quality::High => self.high_res(),
            Quality::Medium => self.medium_res(),
            Quality::Low => self.low_res(),
            Quality::Potato => self.potato_res(),
        }
    }

    /// Create a resolution.
    #[inline]
    #[must_use]
    pub fn resolution(&self, hx: usize) -> (usize, usize) {
        debug_assert!(hx % self.ratio().0 == 0);

        let (rx, ry) = self.ratio();
        (rx * hx, ry * hx)
    }

    /// Generate a super resolution.
    #[inline]
    #[must_use]
    pub fn super_res(&self) -> (usize, usize) {
        self.resolution(match self {
            Self::Square => 4024,
            Self::Classic => 2048,
            Self::Standard => 480,
            Self::Widescreen => 190,
            Self::IPhoneXS => 9,
        })
    }

    /// Generate a full quality resolution.
    #[inline]
    #[must_use]
    pub fn high_res(&self) -> (usize, usize) {
        self.resolution(match self {
            Self::Square => 2048,
            Self::Classic => 1024,
            Self::Standard => 240,
            Self::Widescreen => 80,
            Self::IPhoneXS => 3,
        })
    }

    /// Generate a half-resolution (in each axis) quality resolution.
    #[inline]
    #[must_use]
    pub fn medium_res(&self) -> (usize, usize) {
        self.resolution(match self {
            Self::Square => 1024,
            Self::Classic => 512,
            Self::Standard => 120,
            Self::Widescreen => 40,
            Self::IPhoneXS => 2,
        })
    }

    /// Generate a low quality resolution.
    #[inline]
    #[must_use]
    pub fn low_res(&self) -> (usize, usize) {
        self.resolution(match self {
            Self::Square => 512,
            Self::Classic => 256,
            Self::Standard => 60,
            Self::Widescreen => 20,
            Self::IPhoneXS => 1,
        })
    }

    /// Generate a potato quality resolution.
    #[inline]
    #[must_use]
    pub fn potato_res(&self) -> (usize, usize) {
        self.resolution(match self {
            Self::Square => 512,
            Self::Classic => 128,
            Self::Standard => 30,
            Self::Widescreen => 10,
            Self::IPhoneXS => 1,
        })
    }
}

impl Display for AspectRatio {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let (rx, ry) = self.ratio();
        write!(
            fmt,
            "{} [{}:{}]",
            match self {
                Self::Square => "Square",
                Self::Classic => "Classic",
                Self::Standard => "Standard",
                Self::Widescreen => "Widescreen",
                Self::IPhoneXS => "IPhone XS",
            },
            rx,
            ry
        )
    }
}
