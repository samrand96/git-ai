# Abstract base class for LLM providers
from abc import ABC, abstractmethod

class ProviderBase(ABC):
	"""Base interface for all LLM providers."""

	@abstractmethod
	def generate(self, prompt: str, **kwargs):
		"""Generate a response from the provider."""
		pass

	@abstractmethod
	def list_models(self):
		"""List available models for this provider."""
		pass
