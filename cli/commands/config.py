# CLI config editor for global and provider-specific settings
from core.config import settings
from utils import Colors

def print_current():
	print(Colors.header("📋 Current global settings:"))
	for k, v in settings._data.items():
		print(Colors.info(f"  {k}: ") + Colors.highlight(str(v)))
	print(Colors.header("\n🔧 Provider settings:"))
	for provider, pdata in settings._provider_data.items():
		print(Colors.success(f"[{provider}]"))
		for k, v in pdata.items():
			print(Colors.dim(f"  {k}: ") + Colors.highlight(str(v)))

def interactive_edit():
	print_current()
	print(Colors.header("\n✏️ Edit global settings (press Enter to keep current value):"))
	for k in settings._data:
		current_val = settings._data[k]
		val = input(Colors.dim(f"{k} [{current_val}]: ")).strip()
		if val:
			settings.set(k, val)
			print(Colors.success(f"✅ Updated {k} = {val}"))
	
	print(Colors.header("\n🔧 Edit provider settings:"))
	provider = settings.get_provider()
	provider_items = settings._provider_data.get(provider, {})
	from providers.factory import get_provider
	
	for k in provider_items:
		if k == 'MODEL':
			print(Colors.info(f"📋 Available models for {provider}:"))
			try:
				prov = get_provider(provider)
				models = prov.list_models()
				for i, m in enumerate(models, 1):
					print(Colors.dim(f"  {i}. {m}"))
			except Exception as e:
				print(Colors.error(f"  [Error fetching models: {e}]"))
		
		current_val = provider_items[k]
		val = input(Colors.dim(f"{provider}.{k} [{current_val}]: ")).strip()
		if val:
			settings.set_provider_option(k, val, provider)
			print(Colors.success(f"✅ Updated {provider}.{k} = {val}"))
	
	print(Colors.success("🎉 Settings updated successfully!"))

def main():
	import argparse
	parser = argparse.ArgumentParser(description="Edit git-ai configuration.")
	parser.add_argument('--interactive', action='store_true', help='Edit config interactively')
	parser.add_argument('--set', nargs=2, metavar=('KEY', 'VALUE'), help='Set a global config value')
	parser.add_argument('--set-provider', nargs=3, metavar=('PROVIDER', 'KEY', 'VALUE'), help='Set a provider config value')
	args = parser.parse_args()

	if args.interactive or (not args.set and not args.set_provider):
		interactive_edit()
	if args.set:
		k, v = args.set
		settings.set(k, v)
		print(Colors.success(f"✅ Set {k} = {v}"))
	if args.set_provider:
		provider, k, v = args.set_provider
		settings.set_provider_option(k, v, provider)
		print(Colors.success(f"✅ Set [{provider}].{k} = {v}"))

if __name__ == "__main__":
	main()
