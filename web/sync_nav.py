import os
import re

# Define the new navigation links HTML (lazy style)
NEW_NAV = """<div class="nav-links">
            <a href="index.html#features">features</a>
            <a href="commands.html">commands</a>
            <a href="economy.html">economy</a>
            <a href="team.html">team</a>
            <a href="changelog.html">changelog</a>
            <a href="shards.html">status</a>
            <a href="about.html">about</a>
            <a href="https://github.com/watispro5212/aegisforge" target="_blank" rel="noopener noreferrer" class="nav-link-code">github</a>
            <a href="https://discord.com/oauth2/authorize?client_id=1500582485367722004&permissions=8&scope=bot+applications.commands" target="_blank" rel="noopener noreferrer" class="btn-primary">invite bot</a>
        </div>"""

NEW_FOOTER_LINKS = """<div class="footer-links">
                <a href="index.html#features">features</a>
                <a href="commands.html">commands</a>
                <a href="team.html">team</a>
                <a href="changelog.html">changelog</a>
                <a href="shards.html">status</a>
                <a href="about.html">about</a>
                <a href="https://github.com/watispro5212/aegisforge" target="_blank" rel="noopener noreferrer">github</a>
                <a href="privacy.html">privacy policy</a>
                <a href="terms.html">terms of service</a>
            </div>"""

MOBILE_NAV = """<div class="mobile-nav" id="mobileNav">
        <a href="index.html#features">features</a>
        <a href="commands.html">commands</a>
        <a href="economy.html">economy</a>
        <a href="team.html">team</a>
        <a href="shards.html">status</a>
        <a href="about.html">about</a>
        <a href="https://github.com/watispro5212/aegisforge" target="_blank" rel="noopener noreferrer">github</a>
        <a href="https://discord.com/oauth2/authorize?client_id=1500582485367722004&permissions=8&scope=bot+applications.commands" target="_blank" rel="noopener noreferrer" class="btn-primary mobile-nav-invite">invite bot</a>
    </div>"""

html_files = [os.path.join('web', f) for f in os.listdir('web') if f.endswith('.html')]

for file_path in html_files:
    print(f"Syncing {file_path}...")
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Update Navigation
    content = re.sub(r'<div class="nav-links">.*?</div>', NEW_NAV, content, flags=re.DOTALL)
    
    # Update Footer Links
    content = re.sub(r'<div class="footer-links">.*?</div>', NEW_FOOTER_LINKS, content, flags=re.DOTALL)

    # Update Mobile Nav
    content = re.sub(r'<div class="mobile-nav" id="mobileNav">.*?</div>', MOBILE_NAV, content, flags=re.DOTALL)
    
    # Final cleanup
    content = content.replace('AegisForge', 'aegisforge')
    content = content.replace('Forged with 🦀 Rust and precision', 'Fast moderation, economy, leveling, and utilities.')

    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)

print(f"Updated {len(html_files)} files.")
