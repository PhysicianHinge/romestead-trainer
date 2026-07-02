"""Trainer class that runs gradient descent."""

import numpy as np
from .model import SimpleModel

class Trainer:
    def __init__(self, model: SimpleModel, lr: float = 0.01):
        self.model = model
        self.lr = lr
        self.loss_history = []

    def train(self, X: np.ndarray, y: np.ndarray, epochs: int = 100):
        for epoch in range(epochs):
            output = self.model.forward(X)
            loss = np.mean((output - y) ** 2)
            self.loss_history.append(loss)
            self.model.backward(X, y, output, self.lr)
            if epoch % 20 == 0:
                print(f"Epoch {epoch}, Loss: {loss:.6f}")

    def evaluate(self, X: np.ndarray, y: np.ndarray) -> float:
        preds = self.model.predict(X)
        return np.mean((preds - y) ** 2)