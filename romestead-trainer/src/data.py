"""Synthetic data generation for testing."""

import numpy as np

def generate_synthetic_data(n_samples: int = 100, noise: float = 0.1) -> tuple:
    """Generate y = 2 * x + 1 + Gaussian noise."""
    np.random.seed(42)
    X = np.random.randn(n_samples, 1)
    y = 2 * X + 1 + noise * np.random.randn(n_samples, 1)
    return X, y