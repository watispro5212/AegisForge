import os
import re

html_files = [f for f in os.listdir('web') if f.endswith('.html')]
pattern = re.compile(r'<div class="nav-logo-ring">\s*<span class="nav-logo-icon">.*?</span>\s*</div>', re.DOTALL)
replacement = '<img src="assets/logo.png" alt="AegisForge Logo" class="nav-logo-img">'

for file in html_files:
    filepath = os.path.join('web', file)
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    new_content = pattern.sub(replacement, content)
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(new_content)

print('Done replacing logos!')
