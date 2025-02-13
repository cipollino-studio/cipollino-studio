
import io
import json
import requests
import zipfile

ICON_URL = 'https://phosphoricons.com/assets/phosphor-icons.zip'

print(f"[*] Downloading icons ({ICON_URL})")
response = requests.get(ICON_URL)
print(f"[*] Downloaded {len(response.content)} bytes")
zip = zipfile.ZipFile(io.BytesIO(response.content))

font = zip.read('Fonts/regular/Phosphor.ttf')

with open('res/icons/Phosphor.ttf', 'wb') as file:
    file.write(font)
print('[*] Written font file')

icons = json.loads(zip.read('Fonts/regular/selection.json'))

with open('src/icons/icons.gen.rs', 'w') as file:
    for icon in icons['icons']:
        names = icon['properties']['name'].split(', ')
        code = icon['properties']['code']
        code_hex = hex(code)[2:].upper()

        for name in names:
            rust_name = name.upper().replace('-', '_')
            file.write(f'pub const {rust_name}: &str = "\\u{{{code_hex}}}\";\n')

print('[*] Written icon string constants')