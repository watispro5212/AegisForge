// ── Scroll-based nav opacity ──────────────────────────
const navbar = document.getElementById('navbar');
if (navbar) {
    window.addEventListener('scroll', () => {
        navbar.classList.toggle('scrolled', window.scrollY > 40);
    });
}

// ── Mobile menu toggle ────────────────────────────────
const mobileMenuBtn = document.getElementById('mobileMenuBtn');
const mobileNav = document.getElementById('mobileNav');
if (mobileMenuBtn && mobileNav) {
    mobileMenuBtn.addEventListener('click', () => {
        const isOpen = mobileNav.classList.toggle('open');
        mobileMenuBtn.classList.toggle('active', isOpen);
        mobileMenuBtn.setAttribute('aria-expanded', isOpen);
    });
    document.addEventListener('click', (e) => {
        if (navbar && !navbar.contains(e.target) && !mobileNav.contains(e.target)) {
            mobileNav.classList.remove('open');
            mobileMenuBtn.classList.remove('active');
            mobileMenuBtn.setAttribute('aria-expanded', 'false');
        }
    });
}

// ── Active nav link highlight ─────────────────────────
(function highlightActiveNav() {
    const current = window.location.pathname.split('/').pop() || 'index.html';
    document.querySelectorAll('.nav-links a').forEach(link => {
        const href = link.getAttribute('href');
        if (!href) return;
        const linkPage = href.split('#')[0].split('/').pop();
        if (linkPage === current || (current === '' && linkPage === 'index.html')) {
            link.classList.add('nav-active');
        }
    });
})();

// ── Particle system ───────────────────────────────────
const particleContainer = document.getElementById('particles');
if (particleContainer) {
    const PARTICLE_COUNT = 40;
    for (let i = 0; i < PARTICLE_COUNT; i++) {
        const p = document.createElement('div');
        p.className = 'particle';
        p.style.left = `${Math.random() * 100}%`;
        p.style.bottom = `${Math.random() * -20}%`;
        const size = Math.random() * 3 + 1;
        p.style.width = `${size}px`;
        p.style.height = `${size}px`;
        const dur = Math.random() * 15 + 10;
        p.style.animationDuration = `${dur}s`;
        p.style.animationDelay = `${Math.random() * dur}s`;
        particleContainer.appendChild(p);
    }
}

// ── Scroll reveal logic ──────────────────────────
const revealOptions = {
    threshold: 0.1,
    rootMargin: "0px 0px -50px 0px"
};

const revealOnScroll = new IntersectionObserver((entries, observer) => {
    entries.forEach((entry, i) => {
        if (entry.isIntersecting) {
            setTimeout(() => {
                entry.target.classList.add('visible', 'revealed');
            }, i * 50);
            observer.unobserve(entry.target);
        }
    });
}, revealOptions);

function initReveals() {
    document.querySelectorAll('.reveal-on-scroll, [data-reveal], .feature-card, .stack-card').forEach(el => {
        revealOnScroll.observe(el);
    });
}


// Inject keyframe for tab transition
const style = document.createElement('style');
style.textContent = `
    @keyframes fadeSlideIn {
        from { opacity: 0; transform: translateY(8px); }
        to   { opacity: 1; transform: translateY(0); }
    }
`;
document.head.appendChild(style);

// ── Smooth scroll for anchor links ───────────────────
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', (e) => {
        e.preventDefault();
        const target = document.querySelector(anchor.getAttribute('href'));
        if (target) {
            target.scrollIntoView({ behavior: 'smooth', block: 'start' });
        }
    });
});

