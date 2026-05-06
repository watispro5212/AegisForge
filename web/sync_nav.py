import os
import re

# Define the new navigation links HTML
NEW_NAV = """<div class="nav-links">
            <a href="index.html#features">Features</a>
            <a href="commands.html">Commands</a>
            <a href="team.html">Team</a>
            <a href="changelog.html">Changelog</a>
            <a href="premium.html" class="nav-link-premium">Premium</a>
            <a href="status.html">Status</a>
            <a href="about.html">About</a>
            <a href="https://github.com/watispro5212/AegisForge" target="_blank" rel="noopener noreferrer" class="nav-link-code">GitHub</a>
            <a href="https://discord.com/oauth2/authorize?client_id=1500582485367722004&permissions=8&response_type=code&redirect_uri=https%3A%2F%2Fdiscord.com%2Foauth2%2Fauthorize%3Fclient_id%3D1500582485367722004&integration_type=0&scope=bot+identify+applications.commands+applications.commands.permissions.update+openid" target="_blank" rel="noopener noreferrer" class="btn-primary">Invite Bot</a>
        </div>"""

NEW_FOOTER_LINKS = """<div class="footer-links">
                <a href="index.html#features">Features</a>
                <a href="commands.html">Commands</a>
                <a href="team.html">Team</a>
                <a href="changelog.html">Changelog</a>
                <a href="premium.html">Premium</a>
                <a href="status.html">Status</a>
                <a href="about.html">About</a>
                <a href="https://github.com/watispro5212/AegisForge" target="_blank" rel="noopener noreferrer">GitHub</a>
                <a href="privacy.html">Privacy Policy</a>
                <a href="terms.html">Terms of Service</a>
            </div>"""

html_files = [f for f in os.listdir('.') if f.endswith('.html')]

for file_path in html_files:
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Update Navigation
    content = re.sub(r'<div class="nav-links">.*?</div>', NEW_NAV, content, flags=re.DOTALL)
    
    # Update Footer Links
    content = re.sub(r'<div class="footer-links">.*?</div>', NEW_FOOTER_LINKS, content, flags=re.DOTALL)
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)

print(f"Updated {len(html_files)} files.")
