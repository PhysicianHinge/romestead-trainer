"""Tests for the Romestead Trainer."""

import numpy as np
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from src.model import SimpleModel
from src.trainer import Trainer
from src.data import generate_synthetic_data

def test_model_forward_shape():
    model = SimpleModel(input_size=1, hidden_size=5, output_size=1)
    X = np.random.randn(10, 1)
    out = model.forward(X)
    assert out.shape == (10, 1), f"Expected (10,1), got {out.shape}"

def test_training_reduces_loss():
    X, y = generate_synthetic_data(n_samples=50, noise=0.1)
    model = SimpleModel(input_size=1, hidden_size=10, output_size=1)
    trainer = Trainer(model, lr=0.05)
    trainer.train(X, y, epochs=50)
    final_loss = trainer.evaluate(X, y)
    assert final_loss < 1.0, f"Loss too high: {final_loss}"

def test_predict_returns_same_shape():
    model = SimpleModel(input_size=1, hidden_size=5, output_size=1)
    X = np.random.randn(5, 1)
    preds = model.predict(X)
    assert preds.shape == (5, 1)