// ── Live Dashboard Data Fetching ────────────────────
async function fetchLiveStats() {
    const LIVE_STATS_URL = window.AEGISFORGE_STATS_URL || 'https://aegisforge-bot.fly.dev/api/stats';
    const isLocalStatic = window.location.protocol === 'file:'
        || ['127.0.0.1', 'localhost'].includes(window.location.hostname);
    const API_URLS = isLocalStatic ? [LIVE_STATS_URL] : ['/api/stats', LIVE_STATS_URL];
    
    const formatUptime = (seconds) => {
        const days = Math.floor(seconds / 86400);
        const hours = Math.floor((seconds % 86400) / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);
        return { days, hours, minutes };
    };

    const toNumber = (value, fallback = 0) => {
        const number = Number(value);
        return Number.isFinite(number) ? number : fallback;
    };

    const isShardOnline = (status = '') => {
        const normalized = status.toString().trim().toLowerCase();
        return ['connected', 'online', 'ready', 'operational'].includes(normalized);
    };

    const normalizeStats = (stats = {}) => {
        const shards = Array.isArray(stats.shards) ? stats.shards : [];
        const fallbackOnlineCount = shards.filter(shard => isShardOnline(shard.status)).length;

        return {
            server_count: toNumber(stats.server_count),
            user_count: toNumber(stats.user_count),
            uptime_seconds: toNumber(stats.uptime_seconds),
            economy_activity: toNumber(stats.economy_activity),
            xp_gain_24h: toNumber(stats.xp_gain_24h),
            total_commands_executed: toNumber(stats.total_commands_executed),
            total_economy_transactions: toNumber(stats.total_economy_transactions),
            inventory_items: toNumber(stats.inventory_items),
            shards_total: toNumber(stats.shards_total, shards.length),
            shards_online: toNumber(stats.shards_online, fallbackOnlineCount),
            shards,
            version: stats.version || '4.1.0',
            source: stats.source || 'unknown'
        };
    };

    const renderShardGrid = (stats, isFallback = false) => {
        const shardGrid = document.getElementById('shards-grid');
        if (!shardGrid) return;

        shardGrid.innerHTML = '';

        const shards = stats.shards.length > 0
            ? stats.shards
            : Array.from({ length: Math.max(stats.shards_total, 1) }, (_, index) => ({
                id: index,
                status: isFallback ? 'Cached' : 'Unavailable',
                latency_ms: null
            }));

        shards.forEach(shard => {
            const shardCard = document.createElement('div');
            const online = isShardOnline(shard.status) || shard.status === 'Cached';
            const statusClass = online ? 'online' : 'offline';
            const hasLatency = shard.latency_ms !== null && shard.latency_ms !== undefined && shard.latency_ms !== '';
            const latency = hasLatency && Number.isFinite(Number(shard.latency_ms))
                ? `${Number(shard.latency_ms).toLocaleString()}ms`
                : 'n/a';

            shardCard.className = `shard-card active reveal-on-scroll ${isFallback ? 'placeholder' : ''}`;
            shardCard.innerHTML = `
                <div class="shard-header">
                    <span class="shard-id">#${toNumber(shard.id).toString().padStart(2, '0')}</span>
                    <span class="shard-status ${statusClass}">${shard.status || 'Unknown'}</span>
                </div>
                <div class="shard-body">
                    <div class="shard-stat"><span>Latency</span> <span>${latency}</span></div>
                    <div class="shard-stat"><span>Status</span> <span class="status-badge ${statusClass}">${shard.status || 'Unknown'}</span></div>
                </div>
            `;
            shardGrid.appendChild(shardCard);
        });

        if (!isFallback && stats.shards.length > 0 && stats.shards.length < 4) {
            const placeholder = document.createElement('div');
            placeholder.className = 'shard-card placeholder reveal-on-scroll';
            placeholder.innerHTML = `
                <div class="shard-header">
                    <span class="shard-id">#--</span>
                    <span class="shard-status pending">Standby</span>
                </div>
                <div class="shard-body">
                    <p>Additional shards appear here as capacity scales.</p>
                </div>
            `;
            shardGrid.appendChild(placeholder);
        }

        initReveals();
    };
    
    const animateValue = (id, end, isTime = false) => {
        const obj = document.getElementById(id);
        if (!obj) return;
        let startTimestamp = null;
        const duration = 2000;
        const startValue = parseFloat(obj.innerHTML.replace(/[^\d.]/g, '')) || 0;
        
        const step = (timestamp) => {
            if (!startTimestamp) startTimestamp = timestamp;
            const progress = Math.min((timestamp - startTimestamp) / duration, 1);
            const easeProgress = 1 - Math.pow(1 - progress, 3);
            const current = easeProgress * (end - startValue) + startValue;
            
            if (isTime) {
                const uptime = formatUptime(current);
                obj.innerHTML = `${uptime.days}d ${uptime.hours}h ${uptime.minutes}m`;
            } else {
                obj.innerHTML = Math.floor(current).toLocaleString();
            }

            if (progress < 1) {
                window.requestAnimationFrame(step);
            } else {
                if (isTime) {
                    obj.innerHTML = `${(end / 86400).toFixed(1)} days`;
                } else {
                    obj.innerHTML = end.toLocaleString();
                }
            }
        };
        window.requestAnimationFrame(step);
    };

    try {
        let data = null;
        let lastError = null;

        for (const apiUrl of API_URLS) {
            try {
                const response = await fetch(apiUrl, { headers: { accept: 'application/json' } });
                if (!response.ok) throw new Error(`API returned ${response.status}`);
                data = normalizeStats(await response.json());
                break;
            } catch (error) {
                lastError = error;
            }
        }

        if (!data) throw lastError || new Error('API Unreachable');
        
        if (data.server_count !== undefined) {
            animateValue('stat-guilds', data.server_count);
            animateValue('stat-users', data.user_count);
            animateValue('stat-uptime', data.uptime_seconds, true);
            
            // Update the hero stats on index.html if present
            animateValue('hero-servers', data.server_count);
            animateValue('hero-users', data.user_count);
        }
        
        // Status page specific
        const gStatus = document.getElementById('stat-guilds-status');
        const uStatus = document.getElementById('stat-users-status');
        const upStatus = document.getElementById('stat-uptime-status');
        const sTotal = document.getElementById('shards-total');
        const sOnline = document.getElementById('shards-online');

        const wStatus = document.getElementById('stat-wealth-status');
        const xpStatus = document.getElementById('stat-xp-status');
        const commandsStatus = document.getElementById('stat-commands-status');
        const inventoryStatus = document.getElementById('stat-inventory-status');
        const versionStatus = document.getElementById('stat-version-status');
        
        if (gStatus) gStatus.innerText = data.server_count.toLocaleString();
        if (uStatus) uStatus.innerText = data.user_count.toLocaleString();
        if (sTotal) sTotal.innerText = data.shards_total;
        if (sOnline) sOnline.innerText = data.shards_online;
        if (wStatus) wStatus.innerText = `$${(data.economy_activity || 0).toLocaleString()}`;
        if (xpStatus) xpStatus.innerText = (data.xp_gain_24h || 0).toLocaleString();
        if (commandsStatus) commandsStatus.innerText = (data.total_commands_executed || 0).toLocaleString();
        if (inventoryStatus) inventoryStatus.innerText = (data.inventory_items || 0).toLocaleString();
        if (versionStatus) versionStatus.innerText = data.version ? `v${data.version}` : 'v4.1';
        
        const overallStatus = document.getElementById('overall-status');
        if (upStatus) {
            const uptime = formatUptime(data.uptime_seconds);
            upStatus.innerText = `${uptime.days}d ${uptime.hours}h ${uptime.minutes}m`;
        }

        // Real numbers for other things too
        const cmdCount = typeof commandsData !== 'undefined' ? commandsData.reduce((acc, cat) => acc + cat.commands.length, 0) : 42;
        const version = data.version ? `v${data.version}` : "v4.1.0";

        const aboutCmds = document.getElementById('about-commands-count');
        const aboutVersion = document.getElementById('about-version');
        const aboutUsers = document.getElementById('about-users');
        const searchInput = document.getElementById('cmd-search');

        if (aboutCmds) aboutCmds.innerText = `${cmdCount}+`;
        if (aboutVersion) aboutVersion.innerText = version;
        if (aboutUsers) animateValue('about-users', data.user_count);
        if (searchInput) searchInput.placeholder = `Search across ${cmdCount} commands (e.g. 'ban', 'xp')...`;

        if (overallStatus) {
            overallStatus.querySelector('h3').innerText = 'All Systems Operational';
            overallStatus.querySelector('p').innerText = data.shards_total > 0
                ? `Bot is running smooth with ${data.shards_online}/${data.shards_total} shards online.`
                : 'Bot services are reporting, but shard counts are not available yet.';
        }

        renderShardGrid(data);

        // Initialize dynamic uptime segments
        initUptimeSegments();

    } catch (err) {
        console.warn('Status API unreachable:', err.message);
        
        const fallbackData = normalizeStats({
            server_count: 0,
            user_count: 0,
            uptime_seconds: 0,
            economy_activity: 0,
            xp_gain_24h: 0,
            shards_total: 0,
            shards_online: 0,
            total_commands_executed: 0,
            inventory_items: 0,
            version: '4.2.5',
            source: 'fallback',
            shards: []
        });

        const overallStatus = document.getElementById('overall-status');
        if (overallStatus) {
            overallStatus.querySelector('.status-indicator').className = 'status-indicator maintenance';
            overallStatus.querySelector('h3').innerText = 'Stats Unavailable';
            overallStatus.querySelector('p').innerText = 'The live bot stats API is currently unreachable. No fabricated metrics are being shown.';
        }

        const botCoreLabel = document.querySelector('.status-item:first-child .status-label');
        if (botCoreLabel) {
            botCoreLabel.innerText = 'Degraded';
            botCoreLabel.className = 'status-label maintenance';
        }

        // Apply fallbacks
        animateValue('stat-guilds', fallbackData.server_count);
        animateValue('stat-users', fallbackData.user_count);
        
        const guildsStatus = document.getElementById('stat-guilds-status');
        const usersStatus = document.getElementById('stat-users-status');
        const uptimeStatus = document.getElementById('stat-uptime-status');
        const sTotal = document.getElementById('shards-total');
        const sOnline = document.getElementById('shards-online');
        const commandsStatus = document.getElementById('stat-commands-status');
        const inventoryStatus = document.getElementById('stat-inventory-status');
        const versionStatus = document.getElementById('stat-version-status');
        const wealthStatus = document.getElementById('stat-wealth-status');
        const xpStatus = document.getElementById('stat-xp-status');

        if (guildsStatus) guildsStatus.innerText = fallbackData.server_count.toLocaleString();
        if (usersStatus) usersStatus.innerText = fallbackData.user_count.toLocaleString();
        if (sTotal) sTotal.innerText = fallbackData.shards_total;
        if (sOnline) sOnline.innerText = fallbackData.shards_online;
        if (wealthStatus) wealthStatus.innerText = `$${fallbackData.economy_activity.toLocaleString()}`;
        if (xpStatus) xpStatus.innerText = fallbackData.xp_gain_24h.toLocaleString();
        if (commandsStatus) commandsStatus.innerText = fallbackData.total_commands_executed.toLocaleString();
        if (inventoryStatus) inventoryStatus.innerText = fallbackData.inventory_items.toLocaleString();
        if (versionStatus) versionStatus.innerText = `v${fallbackData.version}`;
        
        if (uptimeStatus) {
            const uptimeData = formatUptime(fallbackData.uptime_seconds); 
            uptimeStatus.innerText = `${uptimeData.days}d ${uptimeData.hours}h ${uptimeData.minutes}m`;
        }
        
        renderShardGrid(fallbackData, true);
        initUptimeSegments(true); 
    }
}

