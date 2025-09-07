# Provider factory for LLM providers
from .base import ProviderBase
from .openai_provider import OpenAIProvider
from .anthropic_provider import AnthropicProvider
from .ollama_provider import OllamaProvider

_PROVIDER_MAP = {
	'openai': OpenAIProvider,
	'anthropic': AnthropicProvider,
	'ollama': OllamaProvider,
}

def get_provider(name: str, **kwargs) -> ProviderBase:
	"""Return an instance of the provider by name."""
	cls = _PROVIDER_MAP.get(name.lower())
	if not cls:
		raise ValueError(f"Unknown provider: {name}")
	return cls(**kwargs)
