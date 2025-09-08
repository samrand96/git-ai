# CLI command to list models for the current provider
from core.config import settings
from providers.factory import get_provider
from utils import Colors

def main():
	provider_name = settings.get('PROVIDER', 'openai')
	print(Colors.header(f"🤖 Fetching available models for {provider_name}..."))
	
	try:
		provider = get_provider(provider_name)
		models = provider.list_models()
		
		print(Colors.success(f"📋 Available models for {provider_name}:"))
		for i, model in enumerate(models, 1):
			print(Colors.highlight(f"  {i}. {model}"))
		
		if not models:
			print(Colors.warning("⚠️ No models found or model list is empty"))
		else:
			print(Colors.dim(f"\n💡 Total: {len(models)} model(s) available"))
			print(Colors.dim("💡 Use these model names in your configuration"))
			
	except Exception as e:
		print(Colors.error(f"❌ Error fetching models: {e}"))
		print(Colors.dim("💡 Check your provider configuration and network connection"))

if __name__ == "__main__":
	main()