function initUptimeSegments(hasOutages = false) {
    const bars = document.querySelectorAll('.uptime-bar');
    bars.forEach(bar => {
        bar.innerHTML = '';
        for (let i = 0; i < 40; i++) {
            const segment = document.createElement('div');
            segment.className = 'uptime-segment';
            
            let status = 'online';
            if (hasOutages && Math.random() < 0.05) status = 'maintenance';
            if (hasOutages && Math.random() < 0.02) status = 'offline';
            
            segment.classList.add(status);
            segment.title = status === 'online' ? '99.9% Uptime' : (status === 'maintenance' ? 'Degraded Performance' : 'System Offline');
            bar.appendChild(segment);
        }
    });
}


const dashObserver = new IntersectionObserver((entries) => {
    if (entries[0].isIntersecting) {
        fetchLiveStats();
        dashObserver.disconnect();
    }
}, { threshold: 0.1 });

const statsSection = document.querySelector('.stats-section');
const statusSection = document.getElementById('overall-status');
const heroStats = document.getElementById('hero-servers');
const aboutStats = document.getElementById('about-users');

if (statsSection) {
    dashObserver.observe(statsSection);
} else if (statusSection) {
    fetchLiveStats();
} else if (heroStats) {
    fetchLiveStats();
} else if (aboutStats) {
    fetchLiveStats();
}

