//! # ODE Solver
//!
//! This crate provides numerical methods for solving ODEs
//!
//! ## Features
//! - Euler's Method
//!
//! ## Example
//! ```rust
//! use ode2joy::{euler};
//!
//! let f = |t: f64, y: f64| y;  // dy/dt = y
//! let euler_result = euler(f, 1.0, 0.0, 1.0, 0.1);
//! println!("Euler: {:?}", euler_result);
//! ```

pub mod methods;

pub use methods::euler::euler;