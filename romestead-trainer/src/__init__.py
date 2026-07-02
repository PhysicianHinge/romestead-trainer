"""Romestead Trainer - A simple neural net trainer for regression tasks."""

from .trainer import Trainer
from .model import SimpleModel
from .data import generate_synthetic_data

__all__ = ["Trainer", "SimpleModel", "generate_synthetic_data"]
__version__ = "0.1.0"