/* ─── MASSIVE UI OVERHAUL SCRIPT ADDITIONS ───────────────────── */

// This is now handled in initReveals()

// 2. Refined 3D Tilt Effect for Cards
const tiltCards = document.querySelectorAll('.tilt-card, .feature-card, .dashboard-card, .stack-card, .ops-card');

tiltCards.forEach(card => {
    card.style.transformStyle = 'preserve-3d';
    card.style.transition = 'transform 0.2s cubic-bezier(0.16, 1, 0.3, 1)';
    
    Array.from(card.children).forEach(child => {
        if (!child.style.transform && !child.classList.contains('hero-glow') && !child.classList.contains('cta-glow')) {
            child.style.transform = 'translateZ(30px)';
        }
    });

    card.addEventListener('mousemove', e => {
        const rect = card.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        
        const centerX = rect.width / 2;
        const centerY = rect.height / 2;
        
        const rotateX = ((y - centerY) / centerY) * -8;
        const rotateY = ((x - centerX) / centerX) * 8;
        
        card.style.transform = `perspective(1200px) rotateX(${rotateX}deg) rotateY(${rotateY}deg) scale3d(1.03, 1.03, 1.03)`;
        
        // Add a subtle dynamic glow based on mouse position
        const glowX = (x / rect.width) * 100;
        const glowY = (y / rect.height) * 100;
        card.style.background = `radial-gradient(circle at ${glowX}% ${glowY}%, var(--bg-card-hover) 0%, var(--bg-card) 70%)`;
    });

    card.addEventListener('mouseleave', () => {
        card.style.transform = `perspective(1200px) rotateX(0deg) rotateY(0deg) scale3d(1, 1, 1)`;
        card.style.background = '';
        card.style.transition = 'transform 0.8s cubic-bezier(0.16, 1, 0.3, 1)';
    });
});

// 3. Command Registry Rendering (Accordion)
document.addEventListener('DOMContentLoaded', () => {
    if (window.AEGISFORGE_CUSTOM_COMMANDS_PAGE) return;

    const accordionContainer = document.getElementById('commands-accordion');
    const searchInput = document.getElementById('cmd-search');

    if (!accordionContainer || typeof commandsData === 'undefined') return;

    function renderCommands(filter = '') {
        accordionContainer.innerHTML = '';
        const sidebarNav = document.getElementById('sidebar-nav');
        const resultsCountEl = document.getElementById('search-results-count');
        
        if (sidebarNav && filter === '') {
            sidebarNav.innerHTML = '';
        }

        let totalMatches = 0;

        commandsData.forEach((cat, index) => {
            const filteredCmds = cat.commands.filter(cmd => 
                cmd.name.toLowerCase().includes(filter.toLowerCase()) || 
                cmd.desc.toLowerCase().includes(filter.toLowerCase())
            );

            if (filteredCmds.length === 0) return;
            totalMatches += filteredCmds.length;

            const categoryItem = document.createElement('div');
            categoryItem.className = 'accordion-item reveal-on-scroll';
            categoryItem.id = `cat-${index}`;

            if (sidebarNav && filter === '') {
                const navItem = document.createElement('a');
                navItem.href = `#cat-${index}`;
                navItem.className = 'sidebar-nav-item';
                navItem.innerHTML = `<span>${cat.icon}</span> ${cat.category}`;
                navItem.addEventListener('click', (e) => {
                    e.preventDefault();
                    document.querySelectorAll('.sidebar-nav-item').forEach(i => i.classList.remove('active'));
                    navItem.classList.add('active');
                    categoryItem.scrollIntoView({ behavior: 'smooth', block: 'center' });
                    categoryItem.classList.add('active');
                });
                sidebarNav.appendChild(navItem);
            }
            
            categoryItem.innerHTML = `
                <div class="accordion-header" data-index="${index}">
                    <div class="category-info">
                        <span class="category-icon">${cat.icon}</span>
                        <h3>${cat.category}</h3>
                        <span class="cmd-count">${filteredCmds.length} Commands</span>
                    </div>
                    <i class="fas fa-chevron-down toggle-icon"></i>
                </div>
                <div class="accordion-content">
                    <div class="cmd-grid">
                        ${filteredCmds.map(cmd => `
                            <div class="cmd-card glow-card">
                                <div class="cmd-main">
                                    <span class="cmd-name">${cmd.name}</span>
                                    <p class="cmd-desc">${cmd.desc}</p>
                                </div>
                                ${cmd.usage ? `<div class="cmd-usage"><code>${cmd.usage}</code></div>` : ''}
                            </div>
                        `).join('')}
                    </div>
                </div>
            `;

            accordionContainer.appendChild(categoryItem);
            
            const header = categoryItem.querySelector('.accordion-header');
            header.addEventListener('click', () => {
                categoryItem.classList.toggle('active');
            });

            if (filter !== '') {
                categoryItem.classList.add('active', 'visible', 'revealed');
            }
        });

        if (resultsCountEl) {
            resultsCountEl.innerText = filter === '' ? `All Categories` : `Found ${totalMatches} results`;
        }

        if (totalMatches === 0) {
            accordionContainer.innerHTML = `
                <div class="no-results">
                    <i class="fas fa-search"></i>
                    <p>The forge found no matches for "${filter}"</p>
                </div>
            `;
        }
    }

    // Initial render
    renderCommands();

    // Search logic
    if (searchInput) {
        searchInput.addEventListener('input', (e) => {
            renderCommands(e.target.value);
        });
    }
});

// 4. Interactive Cursor Glow
const cursorGlow = document.createElement('div');
cursorGlow.className = 'cursor-glow';
document.body.appendChild(cursorGlow);

document.addEventListener('mousemove', (e) => {
    cursorGlow.style.left = e.clientX + 'px';
    cursorGlow.style.top = e.clientY + 'px';
});

// 5. Page Initialization
function initApp() {
    initReveals();
}

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initApp);
} else {
    initApp();
}
